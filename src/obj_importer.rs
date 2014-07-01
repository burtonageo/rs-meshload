use std::io::{BufferedReader, File, IoResult};

use importer::{MeshImporter};
use types::{Mesh, VertexColor, VertexNormal, VertexPoint, VertexUvCoord};

struct ObjImporter;

impl ObjImporter {
    pub fn new() -> ObjImporter {
        ObjImporter
    }
}

impl<F0: Float, VC: VertexColor<F0>,
     F1: Float, VP: VertexPoint<F1>,
     F2: Float, VN: VertexNormal<F2>,
     F3: Float, VU: VertexUvCoord<F3>,
     M:  Mesh<F0, VC, F1, VP, F2, VN, F3, VU>> MeshImporter<F0, VC, F1, VP, F2, VN, F3, VU, M> for ObjImporter {

    fn load_mesh_file(&self, file_name: &str) -> IoResult<M> {
        let path = Path::new(file_name);
        let mut file = BufferedReader::new(File::open(&path));

        let (mut temp_colors, mut final_colors): (&[VC], &[VC]);
        let (mut temp_points, mut final_points): (&[VP], &[VP]);
        let (mut temp_normals, mut final_normals): (&[VN], &[VN]);
        let (mut temp_uvcoords, mut final_uvcoords): (&[VU], &[VU]);

        let mut textures: &[&str];

        file.lines()
            .take_while(|line| line.is_ok())
            .map(|line| line.unwrap());

        unimplemented!();
    }
}

pub fn load_obj_mesh<F0: Float, VC: VertexColor<F0>,
                     F1: Float, VP: VertexPoint<F1>,
                     F2: Float, VN: VertexNormal<F2>,
                     F3: Float, VU: VertexUvCoord<F3>,
                     M:  Mesh<F0, VC, F1, VP, F2, VN, F3, VU>>(file_name: &str) -> IoResult<M> {
    ObjImporter::new().load_mesh_file(file_name)
}
