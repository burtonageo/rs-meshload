use std::io::{IoResult};
use types::Mesh;

pub trait MeshImporter<T0: Float,
                       T1: Float,
                       T2: Float,
                       T3: Float> {
    fn load_mesh_file(&self, file_name: &str) -> IoResult<&Mesh<T0, T1, T2, T3>>;
}