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
        if let Some(unoccupied_id) = find_first_empty_element(tree, super::PARENT_ELEMENT)
        {
            unoccupied_id
        }
        else
        {
            println!("NOTICE: Extending the layout structure!");

            let last_id = tree.last_window_id();
            let extension = Segmentation::init_vert_50_50(tree);
            let preoccupied_id = extension.get_children()[0];
            let unoccupied_id = extension.get_children()[1];

            if let Some(tmp) = tree.swap_element(last_id, LayoutElement::Segm(extension))
            {
                tree.swap_cell(preoccupied_id, tmp);
                unoccupied_id
            }
            else {
                panic!("Last index did not exist!");
            }
        }
    }
}
