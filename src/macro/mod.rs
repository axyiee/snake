#[macro_export]
macro_rules! import {
    ($($name:ident),*) => {
        $(
            mod $name;
            pub use $name::*;
        )*
    };
    ($($name:tt as $alias:ident),*) => {
        $(
            #[path = $name]
            mod $alias;
            pub use $alias::*;
        )*
    }
}

#[macro_export]
macro_rules! labels {
    ($($name:ident),*) => {
        $(
            #[derive(bevy::prelude::SystemLabel, Debug, Clone, Hash, PartialEq, Eq)]
            pub struct $name;
        )*
    }
}

#[macro_export]
macro_rules! define_voxel_material {
    ($types: ident, $name: expr, $id: expr) => {
        pub struct $types;
        impl $types {
            pub const ID: u8 = $id;
            pub const NAME: &'static str = $name;
        }
    };
}

import!("ezinput.rs" as ez);
