use common::definitions::{WM_CATCH_EVENT, LEFT_CLICK, RIGHT_CLICK};
use common::job::{JobType};

use common::definitions::LayoutElemID;
use layout::element::bisect::Orientation;

use wlc::{Point, KeyState, ButtonState};
use wlc::input::pointer;

pub struct InputDevice {
    pub mouse_location: Point,
    pub left_click: ButtonState,
    pub right_click: ButtonState,

    pub resize: Option<(LayoutElemID, Orientation)>
}

impl InputDevice{
    pub fn none() -> InputDevice {
        InputDevice{
            mouse_location: Point{
                x: 0,
                y: 0
            },
            left_click: ButtonState::Released,
            right_click: ButtonState::Released,
            resize: None
        }
    }

    pub fn init() -> InputDevice {
        InputDevice{
            mouse_location: Point{
                x: 0,
                y: 0
            },
            left_click: ButtonState::Released,
            right_click: ButtonState::Released,
            resize: None
        }
    }

    pub fn mouse_travel(&mut self, disposition: Point) {
        self.mouse_location.x += disposition.x;
        self.mouse_location.y += disposition.y;
        pointer::set_position(self.mouse_location);
    }
}
