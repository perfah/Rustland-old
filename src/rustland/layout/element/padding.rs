use num::traits::{NumCast,ToPrimitive};
use std::collections::HashMap;

use common::definitions::{DefaultNumericType, LayoutElemID};
use layout::LayoutTree;
use layout::element::LayoutElement;
use layout::property::{PropertyProvider, PropertyBank};

use rustwlc::*;
use num::traits::cast;

pub struct Padding{
    pub child_elem_id: LayoutElemID,
    pub gap_size: u32
}

impl Padding{
    pub fn init(tree: &mut LayoutTree, gap_size: u32) -> Padding{
        Padding{
            child_elem_id: tree.spawn_element(),
            gap_size: gap_size
        }
    }

    pub fn get_offset_geometry(&self, outer_geometry: Geometry) -> Geometry{
        Geometry{
            origin: Point{ 
                x: outer_geometry.origin.x + self.gap_size as i32, 
                y: outer_geometry.origin.y + self.gap_size as i32
            },
            size: Size{ 
                w: outer_geometry.size.w - 2 * self.gap_size,
                h: outer_geometry.size.h - 2 * self.gap_size
            }
        }
    }
}

impl PropertyProvider for Padding{
    fn register_properties(&self, property_bank: &mut PropertyBank){    
        property_bank.address_property("gap_size".to_string(), make_property_handle!(Padding, gap_size, u32));
    }
}
