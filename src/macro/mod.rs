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
