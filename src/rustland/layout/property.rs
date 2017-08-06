#![macro_use]

use std::collections::HashMap;
use std::iter::FromIterator;
use num_traits::cast::{NumCast,ToPrimitive};
use num::traits::cast;
use std::cell::RefMut;

use common::definitions::{DefaultNumericType, LayoutElemID};
use layout::LayoutTree;
use layout::element::{LayoutElement, LayoutElementProfile};

type handle_function = Fn(&mut LayoutElementProfile, Option<DefaultNumericType>) -> Option<&ToPrimitive>;

macro_rules! assist_property_handle{
    ($profile_type:ident, $profile:expr, $profile_nick:ident, $handling:block) => ({        
        match $profile{
            &mut LayoutElementProfile::$profile_type(ref mut $profile_nick) => {
                $handling
            },
            _ => panic!("Element is not of the needed type.")
        }
    })
}

macro_rules! make_property_handle{
    ($profile_type:ident, $var_type: ty, $var_name:ident) => (|profile: &mut LayoutElementProfile, new_value: Option<DefaultNumericType>| {        
        match profile{
            &mut LayoutElementProfile::$profile_type(ref mut matched_profile) => {
                if let Some(value) = new_value{
                    matched_profile.$var_name = cast(value).expect("Casting error - is the last argument type numeric?");
                }

                Some(&matched_profile.$var_name)
            },
            _ => panic!("Element is not of the needed type.") 
        }
    })
}

pub trait ElementPropertyProvider{
    fn register_properties(&self, property_bank: &mut PropertyBank);    
}

pub struct PropertyBank{
    properties: HashMap<String, Box<handle_function>>
}

impl PropertyBank{
    pub fn empty() -> PropertyBank{
        PropertyBank{
            properties: HashMap::new()
        }
    }

    pub fn address_property<T>(&mut self, name: String, handle: T) where T: Fn(&mut LayoutElementProfile, Option<DefaultNumericType>) -> Option<&ToPrimitive> + 'static{
        self.properties.insert(name, Box::new(handle));
    }

    pub fn get_all_property_names(&self) -> Vec<&String>{
        Vec::from_iter(self.properties.keys())
    }

    pub fn get_handle(&self, handle: String) -> Option<&Box<handle_function>>{
        self.properties.get(&handle)
    }
}