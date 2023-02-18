use crate::describe::{Describe, Describer};
use crate::{Node, Node16, Seek};

// TODO: look into simd
pub(crate) struct Node4<V> {
    // pub prefix: (), // Vec<u8>,
    pub key: [u8; 4],
    pub children: [Node<V>; 4],
    pub count: u8,
}

impl<V> std::fmt::Debug for Node4<V>
where
    V: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node4")
            // .field("prefix", &self.prefix)
            .field("key", &self.key)
            .field("children", &self.children)
            .field("count", &self.count)
            .finish()
    }
}

impl<V> Describer for Node4<V> {
    fn describe(&self, d: &mut Describe) {
        d.push_str("Node4")
    }
}

impl<V> Node4<V> {
    pub fn new() -> Node4<V> {
        Node4 {
            key: [0; 4],
            children: array_init::array_init(|_| Node::None),
            count: 0,
            // prefix: (),
        }
    }
    pub fn is_full(&self) -> bool {
        debug_assert!(self.count <= 4);
        self.count == 4
    }

    pub fn find_child(&self, seek: Seek<'_>) -> Option<&Node<V>> {
        for (i, byte) in self.key[..self.count as usize].iter().enumerate() {
            if byte == &seek.byte {
                return self.children.get(i);
            }
        }
        None
    }

    pub fn add_child(&mut self, seek: Seek<'_>, child: Node<V>) -> &mut Node<V> {
        debug_assert!(self.is_full() == false);
        debug_assert!(self.find_child(seek).is_none());
        self.key[self.count as usize] = seek.byte;
        self.children[self.count as usize] = child;
        let child_mut = &mut self.children[self.count as usize];
        self.count += 1;
        child_mut
    }

    // pub fn set_prefix_len(&mut self, len: usize) {
    //     todo!()
    // }

    pub fn grow(mut self) -> Node16<V> {
        let mut node16 = Node16::<V>::new();
        // node16.prefix = self.prefix;
        node16.count = self.count;
        (&mut node16.key[..self.count as usize])
            .swap_with_slice(&mut self.key[..self.count as usize]);
        (&mut node16.children[..self.count as usize])
            .swap_with_slice(&mut self.children[..self.count as usize]);
        node16
    }

    pub fn iter(&self) -> Node4Iter<V> {
        Node4Iter::<V>::new(self)
    }

    pub fn into_iter(self) -> Node4IntoIter<V> {
        Node4IntoIter::<V>::new(self)
    }
}

pub(crate) struct Node4IntoIter<V> {
    node4: Node4<V>,
    index: usize,
}

impl<V> Node4IntoIter<V> {
    pub fn new(node4: Node4<V>) -> Node4IntoIter<V> {
        Node4IntoIter { node4, index: 0 }
    }
}

impl<V> Iterator for Node4IntoIter<V> {
    type Item = (u8, Node<V>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.node4.count as usize {
            return None;
        }
        let idx = self.node4.key[self.index];
        let child = std::mem::replace(&mut self.node4.children[self.index], Node::None);
        self.index += 1;
        Some((idx, child))
    }
}

pub(crate) struct Node4Iter<'a, V> {
    node4: &'a Node4<V>,
    index: usize,
}

impl<'a, V> Node4Iter<'a, V> {
    pub fn new(node4: &'a Node4<V>) -> Node4Iter<'a, V> {
        Node4Iter { node4, index: 0 }
    }
}

impl<'a, V> Iterator for Node4Iter<'a, V> {
    type Item = (u8, &'a Node<V>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.node4.count as usize {
            return None;
        }
        let idx = self.node4.key[self.index];
        let child = &self.node4.children[self.index];
        self.index += 1;
        Some((idx, child))
    }
}
