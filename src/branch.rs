use crate::describe::{Describe, Describer};
// use crate::{Leaf, Node, Seek};
use crate::Node;

pub(crate) struct Branch<V> {
    sequence: Box<[u8]>,
    node: Node<V>,
}

impl<V> std::fmt::Debug for Branch<V>
where
    V: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Branch")
            .field("sequence", &self.sequence)
            .field("node", &self.node)
            .finish()
    }
}

impl<V> Describer for Branch<V> {
    fn describe(&self, d: &mut Describe) {
        d.push_str("Branch {");
        d.nest(|d| {
            d.push_str(&format!("seq: {:?},", self.sequence()));
            self.node.describe(d);
        });
        d.push_str("}");
    }
}

impl<V> Branch<V> {
    pub fn new(sequence: &[u8], node: Node<V>) -> Self {
        Self {
            sequence: sequence.to_vec().into_boxed_slice(),
            node,
        }
    }

    pub fn sequence(&self) -> &[u8] {
        &self.sequence
    }

    // pub fn find_child<'a>(&self, seek: Seek<'a>) -> FindChild<'a, &Node<V>> {
    //     // let seq = self.sequence();
    //     // let len = seq.len();
    //     // let tail = seek.tail();
    //     // if tail == sequence { the child is the node of the branch }
    //     // if sequence.starts_with(tail) { there is no child }
    //     // if tail.starts_with(sequence) {
    //     //     remove the sequence from the beginning of the tail by
    //     // }
    //     // panic!("Branch find child: len: {len:?}, seq: {seq:?}, tail: {tail:?}");
    //     todo!()
    // }

    // pub fn get_leaf_mut(&mut self) -> Option<&mut Leaf<V>> {
    //     self.node.get_leaf_mut()
    // }
}
