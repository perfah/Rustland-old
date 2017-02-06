use std::cell::*;
use std::borrow::Borrow;
use std::collections::HashMap;

use rustwlc::*;

use wmstate::*;
use layout::segmentation::*;
use layout::LayoutElement;

use super::arrangement::*;
use super::*;



pub struct Workspace{
    active: bool,
    child: LayoutElemID 
}

impl Workspace{
    pub fn new(tree: &mut LayoutTree) -> Workspace{
        Workspace{
            active: true,
            child: tree.spawn_element()
        }
    }

    pub fn is_active(&self) -> bool{
        self.active
    }
}
