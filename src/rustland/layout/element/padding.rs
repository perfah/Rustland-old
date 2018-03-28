use std::cell::RefMut;
use std::cmp::max;

use common::definitions::{DefaultNumericType, LayoutElemID};
use layout::LayoutTree;
use layout::element::{LayoutElement, LayoutElementProfile};
use layout::property::{ElementPropertyProvider, PropertyBank};
use sugars::program::GraphicsProgram;
use sugars::Renderable;
use sugars::frame::Frame;
use utils::geometry::{PointExt, GeometryExt};

use wlc::*;
use num::traits::cast;

#[derive(Serialize, Deserialize, Clone)]
pub struct Padding{
    pub child_elem_id: LayoutElemID,
    pub gap_size: u32,
    pub inner_scale_x: f32, 
    pub inner_scale_y: f32,
    pub positioning_offset: Option<Point>,
    pub frame: Option<Frame>
}

impl Padding{
    pub fn init(ident: LayoutElemID, tree: &mut LayoutTree, gap_size: u32, positioning_offset: Option<Point>) -> (LayoutElemID, Padding) {
        let profile = Padding{
            child_elem_id: tree.spawn_dummy_element(Some(ident)),
            gap_size: gap_size,
            inner_scale_x: 1.0f32,
            inner_scale_y: 1.0f32,
            positioning_offset: positioning_offset,
            frame: None
        };

        (ident, profile)
    }
    
    pub fn apply_frame(&mut self, graphics_program: &GraphicsProgram, initial_opacity: f32){
        self.frame = Some(Frame::new(graphics_program.id, initial_opacity));
    }
    
    pub fn get_offset_geometry(&self, outer_geometry: Geometry, stacked_scale: &mut (f32, f32)) -> Geometry{
        let offset = self.positioning_offset.unwrap_or(Point::origin());

        Geometry{
            origin: Point{ 
                x: offset.x + outer_geometry.origin.x + self.gap_size as i32, 
                y: offset.y + outer_geometry.origin.y + self.gap_size as i32
            },
            size: Size{ 
                w: outer_geometry.size.w.checked_sub(self.gap_size.checked_mul(2).unwrap_or_default()).unwrap_or_default(),
                h: outer_geometry.size.h.checked_sub(self.gap_size.checked_mul(2).unwrap_or_default()).unwrap_or_default()
            }   
        }.scaled(self.inner_scale_x, self.inner_scale_y)
    }
}

impl ElementPropertyProvider for Padding{
    fn register_properties(&self, property_bank: &mut PropertyBank){    
        property_bank.address_property("gap_size", make_property_handle!(Padding, u32, gap_size));
        
        property_bank.address_property("inner_scale_x", make_property_handle!(Padding, u32, inner_scale_x));
        property_bank.address_property("inner_scale_y", make_property_handle!(Padding, u32, inner_scale_y));

        property_bank.address_property("offset_x", |profile: &mut LayoutElementProfile, new_value: Option<DefaultNumericType>| {
            assist_property_handle!(Padding, profile, padding, {
                if let Some(ref mut offset) = padding.positioning_offset{
                    if let Some(value) = new_value { 
                        offset.x = value as i32; 
                    }

                    Some(&offset.x)
                }
                else { None }
            }
        )});

        property_bank.address_property("offset_y", |profile: &mut LayoutElementProfile, new_value: Option<DefaultNumericType>| {
            assist_property_handle!(Padding, profile, padding, {
                if let Some(ref mut offset) = padding.positioning_offset{
                    if let Some(value) = new_value { 
                        offset.y = value as i32; 
                    }

                    Some(&offset.y)
                }
                else { None }
            })
        });

        property_bank.address_property("frame_opacity", |profile: &mut LayoutElementProfile, new_value: Option<DefaultNumericType>| {
            assist_property_handle!(Padding, profile, padding, {
                if let Some(ref mut frame) = padding.frame{
                    if let Some(value) = new_value { 
                        frame.opacity = value; 
                    }

                    Some(&frame.opacity)
                }
                else { None }
            })
        });
    }
}

impl Renderable for Padding {
    fn draw(&mut self, program: &GraphicsProgram, viewport: Geometry){
        if let Some(ref mut frame) = self.frame {
            frame.draw(program, viewport);
        }
    }   
}