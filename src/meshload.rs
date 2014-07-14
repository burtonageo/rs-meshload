#![crate_name = "meshload"]

#![desc = "A library for loading 3D formats"]
#![license = "MIT"]
#![crate_type = "lib"]

pub use meshload = types;
pub use meshload = importers;

mod importers;
mod types;
