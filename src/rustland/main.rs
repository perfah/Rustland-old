#![feature(box_syntax, box_patterns)]
#[allow(inaccessible_extern_crate)]

use std::sync::RwLock;
use std::io::Write;
use std::process::Command;
use std::io::Read;
use std::thread::{spawn, sleep};
use std::time;

#[macro_use]
pub extern crate lazy_static;
#[macro_use]
pub extern crate serde_derive;
#[macro_use]
pub extern crate serde_json;

pub extern crate rustwlc;
use rustwlc::*;

extern crate common;
use common::definitions;
use wmstate::{PENDING_JOBS, FINALIZED_JOBS};
use common::job::JobType;

mod layout;
mod io;
use io::physical::InputDevice;

pub mod wmstate;
use wmstate::{WMState, WM_STATE};
use layout::LayoutTree;
use layout::arrangement::*;
use io::tcp_server::handle_incoming_requests;
use io::process_all_current_jobs;

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
}