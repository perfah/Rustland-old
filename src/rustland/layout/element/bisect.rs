use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::cmp::{Eq, max};
use num::{cast, abs, clamp};

use layout::*;
use common::definitions::LayoutElemID;
use layout::element::ElementPropertyProvider;

pub use wlc::{Geometry, Point, Size};

pub const LOWER_SEGM_BOUND: i32 = 0;

#[derive(Serialize, Deserialize, Clone)]
pub enum Orientation{
    Horizontal,
    Vertical,
}

impl Orientation{
    pub fn opposite(&self) -> Self{
        match *self{
            Orientation::Horizontal => Orientation::Vertical,
            Orientation::Vertical => Orientation::Horizontal
        }
    }
}

impl Copy for Orientation {}

#[derive(Serialize, Deserialize, Clone, Hash)]
pub enum Side {
    Left,
    Right,
    Neither
}

impl Side{
    pub fn opposite(&self) -> Self{
        match *self{
            Side::Left => Side::Right,
            Side::Right => Side::Left,
            Side::Neither => panic!("No opposite of unspecified side!")
        }
    }
}


impl PartialEq for Side {
    fn eq(&self, other: &Self) -> bool {
        match self {
            other => true,
            _ => false
        }
    }
}
impl Eq for Side {}

#[derive(Serialize, Deserialize, Clone)]
pub struct Bisect{
    children: HashMap<Side, LayoutElemID>,
    pub orientation: Orientation,
    pub ratio: f32
}

impl Bisect{
    pub fn init(ident: LayoutElemID, tree: &mut LayoutTree, orientation: Orientation, ratio: f32) -> (LayoutElemID, Bisect) {
        let mut children = HashMap::new();
        children.insert(Side::Left, tree.spawn_dummy_element(Some(ident)));
        children.insert(Side::Right, tree.spawn_dummy_element(Some(ident)));

        let profile = Bisect{
            children: children,
            orientation: orientation,
            ratio: ratio
        };

        (ident, profile)
    }

    pub fn seat_child_on_side(&mut self, side: Side, child_ident: LayoutElemID) -> (Side, Option<LayoutElemID>){
        (side.clone(), self.children.insert(side, child_ident))
    }

    pub fn try_seat_child(&mut self, child_ident: LayoutElemID) -> Side {
        let mut iter = [Side::Left, Side::Right].iter();

        while let Some(side) = iter.next(){
            self.children.entry(side.clone()).or_insert(child_ident);
            // CHILD MUST BE NONE!!!
        }

        self.child_side(child_ident)
    }

    pub fn disown_child(&mut self, child_ident: LayoutElemID){
        self.children.retain(|_, &mut v| v != child_ident);
    }


    pub fn count_active_children(&self, tree: &LayoutTree) -> i32 {
        let mut iter = self.children_iter();
        let mut output = 0;

        while let Some(child_ident) = iter.next() {
            if tree.lookup_element(*child_ident).map_or(false, |child| !child.profile.is_none()) {
                output += 1;
            }
        }

        return output;
    }

    pub fn child_side(&self, element_ident: LayoutElemID) -> Side {
        match self.children.values().position(|&e| e == element_ident){
            Some(p) if p == 0 => Side::Left,
            Some(p) if p == 1 => Side::Right,
            _ => Side::Neither
        }
    }

    pub fn children_iter(&self) -> impl Iterator<Item = &LayoutElemID> {
        self.children.values()
    }

    pub fn get_orientation(self) -> Orientation{
        self.orientation
    }

    pub fn get_offset_geometry(&self, outer_geometry: Geometry, stacked_padding: &Option<u32>, child_index: i32) -> Geometry{
        let padding = (*stacked_padding).unwrap_or(0 as u32);
        let child_scale_factor = if child_index == 0 { self.ratio } else { 1.0f32 - self.ratio };

        Geometry{
            origin: match self.orientation{
                Orientation::Horizontal => {  
                    Point{
                        x: outer_geometry.origin.x 
                            + child_index * (self.ratio * outer_geometry.size.w as f32) as i32
                            + if child_index == LOWER_SEGM_BOUND { 0 } else { padding / 2 } as i32,
                        y: outer_geometry.origin.y
                    }
                }
                Orientation::Vertical => {
                    Point{
                        x: outer_geometry.origin.x,
                        y: outer_geometry.origin.y 
                            + child_index * (self.ratio * outer_geometry.size.h as f32) as i32
                            + if child_index == LOWER_SEGM_BOUND { 0 } else { padding / 2 } as i32,
                    }
                }
            },
            size: match self.orientation {
                Orientation::Horizontal => {  
                    Size{
                        w: max(0i32, ( child_scale_factor * outer_geometry.size.w as f32) as i32 - padding as i32 / 2) as u32,
                        h: outer_geometry.size.h
                    }
                }
                Orientation::Vertical => {
                    Size{
                        w: outer_geometry.size.w,
                        h: max(0i32, ( child_scale_factor * outer_geometry.size.h as f32) as i32 - padding as i32 / 2) as u32
                    }
                }
            }
        }
    }
}


impl ElementPropertyProvider for Bisect{
    fn register_properties(&self, property_bank: &mut PropertyBank){    
        property_bank.address_property("ratio", make_property_handle!(Bisect, f32, ratio, 0.05f32, 0.95f32));
    }
}