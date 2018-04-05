    #![macro_use]

use std::collections::HashMap;
use std::iter::FromIterator;
use num_traits::cast::{NumCast,ToPrimitive};
use num::traits::cast;
use num::clamp;
use std::cell::RefMut;
use std::default::Default;

use common::definitions::{DefaultNumericType, LayoutElemID};
use layout::LayoutTree;
use layout::element::{LayoutElement, LayoutElementProfile};

type handle_function = Fn(&mut LayoutElementProfile, Option<DefaultNumericType>) -> Option<&ToPrimitive>;

macro_rules! assist_property_handle {
    ($profile_type:ident, $profile:expr, $profile_nick:ident, $handling:block) => ({        
        match $profile{
            &mut LayoutElementProfile::$profile_type(ref mut $profile_nick) => {
                $handling
            },
            _ => None
        }
    })
}

macro_rules! make_property_handle {
    ($profile_type:ident, $var_type: ty, $var_name:ident) => { make_property_handle!($profile_type, $var_type, $var_name, 0f32, 0f32); } ;

    ($profile_type:ident, $var_type: ty, $var_name:ident, $min_value:expr, $max_value:expr) => (|profile: &mut LayoutElementProfile, new_value: Option<DefaultNumericType>| {        
        match profile{
            &mut LayoutElementProfile::$profile_type(ref mut matched_profile) => {
                if let Some(value) = new_value{
                    matched_profile.$var_name = cast(
                        match ($min_value, $max_value) {
                            (min, max) if min != max => clamp(value, min, max),
                            _ => value 
                        }
                    ).expect("Casting error - is the last argument type numeric?");
                }

                Some(&matched_profile.$var_name)
            },
            _ => None
        }
    });
}

pub trait ElementPropertyProvider{
    fn register_properties(&self, property_bank: &mut PropertyBank);    
}

pub struct PropertyBank{
    properties: HashMap<&'static str, Box<handle_function>>
}

impl Default for PropertyBank {
    fn default() -> PropertyBank {
        PropertyBank {
            properties: HashMap::new()
        }
    }
}

impl PropertyBank{
    pub fn empty() -> PropertyBank{
        PropertyBank{
            properties: HashMap::new()
        }
    }

    pub fn address_property<T>(&mut self, name: &'static str, handle: T) where T: Fn(&mut LayoutElementProfile, Option<DefaultNumericType>) -> Option<&ToPrimitive> + 'static{
        self.properties.insert(name, Box::new(handle));
    }

    pub fn get_all_property_names(&self) -> Vec<&'static str>{
        let mut prop_names = Vec::new();
        
        for prop_name in self.properties.keys() {
            prop_names.push(*prop_name);
        }

        prop_names
    }

    pub fn get_handle(&self, handle: &'static str) -> Option<&Box<handle_function>>{
        self.properties.get(&handle)
    }
}