pub trait VertexColor<T: Float> {
    fn new_with_rgba(r: T, g: T, b: T, a: T) -> Self;
    fn new_with_rgb(r: T, g: T, b: T)        -> Self;
}

pub trait VertexNormal<T: Float> {
    fn new_with_xyz(x: T, y: T, z: T) -> Self;
}

pub trait VertexPoint<T: Float> {
    fn new_with_xyzw(x: T, y: T, z: T, w: T) -> Self;
    fn new_with_xyz(x: T, y: T, z: T)        -> Self;
}

pub trait VertexUvCoord<T: Float> {
    fn new_with_uv(u: T, v: T) -> Self;
}

pub trait Mesh<T0: Float, T1: Float, T2: Float, T3: Float> {
    fn new(colors:    &[&VertexColor<T0>],
           points:    &[&VertexPoint<T1>],
           normals:   &[&VertexNormal<T2>],
           uv_coords: &[&VertexUvCoord<T3>],
           textures:  &[&str])               -> Self;
}
/*
pub trait SceneNode<T> {
    fn new(contents: T, parent: Option<Self>) -> Self; 

    fn is_root(&self)                         -> bool;
    fn get_root(&self)                        -> Self;
    fn get_parent(&self)                      -> Self;
    fn get_children(&self)                    -> &[Self];

    fn add_child(&mut self, child_node: Self) -> ();
}

pub trait Scene {
    fn new_with_root_node(&SceneNode<T>) -> Self;
    fn new_with_no_root_node()           -> Self;
}

pub trait SceneImporter<T0: Float,
                        T1: Float,
                        T2: Float,
                        T3: Float> {
    fn load_scene_file(file_name: &str) -> IoResult<&Scene<T0, T1, T2, T3>>;
}
*/
