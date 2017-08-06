use common::definitions::{DefaultNumericType, LayoutElemID};
use layout::LayoutTree;
use utils::interpolation::NumericInterpolation;
use utils::interpolation::methods::SineInterpolator;

pub struct Transition{
    pub element_id: LayoutElemID,
    pub transitioning_property: String,
    interpolation: NumericInterpolation,
    time_frame_ms: u64,
    delay_ms: u64
}

impl Transition{
    pub fn new(element_id: LayoutElemID, transitioning_property: String, value_origin: DefaultNumericType, value_dest: DefaultNumericType, relative_transition: bool, time_frame_ms: u64, delay_ms: u64) -> Transition{
        let interpolation = NumericInterpolation::new(
            box SineInterpolator{}, 
            value_origin, 
            if relative_transition { value_origin + value_dest} else { value_dest }, 
            0
        );

        Transition{
            element_id: element_id,
            transitioning_property: transitioning_property,
            interpolation: interpolation,
            time_frame_ms: time_frame_ms,
            delay_ms: delay_ms
        }
    }

    pub fn next(&mut self, tree: &mut LayoutTree, time_delta_ms: u64) -> bool {
        if self.delay_ms > 0u64 {
            self.delay_ms = self.delay_ms.checked_sub(time_delta_ms).unwrap_or(0u64);
            true
        }
        else {
            self.interpolation.intervals = self.time_frame_ms / time_delta_ms;
            if let Some(ref mut elem) = tree.lookup_element(self.element_id){
                let mut new_value = 0f32;
                let result = self.interpolation.next(&mut new_value);
                elem.set_property(self.transitioning_property.clone(), new_value);
                result
            }
            else{
                panic!("Invalid element!");  
            }
        }
        
    }

    pub fn is_ongoing(&self) -> bool{
        self.interpolation.is_ongoing()
    }
}

