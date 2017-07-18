#![feature(box_syntax, box_patterns)]
#![feature(associated_consts)]
#![feature(use_extern_macros)] 
#[allow(inaccessible_extern_crate)]

use std::sync::RwLock;
use std::io::Write;
use std::process::Command;
use std::io::Read;
use std::thread::{spawn, sleep};
use std::time;

#[macro_use] pub extern crate lazy_static;
#[macro_use] pub extern crate serde_derive;
#[macro_use] pub extern crate serde_json;

pub extern crate rustwlc;
use rustwlc::*;

pub extern crate num;
pub extern crate num_traits;

extern crate common;
use common::definitions::FPS;
use wmstate::{PENDING_JOBS, FINALIZED_JOBS, ACTIVE_TRANSITIONS};
use common::job::JobType;

mod layout;
use layout::LayoutTree;
use layout::arrangement::*;
use layout::property::ElementPropertyProvider;

mod io;
use io::physical::InputDevice;
use io::tcp_server::handle_incoming_requests;
use io::process_all_current_jobs;

mod utils;

mod wmstate;
use wmstate::{WMState, WM_STATE};

fn main() {
    // The default log handler will print wlc logs to stdout
    rustwlc::log_set_default_handler();
    let run_fn = rustwlc::init().expect("Unable to initialize!");

    callback::output_resolution(layout::on_output_resolution);
    callback::compositor_ready(compositor_ready);
    run_fn();
}

pub extern fn compositor_ready()
{
    let root = WlcView::root();
    
    if let Ok(mut wm_state) = WM_STATE.write(){
        wm_state.input_dev = Some(InputDevice::init());
        LayoutTree::refresh(&mut wm_state);
    }
    
    // Active callbacks
    callback::view_created(layout::element::window::on_view_created);
    callback::view_destroyed(layout::element::window::on_view_destroyed);
    callback::view_focus(layout::element::window::on_view_focus);
    callback::view_request_move(layout::element::window::on_view_request_move);
    callback::view_request_resize(layout::element::window::on_view_request_resize);

    spawn(move || {
        handle_incoming_requests();
    });     

    spawn(move || {
        loop{
            // Continous processing of jobs
            sleep(time::Duration::from_millis(10));
            process_all_current_jobs();
        }
    });    

    spawn(|| {
        let delta = 1000 / FPS;

        loop{
            if let Ok(ref mut active_transitions) = ACTIVE_TRANSITIONS.lock(){    
                if !active_transitions.is_empty(){     
                    if let Ok(mut wm_state) = WM_STATE.write(){  

                        // Continues to step forward each transition
                        for transition in active_transitions.iter_mut(){
                            transition.next(&mut wm_state.tree, delta);
                        }

                        // Completed transition should be in the list
                        active_transitions.retain(|ref transition| transition.is_ongoing());

                        // A layout refresh is necessary for the changes to apply
                        LayoutTree::refresh(&mut wm_state);
                    }
                }
            }
            
            sleep(time::Duration::from_millis(delta));
        }
    });
}

