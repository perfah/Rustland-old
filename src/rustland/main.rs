#![feature(box_syntax, box_patterns)]
#![feature(associated_consts)]
#![feature(use_extern_macros)] 
#![feature(conservative_impl_trait)] 
#[allow(inaccessible_extern_crate)]

#[macro_use] pub extern crate lazy_static;
#[macro_use] pub extern crate serde_derive;
#[macro_use] pub extern crate serde_json;

pub extern crate wlc;
pub extern crate num;
pub extern crate num_traits;
pub extern crate egli;
pub extern crate image;
pub extern crate gl;
pub extern crate thread_tryjoin;

extern crate common;

mod layout;
mod io;
mod utils;
mod compositor;
mod wmstate;
mod sugars;
mod async;

use compositor::Compositor;

fn main() {
    wlc::init(Compositor).unwrap();
}

