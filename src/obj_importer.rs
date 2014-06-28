use std::io::{BufferedReader, File, IoResult};

use importer::{MeshImporter};
use types::{VertexColor, VertexNormal, VertexPoint, VertexUvCoord, Mesh};

#[path = "./importer.rs"]
mod importer;
#[path = "./types.rs"]
mod types;

pub struct ObjImporter;

impl ObjImporter {  
    fn new() -> ObjImporter {
        ObjImporter
    }
}

impl<T0: Float,
     T1: Float,
     T2: Float,
     T3: Float> MeshImporter<T0, T1, T2, T3> for ObjImporter {

    fn load_mesh_file(&self, file_name: &str) -> IoResult<&Mesh<T0, T1, T2, T3>>
    {
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

pub fn load_obj_mesh<T0: Float,
                     T1: Float,
                     T2: Float,
                     T3: Float>(file_name: &str) -> IoResult<&Mesh<T0, T1, T2, T3>> {
    ObjImporter::new().load_mesh_file(file_name)
}
