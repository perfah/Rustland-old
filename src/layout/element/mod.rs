pub mod workspace;
pub mod window;
pub mod segmentation;

use std::fmt;

pub enum LayoutElement
{
    // Unallocated space in the layout
    None,

    // A reactive container with exactly one child element that can be turned on or off
    Workspace(workspace::Workspace),

    // A segmentation of multiple child elements
    Segm(segmentation::Segmentation),
    
    // An application
    Window(window::Window)
}


impl fmt::Debug for LayoutElement{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!();
    }
}