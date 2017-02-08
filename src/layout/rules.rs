use std::sync::MutexGuard;

use super::*;
use super::arrangement::*;
use super::element::*;

pub trait RuleSet{
    fn dock_window(&self, tree: &mut LayoutTree) -> LayoutElemID;
}


/*
    A set of rules that will make the windows appear spinning
*/

pub struct Circulation{
    last_orientation: Orientation
}

impl Circulation{
    pub fn init() -> Circulation{
        Circulation{
            last_orientation: Orientation::Horizontal
        }
    }
}

impl RuleSet for Circulation{
    fn dock_window(&self, tree: &mut LayoutTree) -> LayoutElemID {
        unimplemented!()
    }
}
