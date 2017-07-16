#![macro_use]

use std::collections::HashMap;
use std::iter::FromIterator;
use num_traits::cast::{NumCast,ToPrimitive};
use num::traits::cast;

use common::definitions::{DefaultNumericType, LayoutElemID};
use layout::LayoutTree;
use layout::element::LayoutElement;

type handle_function = Fn(&mut LayoutElement, Option<DefaultNumericType>) -> Option<&ToPrimitive>;

macro_rules! assist_property_handle{
    ($elem_type:ident, $element:expr, $element_nick:ident, $handling:block) => ({        
        match *$element{
            LayoutElement::$elem_type(ref mut $element_nick) => {
                $handling
            },
            _ => panic!("This shouldn't compile.")
        }
    })
}

macro_rules! make_property_handle{
    ($elem_type:ident, $var_type: ty, $var_name:ident) => (|element: &mut LayoutElement, new_value: Option<DefaultNumericType>| {        
        match *element{
            LayoutElement::$elem_type(ref mut matched_element) => {
                if let Some(value) = new_value{
                    matched_element.$var_name = cast(value).expect("Casting error - is the last argument type numeric?");
                }

                Some(&matched_element.$var_name)
            },
            _ => panic!("This shouldn't compile.")
        }
    })
}

pub trait PropertyProvider{
    fn register_properties(&self, property_bank: &mut PropertyBank);
    fn get_property(&mut self, tree: &LayoutTree, elem_id: LayoutElemID, name: String) -> Option<DefaultNumericType>{
        None
    }
    fn set_property(&mut self, tree: &LayoutTree, elem_id: LayoutElemID, name: String, new_value: DefaultNumericType){
        unimplemented!();
    }
}

pub struct PropertyBank{
    properties: HashMap<String, Box<handle_function>>
}

impl PropertyBank{
    pub fn new() -> PropertyBank{
        PropertyBank{
            properties: HashMap::new()
        }
    }

    pub fn address_property<T>(&mut self, name: String, handle: T) where T: Fn(&mut LayoutElement, Option<DefaultNumericType>) -> Option<&ToPrimitive> + 'static{
        self.properties.insert(name, Box::new(handle));
    }

    pub fn get_all_property_names(&self) -> Vec<&String>{
        Vec::from_iter(self.properties.keys())
    }

    pub fn get_handle(&self, handle: String) -> Option<&Box<handle_function>>{
        self.properties.get(&handle)
    }

    
}