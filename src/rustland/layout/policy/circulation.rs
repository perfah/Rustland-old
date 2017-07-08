use layout::PARENT_ELEMENT;
use layout::LayoutTree;
use layout::arrangement::find_first_empty_element;
use layout::element::LayoutElement;
use layout::element::bisect::{Orientation, Bisect};
use layout::policy::*;

/*
    Generates a clockwise rotating layout 
*/

#[derive(Clone)]
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

impl LayoutPolicy for Circulation{
    fn attach_window(&mut self, mut tree: &mut LayoutTree) -> LayoutElemID {
        // Next orientation:
        self.last_orientation = self.last_orientation.opposite();
        
        if let Some(unoccupied_id) = find_first_empty_element(&tree, PARENT_ELEMENT){
            // Unoccupied spots preexisting in the layout makes this easy: 
            unoccupied_id
        }
        else{
            // Otherwise, we have to extend the layout with new spots:
            println!("NOTICE: Extending the layout structure!");

            if let Some(last_id) = tree.last_window_id(){
                let extension = Bisect::init(&mut tree, self.last_orientation.clone());
                let new_preoccupied_id = extension.get_children()[0];
                let new_unoccupied_id = extension.get_children()[1];

                // update tags according to element swap
                tree.tags.handle_element_swap(last_id, new_preoccupied_id);

                if let Some(thrown_out) = tree.insert_element_at(LayoutElement::Bisect(extension), last_id){
                    tree.swap_cell(new_preoccupied_id, thrown_out);
                    new_unoccupied_id
                }
                else {
                    panic!("ERROR: No space in layout found!")
                }
            }
            else{
                panic!("Last index did not exist!");
            }
        }
    }

    fn box_clone(&self) -> Box<LayoutPolicy> {
        Box::new((*self).clone())
    }
}
