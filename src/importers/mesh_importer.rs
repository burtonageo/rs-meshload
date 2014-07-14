use std::io::{IoResult};
use types::{Mesh, RgbaColor, Point, Normal, UvCoord};

pub trait MeshImporter<F0: Float, P: Point<F0>,
                       F1: Float, N: Normal<F1>,
                       F2: Float, U: UvCoord<F2>,
                       M:  Mesh<F0, P, F1, N, F2, U>> {
    fn load_mesh_file(&self, file_name: &str) -> IoResult<M>;
}
