use std::ptr::null;
use std::ffi::CString;
use std::str;
use std::ops::DerefMut;

use common::definitions::LayoutElemID;
use sugars::Renderable;

use wlc::Geometry;
use gl;
use gl::types::{GLint, GLuint};

// Vertex shader
pub const VERTEX_SHADER: &str = "
    #version 100
    // Constants:
    uniform vec4 color;

    // Inputs:
    attribute vec2 pos;
    attribute vec2 texcoords;

    // Outputs:
    varying vec4 v_color;
    varying vec2 v_texcoords;

    void main() {
        v_color = color;
        v_texcoords = texcoords;
        
        gl_Position = vec4(pos, 0.0, 1.0);
    }
";

// Fragment shader
pub const FRAGMENT_SHADER: &str = "
    #version 100
    precision mediump float;

    uniform sampler2D tex;

    varying vec4 v_color;
    varying vec2 v_texcoords;

    void main() {
        gl_FragColor = texture2D(tex, v_texcoords);
    }
";

pub struct GraphicsProgram {
    pub id: GLuint
}

impl GraphicsProgram {
    pub fn init() -> GraphicsProgram {
        unsafe{
            let mut shader_compiled = 0;

            let mut validate_shader = |name: &str, unit: GLuint| {
                gl::GetShaderiv(unit, gl::COMPILE_STATUS, &mut shader_compiled);
                if shader_compiled == gl::FALSE as GLint {
                    let mut len = 0;
                    gl::GetShaderiv(unit, gl::INFO_LOG_LENGTH, &mut len);

                    let mut info_log = Vec::new();
                    info_log.resize(len as usize, b'\0');
                    gl::GetShaderInfoLog(unit, len, &mut len, info_log.as_mut_ptr() as *mut i8);

                    println!("{} shader compilation error: {}", name, String::from_utf8(info_log).unwrap());
                }
            };

            let vert = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vert, 1, &CString::from_vec_unchecked(String::from(VERTEX_SHADER).into_bytes()).as_ptr(), null());
            gl::CompileShader(vert);
            validate_shader("Vertex", vert);

            let frag: GLuint = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(frag, 1, &CString::from_vec_unchecked(String::from(FRAGMENT_SHADER).into_bytes()).as_ptr(), null());
            gl::CompileShader(frag);
            validate_shader("Fragment", frag);

            let program = gl::CreateProgram();
            gl::AttachShader(program, vert);
            gl::AttachShader(program, frag);
            gl::LinkProgram(program);

            let mut status = 0;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);
    
            if status == gl::FALSE as GLint {
                let mut len = 0;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);

                let mut info_log = Vec::new();
                info_log.resize(len as usize, b'\0');
                gl::GetProgramInfoLog(program, len, &mut len, info_log.as_mut_ptr() as *mut i8);

                println!("Linking error: {}", String::from_utf8(info_log).unwrap());
            }

            GraphicsProgram { 
                id: program
            }
        }
    }

    pub fn run_job<T: Renderable>(&self, job: &mut T, viewport: Geometry) {
        self.safe_graphics_operation(|_| {
            job.draw(&self, viewport);
        });
    }

    pub fn safe_graphics_operation<F: FnMut(GLuint)>(&self, mut operation: F) {
        let mut program: gl::types::GLint = -1;
        let mut viewport: [gl::types::GLint; 4] = [-1, -1, -1, -1]; 
        let mut vao = -1;
        let mut vbo = -1;
        let mut framebuffer = 0;
        
        // Save previous state
        unsafe { 
            gl::GetIntegerv(gl::CURRENT_PROGRAM, &mut program as *mut _);
            gl::GetIntegerv(gl::VIEWPORT, viewport.as_mut_ptr());
            gl::GetIntegerv(gl::VERTEX_ARRAY_BINDING, &mut vao as *mut _ );
            gl::GetIntegerv(gl::ARRAY_BUFFER_BINDING, &mut vbo as *mut _ );
            gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut framebuffer as *mut _)
        }

        // Go into temporary state
        unsafe { 
            gl::UseProgram(self.id); 
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }

        // Execute operation
        operation(self.id);

        // Return to previous state
        unsafe {
            gl::UseProgram(program as GLuint);
            gl::Viewport(viewport[0], viewport[1], viewport[2], viewport[3]);
            gl::BindVertexArray(vao as GLuint);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo as GLuint);
            gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer as GLuint);
        }
    }
}

