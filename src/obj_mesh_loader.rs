use std::io::{BufferedReader, File, IoResult};

use meshload_types::{MeshImporter, Mesh};
// use meshload_types::{VertexColor, VertexNormal, VertexPoint, VertexUvCoord};

#[path = "./meshload_types.rs"]
mod meshload_types;

pub struct ObjFileImporter;

impl ObjFileImporter {  
    fn new() -> ObjFileImporter {
        ObjFileImporter
    }
}

impl<T0: Float,
     T1: Float,
     T2: Float,
     T3: Float> MeshImporter<T0, T1, T2, T3> for ObjFileImporter {

    fn load_mesh_file(&self, file_name: &str) -> IoResult<&Mesh<T0, T1, T2, T3>>
    {
        let path = Path::new(file_name);
        let mut file = BufferedReader::new(File::open(&path));
        for line in file.lines() {
            let line_unwrap = match line {
                                  Ok(l) => l,
                                  Err(e) => return Err(e)
                              };

            println!("{}", line_unwrap);
        }

        fail!("Not yet implemented!");
    }
}

pub fn load_obj_mesh<T0: Float,
                     T1: Float,
                     T2: Float,
                     T3: Float>(file_name: &str) -> IoResult<&Mesh<T0, T1, T2, T3>> {
    ObjFileImporter::new().load_mesh_file(file_name)
}
