use std::io::{BufferedReader, File, IoResult};

use importers::mesh_importer::{MeshImporter};
use types::{Mesh, RgbaColor, Normal, Point, UvCoord};

pub struct ObjImporter;

impl ObjImporter {
    pub fn new() -> ObjImporter {
        ObjImporter
    }
}

impl<F0: Float, P: Point<F0>,
     F1: Float, N: Normal<F1>,
     F2: Float, U: UvCoord<F2>,
     M:  Mesh<F0, P, F1, N, F2, U>> MeshImporter<F0, P, F1, N, F2, U, M> for ObjImporter {

    fn load_mesh_file(&self, file_name: &str) -> IoResult<M> {
        unimplemented!();

        let path = Path::new(file_name);
        let mut file = BufferedReader::new(try!(File::open(&path)));

        let (mut temp_points, mut final_points): (Vec<P>, Vec<P>) = (Vec::new(), Vec::new());
        let (mut temp_normals, mut final_normals): (Vec<N>, Vec<N>) = (Vec::new(), Vec::new());
        let (mut temp_uvcoords, mut final_uvcoords): (Vec<U>, Vec<U>) = (Vec::new(), Vec::new());

        let read_vert_info = |line: String| {
            match line.as_slice().slice_to(2) {
                "v " => (),
                "vt" => (),
                "vn" => (),
                _ => ()
            };
        };

        let read_face_info = |line: String| {
            
        };

        for line in file.lines()
                        .filter(|line| line.is_ok())
                        .map(|line| line.unwrap()) {
            match line.as_slice().slice_to(1) {
                "f" => read_face_info(line),
                "v" => read_vert_info(line),
                "#" | _ => (),
            }
        }

        Ok(Mesh::new(None::<M>,
                     final_points,
                     final_normals,
                     final_uvcoords))
    }
}

pub fn load_obj_mesh<F0: Float, P: Point<F0>,
                     F1: Float, N: Normal<F1>,
                     F2: Float, U: UvCoord<F2>,
                     M:  Mesh<F0, P, F1, N, F2, U>>(file_name: &str) -> IoResult<M> {
    ObjImporter::new().load_mesh_file(file_name)
}
