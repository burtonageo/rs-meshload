#![crate_id = "meshload"]

#![desc = "A library for loading 3D formats"]
#![license = "MIT"]
#![crate_type = "lib"]

pub use meshload = meshload_types;
pub use meshload = obj_mesh_loader;

mod meshload_types;
mod obj_mesh_loader;
