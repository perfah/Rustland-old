use std::sync::{RwLock, Mutex};
use std::marker::Sync;
use std::cell::{RefCell, RefMut};
use std::fs::File;
use std::thread::JoinHandle;
use std::sync::atomic::AtomicBool;

use common::definitions::FALLBACK_RESOLUTION;
use common::job::Job;
use config::Config;
use io::physical::InputDevice;
use layout::transition::Transition;
use layout::*;
use layout::element::LayoutElementProfile;
use layout::element::padding::Padding;
use layout::tag::TagRegister;
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
    pub config: Config,
    pub tree: LayoutTree,
    pub input_dev: Option<InputDevice>,
    pub graphics_program: Option<GraphicsProgram>,
    wallpaper: Option<Wallpaper>,
    pub solid_color: Option<SolidColor>,
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
            let mut scale = (1.0f32, 1.0f32);    
            
            let total_geom = self.tree.get_outer_geometry();
            let geometry = match self.tree.lookup_element_by_tag(self.config.layout.jumper_tag.clone()).first().expect("No jumper!").profile{
                LayoutElementProfile::Padding(ref root) => root.get_offset_geometry(self.tree.get_outer_geometry(), &mut scale),
                _ => total_geom
            };
            

            if self.solid_color.is_some(){    
                program.run_job(self.solid_color.as_mut().unwrap(), total_geom);
            }

            if let Some(ref mut wallpaper) = self.wallpaper {
                program.run_job(wallpaper, total_geom);
            }
        }
    }
}


unsafe impl Send for WMState {}
unsafe impl Sync for WMState {}

lazy_static! {
    pub static ref WM_STATE: RwLock<WMState> = RwLock::new(
        WMState{
            config: Config::default(),
            tree: LayoutTree::init(Geometry::new(Point::origin(), FALLBACK_RESOLUTION), 1, 1),
            input_dev: None,
            graphics_program: None,
            wallpaper: None,
            solid_color: None,
            next_wallpaper_image: None
        }
    );

    pub static ref PENDING_JOBS: Mutex<Vec<Job> >= Mutex::new(Vec::new());
    pub static ref FINALIZED_JOBS: Mutex<Vec<Job>> = Mutex::new(Vec::new()); 
    pub static ref ACTIVE_TRANSITIONS: Mutex<Vec<Transition>> = Mutex::new(Vec::new());
}

unsafe impl Send for ACTIVE_TRANSITIONS {}

