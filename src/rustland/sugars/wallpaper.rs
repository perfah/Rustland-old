use std::os::raw::c_void;
use std::mem::{size_of, transmute};

use sugars::{Renderable, GraphicsProgram};

use wlc::Geometry;
use image::RgbaImage;
use gl;
use gl::types::{GLint, GLuint, GLfloat, GLchar, GLenum};

static FULLSCREEN_VERTICES: [GLfloat; 24] = [
    //  Coordinates (x, y)  Texcoords (u, v)  
        -1f32,   -1f32,     0f32,      1f32, // TOP LEFT
        -1f32,   1f32,      0f32,      0f32, // BOTTOM LEFT
        1f32,   -1f32,      1f32,      1f32, // TOP RIGHT
        
        1f32,   -1f32,      1f32,      1f32, // TOP RIGHT
        -1f32,   1f32,      0f32,      0f32, // BOTTOM LEFT
        1f32,    1f32,      1f32,      0f32, // BOTTOM RIGHT    
];

pub struct Wallpaper {
    vao: GLuint,
    vbo: GLuint,
    texture_id: GLuint
}

impl Wallpaper {
    pub fn new(image: &RgbaImage, program_id: GLuint) -> Wallpaper {
        let texture = image.clone().into_vec();
        
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
            
            gl::ActiveTexture(gl::TEXTURE0);
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl::TexImage2D(
                gl::TEXTURE_2D, 
                0, 
                gl::RGBA as i32, 
                image.width() as i32,  
                image.height() as i32, 
                0, 
                gl::RGBA, 
                gl::UNSIGNED_BYTE, 
                texture.as_ptr() as *const c_void
            );
            
            gl::Uniform1i(gl::GetUniformLocation(program_id, "tex".as_ptr() as *const GLchar), 0);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            // Vertex attributes:
            let attributes = ["pos", "texcoords"];            
            
            for a in 0..attributes.len() {
                match gl::GetAttribLocation(program_id, attributes[a].as_ptr() as *const GLchar) {
                    n if n < 0 => {
                        print!("GL: Attribute '{}' does not exists in shader. ", attributes[a]);
                        print!("Attempting binding of this attribute: ");

                        gl::BindAttribLocation(program_id, a as GLuint, attributes[a].as_ptr() as *const GLchar);
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
                
                gl::EnableVertexAttribArray(a as u32);
                gl::VertexAttribPointer(
                    a as u32, 
                    2, 
                    gl::FLOAT, 
                    gl::FALSE, 
                    4i32 * size_of::<GLfloat>() as i32, 
                    (a * 2 * size_of::<GLfloat>()) as *const c_void
                );
                
                match gl::GetError() {
                    val if val != gl::NO_ERROR => {
                        print!("Something went wrong when initializing the wallpaper: ");
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
        }

        Wallpaper{vao, vbo, texture_id}
    }
}

impl Renderable for Wallpaper {
    fn draw(&mut self, program: &GraphicsProgram, viewport: Geometry) {
        unsafe {  
            gl::Disable(gl::CULL_FACE);

            gl::Viewport(viewport.origin.x, viewport.origin.y, viewport.size.w as i32, viewport.size.h as i32);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);

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


