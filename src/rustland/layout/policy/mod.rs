use std::sync::RwLockWriteGuard;

use common::definitions::{LayoutElemID};
use layout::LayoutTree;
use layout::element::window::Window;
use sugars::program::GraphicsProgram;
use wmstate::WMState;

use gl::types::GLuint;
use wlc::WeakView;

pub mod auto_circulation;

pub trait LayoutPolicy{
    fn seat_window(&mut self, tree: &mut LayoutTree) -> LayoutElemID;
    fn decorate_window(&mut self, wm_state: &mut RwLockWriteGuard<WMState>, element_ident: LayoutElemID);
    fn box_clone(&self) -> Box<LayoutPolicy>;
    fn detach_window(&mut self, tree: &mut LayoutTree, element_ident: LayoutElemID) -> Option<WeakView>;
}

impl Clone for Box<LayoutPolicy>{
    fn clone(&self) -> Self {
        self.box_clone()
    }
}