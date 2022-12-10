use std::{collections::HashMap, fmt::Debug};

use uuid::Uuid;

#[derive(Debug)]
pub struct Tree<T: Debug> {
    pub nodes: HashMap<Uuid, Node<T>>,
}

impl<T: Debug> Tree<T> {
    pub fn default() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, parent: Option<NodeId>, data: T) -> NodeId {
        let new_id = Uuid::new_v4();

        self.nodes.insert(
            new_id,
            Node {
                parent,
                children: Vec::new(),
                data,
            },
        );

        if let Some(parent_id) = parent {
            self.nodes
                .get_mut(&parent_id)
                .unwrap_or_else(|| {
                    panic!(
                        "Error while inserting a node, parent doesn't exist {}",
                        parent_id
                    )
                })
                .children
                .push(new_id);
        }

        new_id
    }

    pub fn remove_node(&mut self, node_id: NodeId) {
        let node = self.nodes.remove(&node_id);
        match node {
            None => (),
            Some(node) => {
                // remove node from it's parent
                if let Some(parent_id) = node.parent {
                    if let Some(parent) = self.nodes.get_mut(&parent_id) {
                        parent.children.retain(|el| el != &node_id)
                    }
                }

                // set node's children to None
                node.children.iter().for_each(|id| {
                    if let Some(child_node) = self.nodes.get_mut(id) {
                        child_node.parent = None;
                    }
                })
            }
        }
    }

    pub fn update_node(&mut self, node_id: NodeId, update_fn: &dyn Fn(&mut Node<T>)) {
        let node = self.nodes.get_mut(&node_id).unwrap_or_else(|| {
            panic!(
                "Error while inserting a node, parent doesn't exist {}",
                node_id
            )
        });

        update_fn(node);
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn get_node(&self, node_id: &NodeId) -> Option<&Node<T>> {
        self.nodes.get(node_id)
    }
}

impl<'a, T: Debug> IntoIterator for &'a Tree<T> {
    type Item = &'a T;
    type IntoIter = TreeIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        TreeIter::from(self)
    }
}

impl<'a, T: Debug> Iterator for TreeIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.idx;
        self.idx += 1;
        let node_id = self.node_ids.get(idx)?;

        self.tree.get_node(node_id).map(|node| &node.data)
    }
}

pub struct TreeIter<'a, T: Debug> {
    tree: &'a Tree<T>,
    idx: usize,
    node_ids: Vec<NodeId>,
}

impl<'a, T: Debug> From<&'a Tree<T>> for TreeIter<'a, T> {
    fn from(t: &'a Tree<T>) -> Self {
        let ids = t.nodes.keys().map(|id| id.to_owned()).collect();
        Self {
            tree: t,
            idx: 0,
            node_ids: ids,
        }
    }
}

pub type NodeId = Uuid;

#[derive(Debug)]
pub struct Node<T: Debug> {
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub data: T,
}

#[cfg(test)]
mod tests {
    use super::Tree;

    #[test]
    fn test_creating_and_deleting_nodes() {
        let mut t = Tree::default();

        let id = t.add_node(None, 42);
        let id2 = t.add_node(Some(id), 17);
        let id3 = t.add_node(Some(id), 18);

        let id4 = t.add_node(Some(id2), 27);
        let id5 = t.add_node(Some(id2), 28);

        assert!(
            !t.is_empty(),
            "Tree should not be empty after inserting nodes"
        );

        t.remove_node(id);

        // removing some node doesn't remove it's subtree, but it makes them orphans;
        assert!(
            !t.is_empty(),
            "Tree should not be empty after only deleting the root node"
        );
        assert!(
            t.get_node(&id2).unwrap().parent.is_none(),
            "Children of removed node should have parent set to None",
        );

        vec![id2, id3, id4, id5].iter().for_each(|id| {
            t.remove_node(*id);
        });

        assert!(
            t.is_empty(),
            "Tree should be empty after all nodes were removed"
        );
    }
}
