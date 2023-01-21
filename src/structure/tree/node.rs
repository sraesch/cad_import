use std::fmt::{Debug, Display};

use nalgebra_glm::Mat4;

use crate::{
    id::{IDCounter, ID},
    structure::shape::shape::Shape,
};

static ID_COUNTER: IDCounter = IDCounter::new();

/// A single node in the assembly structure of the CAD data.
pub struct Node {
    id: u64,
    label: String,
    transform: Option<Mat4>,
    shapes: Vec<Shape>,
    children: Vec<Node>,
}

impl Node {
    /// Creates a new node with the given label.
    ///
    /// # Arguments
    /// * `label` - The label of the node.
    pub fn new(label: String) -> Self {
        let id = ID_COUNTER.gen();

        Self {
            id,
            label,
            transform: None,
            shapes: Vec::new(),
            children: Vec::new(),
        }
    }

    /// Returns the id of the node
    pub fn get_id(&self) -> u64 {
        self.id
    }

    /// Returns true if the node is a leaf node.
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    /// Returns a reference onto the label of the node.
    pub fn get_label(&self) -> &str {
        &self.label
    }

    /// Adds the given node as child.
    ///
    /// # Arguments
    /// * `child` - The node to add as child.
    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }

    /// Attaches a shape to the current node.
    ///
    /// # Arguments
    /// * `shape` - The shape to attach.
    pub fn attach_shape(&mut self, shape: Shape) {
        self.shapes.push(shape);
    }

    /// Sets the given transformation for the node.
    ///
    /// # Arguments
    /// * `transform` - The transformation to set.
    pub fn set_transform(&mut self, transform: Mat4) {
        self.transform = Some(transform)
    }

    /// Returns the local transformation of the node.
    pub fn get_transform(&self) -> Option<Mat4> {
        self.transform
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Node({})[label={}, #Children={}, #Shapes={}]",
            self.id,
            self.label,
            self.children.len(),
            self.shapes.len()
        )
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let children_ids: Vec<ID> = self.children.iter().map(|c| c.get_id()).collect();
        let shape_ids: Vec<ID> = self.shapes.iter().map(|s| s.get_id()).collect();

        write!(
            f,
            "Node({})[label={}, #Children={:?}, #Shapes={:?}]",
            self.id, self.label, children_ids, shape_ids
        )
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Node {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_eq() {
        let node0 = Node::new("node".to_owned());
        let node1 = Node::new("node".to_owned());

        assert_eq!(node0, node0);
        assert_eq!(node1, node1);
        assert_ne!(node0, node1);
    }
}
