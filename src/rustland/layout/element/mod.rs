pub mod workspace;
pub mod window;
pub mod bisect;
pub mod padding;

use std::fmt;
use super::LayoutTree;

pub enum LayoutElement
{
    // Unallocated space in the layout
    None,

    // A container of exactly two child elements
    Bisect(bisect::Bisect),

    // A container of multiple child elements with only one active in a given moment
    Workspace(workspace::Workspace),
    
    // A container that can be smaller in relation to the outside geometry 
    Padding(padding::Padding),

    // An arbitrary window/application
    Window(window::Window)
}


impl fmt::Debug for LayoutElement{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!();
    }
}