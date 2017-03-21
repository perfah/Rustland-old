#![feature(box_syntax, box_patterns)]
#[allow(inaccessible_extern_crate)]

use std::sync::RwLock;
use std::io::Write;
use std::process::Command;
use std::io::Read;
use std::thread;

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
use wmstate::WM_STATE;
use layout::arrangement::*;
use io::tcp_server::handle_incoming_requests;


fn main() {
    callback::compositor_ready(compositor_ready);
    callback::view_created(layout::element::window::on_view_created);
    callback::view_destroyed(layout::element::window::on_view_destroyed);
    callback::view_focus(layout::element::window::on_view_focus);
    callback::view_request_move(layout::element::window::on_view_request_move);
    callback::view_request_resize(layout::element::window::on_view_request_resize);
    callback::output_resolution(layout::on_output_resolution);

    // The default log handler will print wlc logs to stdout
    rustwlc::log_set_default_handler();
    let run_fn = rustwlc::init().expect("Unable to initialize!");

    run_fn();
    
}

pub extern fn compositor_ready()
{
    let root = WlcView::root();
    
    if let Ok(mut wm_state) =  WM_STATE.lock()
    {
        wm_state.input_dev = InputDevice::init();
        
        wm_state.tree.refresh();
    }

    thread::spawn(move || {
        handle_incoming_requests();
    });     
}
