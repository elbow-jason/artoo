use crate::{BoxNode, Leaf, Node, Node4, Seek};

pub struct Tree<V> {
    root: Node<V>,
    count: usize,
}

impl<V> Tree<V> {
    pub fn new() -> Tree<V> {
        Tree {
            root: Node::None,
            count: 0,
        }
    }

    pub fn get(&self, key: &[u8]) -> Option<&V> {
        let mut node: *const Node<V> = &self.root as *const Node<V>;
        let mut seek = Seek::new(key);
        for (i, byte) in key.iter().enumerate() {
            seek.idx = i;
            seek.byte = *byte;
            match unsafe { &*node }.find_child(&seek) {
                Some(child) => {
                    node = child;
                    continue;
                }
                None => return None,
            }
        }
        match unsafe { &*node } {
            Node::Leaf(leaf) => Some(&leaf.val),
            Node::BoxNodeLeaf(_, leaf) => Some(&leaf.val),
            _ => None,
        }
    }

    pub fn get_mut(&mut self, key: &[u8]) -> Option<&mut V> {
        unsafe { std::mem::transmute(self.get(key)) }
    }

    pub fn insert(&mut self, key: &[u8], val: V) -> Option<V> {
        let mut node: *mut Node<V> = &mut self.root as *mut Node<V>;
        let mut seek = Seek::new(key);
        for (i, byte) in (&key[..key.len() - 1]).iter().enumerate() {
            seek.idx = i;
            seek.byte = *byte;
            match unsafe { &mut *node }.find_child_mut(&seek) {
                Some(child) => {
                    node = child;
                    continue;
                }
                None => {
                    let new_node4 = Node4::new();
                    let new_node = Node::BoxNode(BoxNode::Node4(Box::new(new_node4)));
                    node = unsafe { node.as_mut().unwrap() }.add_child(&seek, new_node);
                }
            }
        }
        let node = unsafe { &mut *node };
        seek.idx = key.len() - 1;
        seek.byte = seek.key[seek.idx];
        match node.find_child_mut(&seek) {
            Some(child) => child.insert_in_leaf(val),
            None => {
                let new_node = Node::Leaf(Leaf::new(val));
                let _ = node.add_child(&seek, new_node);
                self.count += 1;
                None
            }
        }
    }

    fn remove(&mut self, key: &[u8]) -> Option<V> {
        let mut node: *mut Node<V> = &mut self.root as *mut Node<V>;
        let mut seek = Seek::new(key);
        for (i, byte) in key.iter().enumerate() {
            seek.idx = i;
            seek.byte = *byte;
            match unsafe { &mut *node }.find_child_mut(&seek) {
                Some(child) => {
                    node = child;
                    continue;
                }
                None => {
                    return None;
                }
            }
        }
        match unsafe { &mut *node } {
            node @ Node::Leaf(_) => {
                let leaf = match std::mem::replace(node, Node::None) {
                    Node::Leaf(leaf) => leaf,
                    _ => unreachable!(),
                };
                Some(leaf.val)
            }
            node @ Node::BoxNodeLeaf(_, _) => {
                let (box_node, leaf) = match std::mem::replace(node, Node::None) {
                    Node::BoxNodeLeaf(box_node, leaf) => (box_node, leaf),
                    _ => unreachable!(),
                };
                let none_node = std::mem::replace(node, Node::BoxNode(box_node));
                debug_assert!(none_node.is_none());
                Some(leaf.val)
            }
            _ => None,
        }
        // for performance's sake - we don't need to shrink when a node is empty.
        // TODO: do we need a policy?
    }
}

#[test]
fn test_tree_insert() {
    let mut tree = Tree::<i32>::new();
    let ins1 = tree.insert(b"a", 1);
    assert_eq!(ins1, None);
    match tree.root {
        Node::BoxNode(bn) => match bn {
            BoxNode::Node4(n4) => {
                // 97 is 'a'
                assert_eq!(n4.count, 1);
                assert_eq!(n4.key, [97, 0, 0, 0]);
                assert!(n4.children[0].is_none() == false);
                assert!(n4.children[1].is_none() == true);
                assert!(n4.children[2].is_none() == true);
                assert!(n4.children[3].is_none() == true);

                let node: &Node<i32> = &n4.children[0];
                match node {
                    Node::Leaf(leaf) => {
                        assert_eq!(leaf.val, 1);
                    }
                    _ => panic!("expected Node::Leaf got: {:?}", node),
                }
            }
            got => panic!("BoxNode was not a Node4 - got: {:?}", got),
        },
        got => panic!("tree.root was not a BoxNode - got: {:?}", got),
    }
}

#[test]
fn test_tree_insert_replaces_previous_value() {
    let mut tree = Tree::<i32>::new();
    let ins1 = tree.insert(b"a", 1);
    assert_eq!(ins1, None);
    let ins2 = tree.insert(b"a", 2);
    assert_eq!(tree.count, 1);
    assert_eq!(ins2, Some(1));
}

#[test]
fn test_tree_get() {
    let mut tree = Tree::<i32>::new();
    let ins1 = tree.insert(b"a", 1);
    assert_eq!(ins1, None);
    let got1 = tree.get(b"a");
    assert_eq!(got1, Some(&1));
    let ins2 = tree.insert(b"a", 2);
    let got2 = tree.get(b"a");
    assert_eq!(ins2, Some(1));
    assert_eq!(got2, Some(&2));
}

#[cfg(test)]
fn tree_with_n_keys(n: usize) -> Tree<usize> {
    let mut tree = Tree::<usize>::new();
    for i in 0..n {
        let arr = i.to_be_bytes();
        let key = &arr[..];
        let ins = tree.insert(key, i);
        assert_eq!(ins, None);
        assert_eq!(tree.get(key), Some(&i));
    }
    tree
}

#[test]
fn test_tree_can_grow_bigger_than_4_keys() {
    let _ = tree_with_n_keys(5);
}

#[test]
fn test_tree_can_grow_bigger_than_16_keys() {
    let _ = tree_with_n_keys(17);
}

#[test]
fn test_tree_can_grow_bigger_than_48_keys() {
    let _ = tree_with_n_keys(49);
}

#[test]
fn test_tree_can_grow_bigger_than_256_keys() {
    let _ = tree_with_n_keys(257);
}

#[test]
fn test_tree_handle_100k_keys() {
    let _ = tree_with_n_keys(100_000);
}
