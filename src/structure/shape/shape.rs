use std::rc::Rc;

use super::{mesh::Mesh, material::Material};

pub struct ShapeEntry {
    mesh: Rc<Mesh>,
    material: Rc<Material>,
}

pub struct Shape {
    entries: Vec<ShapeEntry>,
}