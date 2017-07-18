use common::definitions::{DefaultNumericType, LayoutElemID};
use layout::LayoutTree;
use layout::property::ElementPropertyProvider;
use utils::interpolation::NumericInterpolation;
use utils::interpolation::methods::{LinearInterpolator, DecelerationInterpolator};

pub struct Transition{
    pub element_id: LayoutElemID,
    pub transitioning_property: String,
    interpolation: NumericInterpolation,
    time_frame_ms: u64
}

impl Transition{
    pub fn new(element_id: LayoutElemID, transitioning_property: String, value_origin: DefaultNumericType, value_dest: DefaultNumericType, relative_transition: bool, time_frame_ms: u64) -> Transition{
        let interpolation = NumericInterpolation::new(
            box DecelerationInterpolator{}, 
            value_origin, 
            if relative_transition { value_origin + value_dest} else { value_dest }, 
            0
        );

        Transition{
            element_id: element_id,
            transitioning_property: transitioning_property,
            interpolation: interpolation,
            time_frame_ms: time_frame_ms
        }
    }

    pub fn next(&mut self, tree: &mut LayoutTree, time_delta_ms: u64) -> bool{
        self.interpolation.intervals = self.time_frame_ms / time_delta_ms;

        if let Some(ref mut elem) = tree.lookup_element(self.element_id){
            let mut new_value = 0f32;
            let result = self.interpolation.next(&mut new_value);
            (*elem).set_property(tree, self.element_id, self.transitioning_property.clone(), new_value);
            result
        }
        else{
            panic!("Invalid element!");  
        }
        
    }

    pub fn is_ongoing(&self) -> bool{
        self.interpolation.is_ongoing()
    }
}

