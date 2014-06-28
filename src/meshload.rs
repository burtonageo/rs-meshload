#![crate_id = "meshload"]

#![desc = "A library for loading 3D formats"]
#![license = "MIT"]
#![crate_type = "lib"]

pub use meshload = types;
pub use meshload = importer;
pub use meshload = obj_importer;

mod importer;
mod types;
mod obj_importer;
