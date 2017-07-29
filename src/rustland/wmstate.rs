extern crate serde;
extern crate serde_json;

use std::sync::{RwLock, Mutex};
use std::marker::Sync;

use common::definitions::FALLBACK_RESOLUTION;
use common::job::Job;
use io::physical::InputDevice;
use layout::transition::Transition;
use layout::*;
use utils::geometry::{PointExt, GeometryExt};

use wlc::*;

pub struct WMState{
    pub tree: LayoutTree,
    pub input_dev: Option<InputDevice>
}


unsafe impl Send for WMState {}
unsafe impl Sync for WMState {}

lazy_static! {
    pub static ref WM_STATE: RwLock<WMState> = RwLock::new(
        WMState{
            tree: LayoutTree::init(
                Geometry::new(Point::origin(), FALLBACK_RESOLUTION),
                1
            ),
            input_dev: None
        }
    );

    pub static ref PENDING_JOBS: Mutex<Vec<Job> >= Mutex::new(Vec::new());
    pub static ref FINALIZED_JOBS: Mutex<Vec<Job>> = Mutex::new(Vec::new());
    pub static ref ACTIVE_TRANSITIONS: Mutex<Vec<Transition>> = Mutex::new(Vec::new());
}

unsafe impl Send for ACTIVE_TRANSITIONS {}