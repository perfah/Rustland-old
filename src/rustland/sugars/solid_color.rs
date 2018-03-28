use std::os::raw::c_void;
use std::mem::{size_of, transmute};

use sugars::{Renderable, GraphicsProgram};

use wlc::Geometry;
use image::RgbaImage;
use gl;
use gl::types::{GLint, GLuint, GLfloat, GLchar, GLenum};

pub struct SolidColor {
    r: f32,
    g: f32,
    b: f32,
    a: f32
}

impl SolidColor {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> SolidColor {
        SolidColor{ r, g, b, a }
    }
}

impl Renderable for SolidColor {
    fn draw(&mut self, program: &GraphicsProgram, viewport: Geometry) {
        unsafe {           
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable( gl::BLEND );
            gl::Viewport(viewport.origin.x, viewport.origin.y, viewport.size.w as i32, viewport.size.h as i32);
            gl::ClearColor(self.r, self.g, self.b, self.a);
            gl::Clear(gl::COLOR_BUFFER_BIT);    

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


