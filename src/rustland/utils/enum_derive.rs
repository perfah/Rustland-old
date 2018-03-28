#![macro_use]

#[macro_export]
macro_rules! enum_derive_trait {
    ($enum_structure:ident, $derived_trait:ident, $derived_func:tt; $($var:ident($ty:ty)),*) => ({
        impl $derived_trait for $enum_structure {
            fn $derived_func(self, args...) {
                match self {
                    $($variant::$var(sub_structure) => sub_structure.$derived_func(),)*
                }
            }
        }
    })
}


#[macro_export]
macro_rules! my_macro(() => (42));