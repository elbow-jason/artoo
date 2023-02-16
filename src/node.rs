use crate::{Branch, Leaf, Node16, Node256, Node4, Node48};

/// SeekKey is a fast, efficient whole-key struct that:
/// 1) keeps track of the index that the key is currently on (for finding the next node)
/// 2) has the whole key itself
/// 3) has a cache of the current byte (for speed)
#[derive(Debug, Clone, Copy)]
pub(crate) struct Seek<'a> {
    pub byte: u8,
    pub idx: usize,
    pub key: &'a [u8],
}

impl<'a> Seek<'a> {
    pub fn new(key: &'a [u8]) -> Seek {
        Seek {
            byte: key[0],
            idx: 0,
            key,
        }
    }
}

pub(crate) enum Node<V> {
    None,
    Branch(Box<Branch<V>>),
    Leaf(Leaf<V>),
    BoxNode(BoxNode<V>),
    BoxNodeLeaf(BoxNode<V>, Leaf<V>),
}

impl<V> std::fmt::Debug for Node<V>
where
    V: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::None => write!(f, "Node::None"),
            Node::Leaf(leaf) => write!(f, "Node::Leaf({:?})", leaf),
            Node::BoxNode(box_node) => write!(f, "Node::BoxNode({:?})", box_node),
            Node::BoxNodeLeaf(box_node, leaf) => {
                write!(f, "Node::BoxNodeLeaf({:?}, {:?})", box_node, leaf)
            }
            Node::Branch(branch) => write!(f, "Node::Branch({:?})", branch),
        }
    }
}

impl<V> Node<V> {
    pub fn find_child<'a>(&self, seek: Seek<'a>) -> Option<&Node<V>> {
        match self {
            Node::None | Node::Leaf(_) => None,
            Node::BoxNode(bn) => bn.find_child(seek),
            Node::BoxNodeLeaf(bn, _) => bn.find_child(seek),
            Node::Branch(_) => unreachable!(),
        }
    }

    // pub fn get_leaf_mut(&mut self) -> Option<&mut Leaf<V>> {
    //     match self {
    //         Node::Leaf(leaf) | Node::BoxNodeLeaf(_, leaf) => Some(leaf),
    //         _ => None,
    //     }
    // }

    pub fn insert_in_leaf(&mut self, val: V) -> Option<V> {
        match self {
            Node::None => {
                let leaf = Leaf::new(val);
                let none_node = std::mem::replace(self, Node::Leaf(leaf));
                assert!(none_node.is_none());
                None
            }
            Node::Leaf(leaf) => Some(leaf.insert(val)),
            Node::BoxNode(bn) => {
                let leaf = Leaf::new(val);
                let owned_bn = std::mem::replace(bn, BoxNode::None);
                let new_node = Node::BoxNodeLeaf(owned_bn, leaf);
                let none_node = std::mem::replace(self, new_node);
                assert!(none_node.is_none());
                None
            }
            Node::BoxNodeLeaf(_, leaf) => Some(leaf.insert(val)),
            Node::Branch(_) => unreachable!(),
        }
    }

    // pub fn add_new_child(&mut self, byte: u8) -> &mut Node<V> {
    //     match self {
    //         Node::None => {
    //             let mut child = Node4::new();
    //             self.add_child(byte, child)

    //             | Node::Leaf(_) => None,
    //         }
    //     }
    // }

    pub fn find_child_mut<'a>(&mut self, seek: Seek<'a>) -> Option<&mut Node<V>> {
        unsafe { std::mem::transmute(self.find_child(seek)) }
    }

    pub fn is_none(&self) -> bool {
        match self {
            Node::None => true,
            Node::BoxNode(BoxNode::None) => true,
            _ => false,
        }
    }

    // pub fn is_leaf(&self) -> bool {
    //     todo!()
    // }

    // pub fn is_full(&self) -> bool {
    //     match self {
    //         Node::None | Node::Leaf(_) | Node::Branch(_) => true,
    //         Node::BoxNode(bn) => bn.is_full(),
    //         Node::BoxNodeLeaf(bn, _) => bn.is_full(),
    //     }
    // }

    // pub fn load_key(&self) -> &[u8] {
    //     todo!()
    // }

    // pub fn prefix(&self) -> Option<&[u8]> {
    //     todo!()
    // }

    // pub fn insert(&mut self, key: &[u8], leaf: Leaf<V>, depth: usize) {
    //     todo!()
    // }

    pub fn grow_if_full(&mut self) {
        match self {
            Node::None => {
                // None is always full
                let new_node4 = Node4::new();
                let new_node = Node::BoxNode(BoxNode::Node4(Box::new(new_node4)));
                let none_node = std::mem::replace(self, new_node);
                debug_assert!(none_node.is_none());
            }
            Node::Leaf(_) => {
                // Leaf is always full
                let new_node4 = Node4::new();
                // destructuring assignment!? yoooooo!
                let leaf = match std::mem::replace(self, Node::None) {
                    Node::Leaf(leaf) => leaf,
                    _ => unreachable!(),
                };
                let new_node = Node::BoxNodeLeaf(BoxNode::Node4(Box::new(new_node4)), leaf);
                let none_node = std::mem::replace(self, new_node);
                debug_assert!(none_node.is_none());
            }
            Node::BoxNode(bn) => bn.grow_if_full(),
            Node::BoxNodeLeaf(bn, _leaf) => bn.grow_if_full(),
            Node::Branch(_) => {
                unreachable!()
            }
        }
    }

    pub fn shrink(&mut self) {
        todo!()
    }

    pub fn add_child(&mut self, seek: Seek<'_>, child: Node<V>) -> &mut Node<V> {
        self.grow_if_full();
        match self {
            Node::None | Node::Leaf(_) => {
                // both None and Leaf are not possible after grow_if_full().
                unreachable!()
            }
            Node::BoxNode(bn) => bn.add_child(seek, child),
            Node::BoxNodeLeaf(bn, _) => bn.add_child(seek, child),
            _ => unreachable!(),
        }
    }

    //     pub fn add_new_child(&mut self, byte: u8) -> &mut Node<V> {
    //         match self {
    //             Node::None => {
    //                 let new_node4 = Node4::new();
    //                 let new_node = Node::BoxNode(BoxNode::Node4(Box::new(new_node4)));
    //                 let none = std::mem::replace(self, new_node);
    //                 debug_assert!(none.is_none());
    //                 self
    //             }
    //             Node::Leaf(_) => {
    //                 // turn Node::Leaf into a BoxNodeLeaf
    //                 let Node::Leaf(leaf) = std::mem::replace(self, Node::None);
    //                 let new_node4 = Node4::new();
    //                 let new_node = Node::BoxNodeLeaf(BoxNode::Node4(Box::new(new_node4)), leaf);
    //                 let none = std::mem::replace(self, &mut new_node);
    //                 debug_assert!(none.is_none());
    //                 self
    //             }
    //         }
}

impl<V> Default for Node<V> {
    fn default() -> Self {
        Node::None
    }
}

pub(crate) enum BoxNode<V> {
    // None is a temporary value used for replacing a boxed node, making changes to it, and putting it
    // back.
    None,
    Node4(Box<Node4<V>>),
    Node16(Box<Node16<V>>),
    Node48(Box<Node48<V>>),
    Node256(Box<Node256<V>>),
}

impl<V> std::fmt::Debug for BoxNode<V>
where
    V: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoxNode::None => write!(f, "BoxNode::None"),
            BoxNode::Node4(n) => write!(f, "BoxNode::Node4({:?})", n),
            BoxNode::Node16(n) => write!(f, "BoxNode::Node16({:?})", n),
            BoxNode::Node48(n) => write!(f, "BoxNode::Node48({:?})", n),
            BoxNode::Node256(n) => write!(f, "BoxNode::Node256({:?})", n),
        }
    }
}

impl<V> BoxNode<V> {
    pub fn find_child(&self, seek: Seek<'_>) -> Option<&Node<V>> {
        match self {
            BoxNode::Node4(node4) => node4.find_child(seek),
            BoxNode::Node16(node16) => node16.find_child(seek),
            BoxNode::Node48(node48) => node48.find_child(seek),
            BoxNode::Node256(node256) => node256.find_child(seek),
            _ => unreachable!(),
        }
    }

    pub fn is_full(&self) -> bool {
        match self {
            BoxNode::Node4(n) => n.is_full(),
            BoxNode::Node16(n) => n.is_full(),
            BoxNode::Node48(n) => n.is_full(),
            BoxNode::Node256(n) => n.is_full(),
            _ => unreachable!(),
        }
    }

    pub fn is_none(&self) -> bool {
        match self {
            BoxNode::None => true,
            _ => false,
        }
    }

    pub fn add_child(&mut self, seek: Seek<'_>, child: Node<V>) -> &mut Node<V> {
        match self {
            BoxNode::Node4(n) => n.add_child(seek, child),
            BoxNode::Node16(n) => n.add_child(seek, child),
            BoxNode::Node48(n) => n.add_child(seek, child),
            BoxNode::Node256(n) => n.add_child(seek, child),
            _ => unreachable!(),
        }
    }

    pub fn grow_if_full(&mut self) {
        if self.is_full() {
            self.grow()
        }
    }

    pub fn grow(&mut self) {
        // take ownership.
        let owned_self = std::mem::replace(self, BoxNode::None);
        match owned_self {
            BoxNode::Node4(node4) => {
                let none = std::mem::replace(self, BoxNode::Node16(Box::new(node4.grow())));
                assert!(none.is_none());
            }
            BoxNode::Node16(node16) => {
                let none = std::mem::replace(self, BoxNode::Node48(Box::new(node16.grow())));
                assert!(none.is_none());
            }
            BoxNode::Node48(node48) => {
                let none = std::mem::replace(self, BoxNode::Node256(Box::new(node48.grow())));
                assert!(none.is_none());
            }
            bn256 @ BoxNode::Node256(_) => {
                let none = std::mem::replace(self, bn256);
                assert!(none.is_none());
            }
            _ => unreachable!(),
        }
    }
}

// fn is_leaf<V>(node: &Node<V>) -> bool {
//     todo!()
// }

// fn leaf_matches<V>(node: &Node<V>, key: &[u8], depth: usize) -> bool {
//     todo!()
// }

// fn check_prefix<V>(node: &Node<V>, key: &[u8], depth: usize) -> usize {
//     todo!()
// }

// fn add_child<V>(node: &mut Node<V>, key_byte: u8, child: Node<V>) {
//     todo!()
// }

// fn search<'a, V>(node: &'a Node<V>, key: &[u8], mut depth: usize) -> Option<&'a Node<V>> {
//     if node.is_none() {
//         return None;
//     }

//     if is_leaf(node) {
//         if leaf_matches(node, key, depth) {
//             return Some(node);
//         }
//         return None;
//     }
//     let prefix = match node.prefix() {
//         None => return None,
//         Some(prefix) => {
//             if check_prefix(node, key, depth) != prefix.len() {
//                 return None;
//             }
//             prefix
//         }
//     };

//     depth = depth + prefix.len();
//     match node.find_child(key[depth]) {
//         Some(next) => search(next, key, depth + 1),
//         None => None,
//     }
//     // TODO: use a while loop instead of recursion.
// }

// fn insert<V>(maybe_node: &mut Option<Node<V>>, key: &[u8], leaf: Leaf<V>, mut depth: usize) {
//     match maybe_node {
//         None => {
//             // handle empty tree
//             let _ = std::mem::replace(maybe_node, Some(Node::Leaf(leaf)));
//             return;
//         }
//         Some(ref mut node) => {
//             if node.is_leaf() {
//                 // expand node
//                 let mut new_node = Node4::new();
//                 let key2 = node.load_key();
//                 // let mut i = depth;
//                 // while key[i] == key2[i] {
//                 //     new_node.prefix.push(key[i]);
//                 //     i += 1;
//                 // }
//                 // new_node.prefix_len = (i - 1) - depth;
//                 // depth = depth + new_node.prefix.len();
//                 let node_as_child_byte = key2[depth];
//                 std::mem::drop(key2);
//                 new_node.add_child(key[depth], Node::Leaf(leaf));
//                 let node_as_child = std::mem::replace(node, Node::None);
//                 new_node.add_child(node_as_child_byte, node_as_child);
//                 let nothing =
//                     std::mem::replace(node, Node::BoxNode(BoxNode::Node4(Box::new(new_node))));
//                 assert!(nothing.is_none());
//                 return;
//             }

//             let p = check_prefix(&node, key, depth);
//             if p != node.prefix().unwrap().len() {
//                 // prefix mismatch
//                 let mut new_node4 = Node4::new();
//                 new_node4.add_child(key[depth + p], Node::Leaf(leaf));
//                 let node_as_child = std::mem::replace(node, Node::None);
//                 new_node4.set_prefix_len(p);
//                 // new_node4
//                 //     .prefix
//                 //     .copy_from_slice(&node.prefix().unwrap()[0..p]);
//                 new_node4.add_child(node.prefix().unwrap()[p], node_as_child);

//                 // node.prefixLen=node.prefixLen-(p+1)
//                 // memmove(node.prefix, node.prefix+p+1, node.prefixLen);
//                 let _ = std::mem::replace(node, Node::BoxNode(BoxNode::Node4(Box::new(new_node4))));
//                 return;
//             }

//             depth = depth + node.prefix().unwrap().len();
//             match node.find_child_mut(key[depth]) {
//                 Some(next) => {
//                     // recurse
//                     next.insert(key, leaf, depth + 1)
//                 }
//                 None => {
//                     node.grow_if_full();
//                     node.add_child(key[depth], Node::Leaf(leaf));
//                 }
//             }
//         }
//     }
// }

#[test]
fn test_sizeof_node() {
    use std::mem::size_of;
    assert_eq!(size_of::<Node<i32>>(), 24);
    assert_eq!(size_of::<Node<i64>>(), 24);
    assert!(page_size::get() > size_of::<Node<i32>>());
}
