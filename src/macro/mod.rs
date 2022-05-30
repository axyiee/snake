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

import!("ezinput.rs" as ez);
