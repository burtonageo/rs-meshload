use std::io::{IoResult};
use types::{Mesh, VertexColor, VertexPoint, VertexNormal, VertexUvCoord};

pub trait MeshImporter<F0: Float, VC: VertexColor<F0>,
                       F1: Float, VP: VertexPoint<F1>,
                       F2: Float, VN: VertexNormal<F2>,
                       F3: Float, VU: VertexUvCoord<F3>,
                       M:  Mesh<F0, VC, F1, VP, F2, VN, F3, VU>> {
    fn load_mesh_file(&self, file_name: &str) -> IoResult<M>;
}
