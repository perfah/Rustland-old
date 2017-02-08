pub mod workspace;
pub mod window;
pub mod segmentation;

use std::fmt;

pub enum LayoutElement
{
    // Unallocated space in the layout
    None,

    // A segmentation of multiple child elements
    Segm(segmentation::Segmentation),

    // A passthrough container with exactly one active child element at a time
    Workspace(workspace::Workspace),
    
    // An application
    Window(window::Window)
}


impl fmt::Debug for LayoutElement{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!();
    }
}