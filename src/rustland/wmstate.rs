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
                    if let Ok(img) = handle.join(){
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

    pub fn render_wallpaper(&mut self){
        if let Some(ref mut existing_program) = self.graphics_program{
            if let Some(ref mut existing_wallpaper) = self.wallpaper{
                existing_program.run_job(existing_wallpaper, self.tree.get_outer_geometry());
            }
        }
    }
}


unsafe impl Send for WMState {}
unsafe impl Sync for WMState {}

lazy_static! {
    pub static ref WM_STATE: RwLock<WMState> = RwLock::new(
        WMState{
            tree: LayoutTree::init(Geometry::new(Point::origin(), FALLBACK_RESOLUTION)),
            input_dev: None,
            graphics_program: None,
            wallpaper: None,
            next_wallpaper_image: None
        }
    );

    pub static ref PENDING_JOBS: Mutex<Vec<Job> >= Mutex::new(Vec::new());
    pub static ref FINALIZED_JOBS: Mutex<Vec<Job>> = Mutex::new(Vec::new()); 
    pub static ref ACTIVE_TRANSITIONS: Mutex<Vec<Transition>> = Mutex::new(Vec::new());
}

unsafe impl Send for ACTIVE_TRANSITIONS {}