use std::sync::Mutex;

extern crate rustwlc;

use rustwlc::*;
use rustwlc::xkb::keysyms;
use rustwlc::xkb::Keysym;
use rustwlc::types::{ButtonState, KeyboardModifiers, KeyState, KeyboardLed, ScrollAxis, Size,
                     Point, Geometry, ResizeEdge, ViewState, VIEW_ACTIVATED, VIEW_RESIZING,
                     MOD_NONE, MOD_CTRL, RESIZE_LEFT, RESIZE_RIGHT, RESIZE_TOP, RESIZE_BOTTOM};

use wmstate::*;
use definitions::{WM_FORWARD_EVENT_TO_CLIENT, WM_CATCH_EVENT, LEFT_CLICK, RIGHT_CLICK};

use layout::arrangement::*;
use layout::*;
use layout::element::LayoutElement;

pub struct InputDevice {
    pub mouse_location: Point,
    pub resizing: bool
}

impl InputDevice{
    pub fn none() -> InputDevice
    {
        InputDevice{
            mouse_location: Point{
                x: 0,
                y: 0
            },
            resizing: false
        }
    }

    pub fn init() -> InputDevice
    {
        callback::pointer_motion(on_pointer_motion);
        callback::pointer_button(on_pointer_button);
        callback::keyboard_key(on_keyboard_key);
        callback::pointer_scroll(pointer_scroll);

        InputDevice{
            mouse_location: Point{
                x: 0,
                y: 0
            },
            resizing: false
        }
    }

    fn mouseTravel(&mut self, disposition: Point)
    {
        self.mouse_location.x += disposition.x;
        self.mouse_location.y += disposition.y;
        rustwlc::input::pointer::set_position(self.mouse_location);
    }
}

pub extern fn on_pointer_motion(_in_view: WlcView, _time: u32, point: &Point) -> bool {
    let mut wm_state = WM_STATE.lock().unwrap();

    let dx = point.x - wm_state.input_dev.mouse_location.x;
    let dy = point.y - wm_state.input_dev.mouse_location.y;

    wm_state.input_dev.mouseTravel(
        Point{
            x: dx,
            y: dy
        }
    );

    // Note: Forward is REQUIRED for input to be registered by clients
    WM_FORWARD_EVENT_TO_CLIENT
}

pub extern fn pointer_scroll(_view: WlcView, _time: u32,
                         _mods_ptr: &KeyboardModifiers, _axis: ScrollAxis,
                         _heights: [f64; 2]) -> bool {
    WM_FORWARD_EVENT_TO_CLIENT
}

extern fn on_pointer_button(view: WlcView, _time: u32, mods: &KeyboardModifiers, button: u32, state: ButtonState, point: &Point) -> bool {
    use std::process::Command;

    if state == ButtonState::Pressed {
        match button{
            LEFT_CLICK => {  },
            RIGHT_CLICK => { },
            _ => {  }
        }

        if !view.is_root() && mods.mods.contains(MOD_CTRL) {
            if mods.mods.contains(MOD_CTRL) {
                // Button left, we need to include linux/input.h somehow
                //if button == 0x110 {
                    //start_interactive_move(&view, point);
                //}
                //else if button == 0x111 {
                    //start_interactive_resize(&view, ResizeEdge::empty(), point);
                //}

                return WM_CATCH_EVENT;
            }
        }
    }
    else {
        //stop_interactive_action();
    
    }

    WM_FORWARD_EVENT_TO_CLIENT
}

fn get_topmost_view(output: WlcOutput, offset: usize) -> Option<WlcView> {
    let views = output.get_views();
    if views.is_empty() { None }
    else {
        Some(views[(views.len() - 1 + offset) % views.len()])
    }
}

extern fn on_keyboard_key(view: WlcView, _time: u32, mods: &KeyboardModifiers, key: u32, state: KeyState) -> bool {
    use std::process::Command;
    let sym = input::keyboard::get_keysym_for_key(key, *mods);

    if state == KeyState::Pressed {
        let mut wm_state = WM_STATE.lock().unwrap();

        //Press F4 for tree view
        if sym == keysyms::KEY_F3{
            println!();
            println!("~ Layout structure ~");
            tree(&wm_state, PARENT_ELEMENT);
            return WM_CATCH_EVENT;
        }

        //Press F5 to force an update to the arrangement
        if sym == keysyms::KEY_F5{
            wm_state.tree.arrange();
            return WM_CATCH_EVENT;
        }

        if mods.mods == MOD_CTRL {
            if sym == keysyms::KEY_Left || sym == keysyms::KEY_Right {
                use std::cmp;
                use std::u16;

                if let Some(mut element) = wm_state.tree.lookup_element(1) {
                    match *element{
                        LayoutElement::Workspace(ref mut wrkspc) => {
                            match sym{
                                keysyms::KEY_Left => {
                                    wrkspc.active_child = cmp::max(wrkspc.active_child - 1, 1);
                                    println!("Switched workspace to the left: {}", wrkspc.active_child);
                                },
                                keysyms::KEY_Right => {
                                    wrkspc.active_child = cmp::min(wrkspc.active_child + 1, u16::MAX);
                                    println!("Switched workspace to the right: {}", wrkspc.active_child);
                                },
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }

                wm_state.tree.arrange();
            }

            if sym == keysyms::KEY_c {
                if !view.is_root() {
                    view.close();
                }
                
                return WM_CATCH_EVENT;
            } 
            else if sym == keysyms::KEY_Tab {
                view.send_to_back();
                get_topmost_view(view.get_output(), 0).unwrap().focus();
                return WM_CATCH_EVENT;
            }
            else if sym == keysyms::KEY_Escape {
                terminate();
                return WM_CATCH_EVENT;
            }
            else if sym == keysyms::KEY_space {
                let _ = Command::new("sh")
                    .arg("-c")
                    .arg("/usr/bin/dmenu_run")
                    .spawn()
                    .unwrap_or_else(|e| {
                        panic!("Can't spawn process!");
                    });
                return WM_CATCH_EVENT;
            }
        }
    }
    return WM_FORWARD_EVENT_TO_CLIENT;
}