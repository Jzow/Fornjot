use std::collections::HashMap;

pub struct Nodes<Branch, Leaf> {
    map: HashMap<u32, Node<Branch, Leaf>>,
    next_id: u32,
}

impl<Branch, Leaf> Nodes<Branch, Leaf> {
    pub fn new() -> Self {
        Nodes {
            map: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn insert_leaf(&mut self, leaf: Leaf) -> GenericId {
        let id = self.next_id;
        self.next_id += 1;

        self.map
            .insert(id, Node::Leaf(LeafNode { parent: None, leaf }));

        GenericId(id)
    }

    /// Return a reference to a node
    ///
    /// This can never fail, as nodes are never removed, meaning all node ids
    /// are always valid.
    pub fn get(&self, id: impl Into<GenericId>) -> &Node<Branch, Leaf> {
        self.map.get(&id.into().0).unwrap()
    }

    /// Return a mutable reference to a node
    ///
    /// This can never fail, as nodes are never removed, meaning all node ids
    /// are always valid.
    pub fn get_mut(
        &mut self,
        id: impl Into<GenericId>,
    ) -> &mut Node<Branch, Leaf> {
        self.map.get_mut(&id.into().0).unwrap()
    }

    pub fn leafs(&self) -> impl Iterator<Item = (GenericId, &Leaf)> + '_ {
        self.map.iter().filter_map(|(&id, node)| match node {
            Node::Leaf(LeafNode { leaf, .. }) => Some((GenericId(id), leaf)),
            _ => None,
        })
    }
}

/// Identifies a node
///
/// Since nodes can only be added, never removed, a `NodeId` instance is always
/// going to be valid.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct GenericId(pub u32);

#[derive(Debug, PartialEq)]
pub enum Node<Branch, Leaf> {
    Branch(BranchNode<Branch>),
    Leaf(LeafNode<Leaf>),
}

impl<Branch, Leaf> Node<Branch, Leaf> {
    pub fn parent(&self) -> &Option<GenericId> {
        match self {
            Node::Branch(BranchNode { parent, .. }) => parent,
            Node::Leaf(LeafNode { parent, .. }) => parent,
        }
    }

    pub fn parent_mut(&mut self) -> &mut Option<GenericId> {
        match self {
            Node::Branch(BranchNode { parent, .. }) => parent,
            Node::Leaf(LeafNode { parent, .. }) => parent,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct BranchNode<T> {
    pub parent: Option<GenericId>,
    pub above: GenericId,
    pub below: GenericId,
    pub branch: T,
}

#[derive(Debug, PartialEq)]
pub struct LeafNode<T> {
    parent: Option<GenericId>,
    pub leaf: T,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Relation {
    Above,
    Below,
}

#[cfg(test)]
mod tests {
    use super::{LeafNode, Node};

    type Nodes = super::Nodes<(), u8>;

    #[test]
    fn nodes_should_insert_leafs() {
        let mut nodes = Nodes::new();

        let leaf = 5;
        let id = nodes.insert_leaf(leaf);

        let mut expected_node = Node::Leaf(LeafNode { parent: None, leaf });

        assert_eq!(nodes.get(id), &expected_node);
        assert_eq!(nodes.get_mut(id), &mut expected_node);
    }

    #[test]
    fn nodes_should_assign_new_id_when_adding_nodes() {
        let mut nodes = Nodes::new();

        let id_a = nodes.insert_leaf(5);
        let id_b = nodes.insert_leaf(8);

        assert_ne!(id_a, id_b);
    }

    #[test]
    fn nodes_should_return_all_leafs() {
        let mut nodes = Nodes::new();

        let leaf_a = 5;
        let leaf_b = 8;

        let id_a = nodes.insert_leaf(leaf_a);
        let id_b = nodes.insert_leaf(leaf_b);

        let mut saw_a = false;
        let mut saw_b = false;

        for (id, leaf) in nodes.leafs() {
            if id == id_a && leaf == &leaf_a {
                saw_a = true;
            }
            if id == id_b && leaf == &leaf_b {
                saw_b = true;
            }
        }

        assert!(saw_a);
        assert!(saw_b);
    }
}
