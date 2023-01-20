use std::rc::Rc;

use super::shape::mesh::Mesh;

/// A single loaded cad data asset
pub struct CADData {
    meshes: Vec<Rc<Mesh>>
}

impl CADData {
    pub fn new() -> Self {
        Self{
            meshes: Vec::new(),
        }
    }
}