use std::cell::*;
use std::borrow::Borrow;
use std::collections::HashMap;

use rustwlc::*;

use wmstate::*;
use definitions::LayoutElemID;
use layout::*;
use layout::element::segmentation::*;
use super::LayoutElement;
use layout::arrangement::*;

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
