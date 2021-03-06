
// using workaround described in
// http://stackoverflow.com/questions/20342436/rust-invoke-trait-method-on-generic-type-parameter

#[deriving(Clone)]
pub trait RgbaColor<T: Float> {
    fn new(_: Option<Self>, r: T, g: T, b: T, a: T) -> Self;

    fn get_r(&self) -> T;
    fn get_g(&self) -> T;
    fn get_b(&self) -> T;
    fn get_a(&self) -> T;
}

#[deriving(Clone)]
pub trait Normal<T: Float> {
    fn new(_: Option<Self>, x: T, y: T, z: T) -> Self;

    fn get_x(&self) -> T;
    fn get_y(&self) -> T;
    fn get_z(&self) -> T;
}

#[deriving(Clone)]
pub trait Point<T: Float> {
    fn new(_: Option<Self>, x: T, y: T, z: T, w: T) -> Self;

    fn get_x(&self) -> T;
    fn get_y(&self) -> T;
    fn get_z(&self) -> T;
    fn get_w(&self) -> T;
}

#[deriving(Clone)]
pub trait UvCoord<T: Float> {
    fn new(_: Option<Self>, u: T, v: T) -> Self;

    fn get_u(&self) -> T;
    fn get_v(&self) -> T;
}

pub trait Mesh<F0: Float, P: Point<F0>,
               F1: Float, N: Normal<F1>,
               F2: Float, U: UvCoord<F2>> {
    fn new(_: Option<Self>,
           points:    Vec<P>,
           normals:   Vec<N>,
           uv_coords: Vec<U>) -> Self;
}
/*
pub trait NodeElement { }

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
