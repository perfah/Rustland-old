use std::os::raw::c_void;
use std::mem::{size_of, transmute};

use sugars::{Renderable, GraphicsProgram};

use wlc::Geometry;
use gl;
use gl::types::{GLint, GLuint, GLfloat, GLchar, GLenum};

static FULLSCREEN_VERTICES: [GLfloat; 12] = [
    //  Coordinates (x, y)  Texcoords (u, v)  
        -1f32,   -1f32, 
        -1f32,   1f32,  
        1f32,   -1f32,   
        
        1f32,   -1f32,     
        -1f32,   1f32,     
        1f32,    1f32,  
];

#[derive(Serialize, Deserialize, Clone)]
pub struct Frame {
    vao: GLuint,
    vbo: GLuint,
    pub opacity: f32
}

impl Frame {
    pub fn new(program_id: GLuint, initial_opacity: f32) -> Frame {
        let mut vao: GLuint = 0;
        let mut vbo: GLuint = 2;
        let mut texture_id: GLuint = 0;
    
        unsafe{   
            // VAO
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            // VBO
            gl::GenBuffers(1, &mut vbo); 
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (FULLSCREEN_VERTICES.len() * size_of::<GLfloat>()) as gl::types::GLsizeiptr,
                transmute(&FULLSCREEN_VERTICES[0]),        
                gl::STATIC_DRAW
            );
            
            let pos = "pos";

            // Vertex attribute:
            match gl::GetAttribLocation(program_id, pos.as_ptr() as *const GLchar) {
                n if n < 0 => {
                    print!("GL: Attribute '{}' does not exists in shader. ", pos);
                    print!("Attempting binding of this attribute: ");

                    gl::BindAttribLocation(program_id, 0 as GLuint, pos.as_ptr() as *const GLchar);
                    match gl::GetError() {
                        gl::NO_ERROR => println!("[SUCCESS]"),
                        gl::INVALID_ENUM => println!("[ERROR: INVALID ENUM]"),
                        gl::INVALID_VALUE => println!("[ERROR: INVALID VALUE]"),
                        gl::INVALID_OPERATION => println!("[ERROR: INVALID OPERATION]"), 
                        gl::INVALID_FRAMEBUFFER_OPERATION => println!("[ERROR: INVALID FRAME BUFFER]"),
                        gl::OUT_OF_MEMORY => println!("[ERROR: OUT OF MEMORY]"),
                        _ => println!("[ERROR: UNDEFINED ERROR]")
                    }
                },
                _ => {}
            }

            gl::EnableVertexAttribArray(0u32);
            gl::VertexAttribPointer(
                0u32, 
                2, 
                gl::FLOAT, 
                gl::FALSE, 
                2i32 * size_of::<GLfloat>() as i32, 
                0 as *const c_void
            );

            match gl::GetError() {
                val if val != gl::NO_ERROR => {
                    print!("Something went wrong when initializing the a frame: ");
                    match val{
                        gl::NO_ERROR => {},
                        gl::INVALID_ENUM => println!("[ERROR: INVALID ENUM]"),
                        gl::INVALID_VALUE => println!("[ERROR: INVALID VALUE]"),
                        gl::INVALID_OPERATION => println!("[ERROR: INVALID OPERATION]"), 
                        gl::INVALID_FRAMEBUFFER_OPERATION => println!("[ERROR: INVALID FRAME BUFFER]"),
                        gl::OUT_OF_MEMORY => println!("[ERROR: OUT OF MEMORY]"),
                        _ => println!("[ERROR: UNDEFINED ERROR]")
                    }
                },
                _ => {}
            }
        }

        Frame{vao, vbo, opacity: initial_opacity }
    }
}

impl Renderable for Frame {
    fn draw(&mut self, program: &GraphicsProgram, viewport: Geometry) {
        unsafe {  
            gl::Viewport(viewport.origin.x, viewport.origin.y, viewport.size.w as i32, viewport.size.h as i32);
            gl::ClearColor(1.0, 1.0, 1.0, self.opacity);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            gl::DrawArrays(gl::TRIANGLES, 0, 12);
            gl::BindVertexArray(0);
            gl::Flush();

            match gl::GetError() {
                gl::NO_ERROR => {},
                gl::INVALID_ENUM => println!("GL: INVALID ENUM"),
                gl::INVALID_VALUE => println!("GL: INVALID VALUE"),
                gl::INVALID_OPERATION => println!("GL: INVALID OPERATION"), 
                gl::INVALID_FRAMEBUFFER_OPERATION => println!("GL: INVALID FRAME BUFFER"),
                gl::OUT_OF_MEMORY => println!("GL: OUT OF MEMORY"),
                _ => println!("GL: UNDEFINED ERROR")
            }
        }
    }
}


