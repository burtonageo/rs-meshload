use std::io::{BufferedReader, File, IoResult};

use importer::{MeshImporter};
use types::{VertexColor, VertexNormal, VertexPoint, VertexUvCoord, Mesh};

struct ObjImporter;

impl ObjImporter {  
    pub fn new() -> ObjImporter {
        ObjImporter
    }
}

impl<F0: Float,
     VC: VertexColor<F0>,
     F1: Float,
     VP: VertexPoint<F1>,
     F2: Float,
     VN: VertexNormal<F2>,
     F3: Float,
     VU: VertexUvCoord<F3>,
     M:  Mesh<F0, VC, F1, VP, F2, VN, F3, VU>> MeshImporter<F0, VC, F1, VP, F2, VN, F3, VU, M> for ObjImporter {

    fn load_mesh_file(&self, file_name: &str) -> IoResult<M> {
        let path = Path::new(file_name);
        let mut file = BufferedReader::new(File::open(&path));
        for ln in file.lines() {
            let line = match ln {
                           Ok(l)  => l,
                           Err(e) => return Err(e)
                       };

            println!("{}", line);
        }

        fail!("Not yet implemented!");
    }
}

pub fn load_obj_mesh<F0: Float,
                     VC: VertexColor<F0>,
                     F1: Float,
                     VP: VertexPoint<F1>,
                     F2: Float,
                     VN: VertexNormal<F2>,
                     F3: Float,
                     VU: VertexUvCoord<F3>,
                     M:  Mesh<F0, VC, F1, VP, F2, VN, F3, VU>>(file_name: &str) -> IoResult<M> {
    ObjImporter::new().load_mesh_file(file_name)
}
