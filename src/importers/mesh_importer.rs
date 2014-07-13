use std::io::{IoResult};
use types::{Mesh, RgbaColor, Point, Normal, UvCoord};

pub trait MeshImporter<F0: Float, VC: RgbaColor<F0>,
                       F1: Float, VP: Point<F1>,
                       F2: Float, VN: Normal<F2>,
                       F3: Float, VU: UvCoord<F3>,
                       M:  Mesh<F0, VC, F1, VP, F2, VN, F3, VU>> {
    fn load_mesh_file(&self, file_name: &str) -> IoResult<M>;
}
