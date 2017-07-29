#![feature(box_syntax, box_patterns)]
#![feature(associated_consts)]
#![feature(use_extern_macros)] 
#[allow(inaccessible_extern_crate)]

#[macro_use] pub extern crate lazy_static;
#[macro_use] pub extern crate serde_derive;
#[macro_use] pub extern crate serde_json;

pub extern crate wlc;
pub extern crate num;
pub extern crate num_traits;
extern crate common;

mod layout;
mod io;
mod utils;
mod compositor;
mod wmstate;

use compositor::Compositor;

fn main() {
    wlc::init(Compositor).unwrap();
}

