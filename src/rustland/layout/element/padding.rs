use common::definitions::{DefaultNumericType, LayoutElemID};
use layout::LayoutTree;
use layout::element::LayoutElement;
use layout::property::{ElementPropertyProvider, PropertyBank};
use utils::geometry::PointExt;

use wlc::*;
use num::traits::cast;

pub struct Padding{
    pub child_elem_id: LayoutElemID,
    pub gap_size: u32,
    pub positioning_offset: Option<Point>
}

impl Padding{
    pub fn init(tree: &mut LayoutTree, gap_size: u32, positioning_offset: Option<Point>) -> Padding{
        Padding{
            child_elem_id: tree.spawn_element(),
            gap_size: gap_size,
            positioning_offset: positioning_offset
        }
    }

    pub fn get_offset_geometry(&self, outer_geometry: Geometry) -> Geometry{
        let offset = self.positioning_offset.unwrap_or(Point::origin());
        Geometry{
            origin: Point{ 
                x: offset.x + outer_geometry.origin.x + self.gap_size as i32, 
                y: offset.y + outer_geometry.origin.y + self.gap_size as i32
            },
            size: Size{ 
                w: outer_geometry.size.w - 2 * self.gap_size,
                h: outer_geometry.size.h - 2 * self.gap_size
            }
        }
    }
}

impl ElementPropertyProvider for Padding{
    fn register_properties(&self, property_bank: &mut PropertyBank){    
        property_bank.address_property("gap_size".to_string(), make_property_handle!(Padding, u32, gap_size));

        property_bank.address_property("offset_x".to_string(), |element: &mut LayoutElement, new_value: Option<DefaultNumericType>| {
            assist_property_handle!(Padding, element, padding, {
                if let Some(ref mut offset) = padding.positioning_offset{
                    if let Some(value) = new_value { 
                        offset.x = value as i32; 
                    }

                    Some(&offset.x)
                }
                else { None }
            }
        )});

        property_bank.address_property("offset_y".to_string(), |element: &mut LayoutElement, new_value: Option<DefaultNumericType>| {
            assist_property_handle!(Padding, element, padding, {
                if let Some(ref mut offset) = padding.positioning_offset{
                    if let Some(value) = new_value { 
                        offset.y = value as i32; 
                    }

                    Some(&offset.y)
                }
                else { None }
            }
        )});
    }
}
