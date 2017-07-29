use common::definitions::{LayoutElemID};
use layout::LayoutTree;

pub mod circulation;

pub trait LayoutPolicy{
    fn attach_window(&mut self, tree: &mut LayoutTree) -> LayoutElemID;
    fn box_clone(&self) -> Box<LayoutPolicy>;
}

impl Clone for Box<LayoutPolicy>{
    fn clone(&self) -> Self {
        self.box_clone()
    }
}