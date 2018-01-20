extern crate serde;
extern crate serde_json;

use std::sync::{RwLock, Mutex};
use std::marker::Sync;
use std::cell::{RefCell, RefMut};
use std::fs::File;
use std::thread::JoinHandle;
use std::sync::atomic::AtomicBool;

use common::definitions::FALLBACK_RESOLUTION;
use common::job::Job;
use io::physical::InputDevice;
use layout::transition::Transition;
use layout::*;
use sugars::program::GraphicsProgram;
use sugars::wallpaper::Wallpaper;
use sugars::solid_color::SolidColor;
use sugars::frame::Frame;
use utils::geometry::{PointExt, GeometryExt};

use wlc::*;

use image::RgbaImage;
use gl;
use gl::types::{GLint, GLuint};
use thread_tryjoin::TryJoinHandle;

pub struct WMState{
    pub tree: LayoutTree,
    pub input_dev: Option<InputDevice>,
    pub graphics_program: Option<GraphicsProgram>,
    wallpaper: Option<Wallpaper>,
    solid_color: SolidColor,
    pub next_wallpaper_image: Option<JoinHandle<RgbaImage>>
}

impl WMState {
    pub fn init_graphics_program(&mut self) -> GLuint{
        let program = GraphicsProgram::init();
        let id = program.id;        

        self.graphics_program = Some(program);
        id
    }

    pub fn refresh_wallpaper(&mut self){
        self.wallpaper = match self.graphics_program{
            Some(ref program) => {
                let mut wallpaper = None;
                
                if let Some(handle) = self.next_wallpaper_image.take(){
                    if let Ok(img) = handle.join() {
                        program.safe_graphics_operation(|program_id| {
                            wallpaper = Some(Wallpaper::new(&img, program_id)); 
                        });
                    }

                }

                wallpaper
            },
            None => panic!("No graphics program available!")
        };

        println!("Wallpaper set."); 
    }

    pub fn render_background(&mut self){
        if let Some(ref mut program) = self.graphics_program {
            let geometry = self.tree.get_outer_geometry();

            match self.wallpaper {
                Some(ref mut wallpaper) => program.run_job(wallpaper, geometry),
                None => program.run_job(&mut self.solid_color, geometry)
            }
        }
    }
}


unsafe impl Send for WMState {}
unsafe impl Sync for WMState {}

lazy_static! {
    pub static ref WM_STATE: RwLock<WMState> = RwLock::new(
        WMState{
            tree: LayoutTree::init(Geometry::new(Point::origin(), FALLBACK_RESOLUTION), 3, 3),
            input_dev: None,
            graphics_program: None,
            wallpaper: None,
            solid_color: SolidColor::new(1.0, 1.0, 1.0, 0.1),
            next_wallpaper_image: None
        }
    );

    pub static ref PENDING_JOBS: Mutex<Vec<Job> >= Mutex::new(Vec::new());
    pub static ref FINALIZED_JOBS: Mutex<Vec<Job>> = Mutex::new(Vec::new()); 
    pub static ref ACTIVE_TRANSITIONS: RwLock<Vec<Transition>> = RwLock::new(Vec::new());
}

unsafe impl Send for ACTIVE_TRANSITIONS {}

