use meshload_types::{MeshLoader, Mesh};
use meshload_types::{VertexColor, VertexNormal, VertexPoint, VertexUvCoord};

#[path = "./meshload_types.rs"]
mod meshload_types;

pub struct ObjFileLoader<T0, T1, T2, T3>;

impl<T0: Float,
     T1: Float,
     T2: Float,
     T3: Float> MeshLoader<T0, T1, T2, T3> for ObjFileLoader<T0, T1, T2, T3> {

    fn load_mesh_file(file_name: &str) -> &Mesh<T0, T1, T2, T3>
    {
        fail!("Not yet implemented!");
    }
}
