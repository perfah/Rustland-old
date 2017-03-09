
use std::sync::Mutex;
use std::marker::Sync;
use std::cell::*;
use std::rc::Rc;

use rustwlc::*;
use io::physical::InputDevice;

use layout::*;
use layout::element::workspace::Workspace;
use layout::arrangement::*;
use layout::rules::*;

extern crate serde;
extern crate serde_json;
use self::serde_json::Map;

pub struct WMState{
    pub input_dev: InputDevice,
    pub tree: LayoutTree,
    pub resolution: Size
}

unsafe impl Send for WMState {}

impl WMState{
    fn new(resolution: Size, no_monitors: u16) -> WMState{
        WMState{
            input_dev: InputDevice::none(),
            tree: LayoutTree::init(
                Geometry::new(Point{x: 0, y: 0}, resolution),
                no_monitors, 
                RefCell::new(Box::new(Circulation::init()))
            ),
            resolution: resolution
        }
    }

}

lazy_static! {
    pub static ref WM_STATE: Mutex<WMState> = Mutex::new(WMState::new(
        Size{
            w: 1920,
            h: 1080
        },
        1
    ));
}
