use crate::{Node, Node256, Seek};

pub(crate) struct Node48<V> {
    // pub prefix: Vec<u8>,
    pub children_index: [u8; 256],
    pub children: [Node<V>; 48],
    pub count: u8,
}

impl<V> std::fmt::Debug for Node48<V>
where
    V: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node48")
            // .field("prefix", &self.prefix)
            .field("children_index", &self.children_index)
            .field("children", &self.children)
            .field("count", &self.count)
            .finish()
    }
}

impl<V> Node48<V> {
    pub fn new() -> Node48<V> {
        Node48 {
            // prefix: Vec::new(),
            children_index: [0; 256],
            children: array_init::array_init(|_| Node::None),
            count: 0,
        }
    }
    pub fn find_child(&self, seek: &Seek<'_>) -> Option<&Node<V>> {
        match self.children_index[seek.byte as usize] {
            0 => None,
            i => self.children.get((i - 1) as usize),
        }
    }

    pub fn is_full(&self) -> bool {
        debug_assert!(self.count <= 48);
        self.count == 48
    }

    pub fn grow(mut self) -> Node256<V> {
        let mut node256 = Node256::new();
        for byte_plus_one in (&self.children_index[..self.count as usize]).iter() {
            let byte: usize = *byte_plus_one as usize - 1;
            std::mem::swap(&mut node256.children[byte], &mut self.children[byte]);
        }
        node256.count = self.count as u16;
        node256
    }

    pub fn add_child(&mut self, seek: &Seek<'_>, child: Node<V>) -> &mut Node<V> {
        let i = self.count;
        self.count += 1;
        self.children_index[seek.byte as usize] = (i + 1) as u8;
        self.children[i as usize] = child;
        &mut self.children[i as usize]
    }
}

pub(crate) struct Node48IntoIter<V> {
    node4: Node48<V>,
    index: usize,
}

impl<V> Node48IntoIter<V> {
    pub fn new(node4: Node48<V>) -> Node48IntoIter<V> {
        Node48IntoIter { node4, index: 0 }
    }
}

impl<V> Iterator for Node48IntoIter<V> {
    type Item = (u8, Node<V>);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
        // if self.index == self.node4.count {
        //     return None;
        // }
        // let idx = self.node4.key[self.index];
        // let child = std::mem::replace(&mut self.node4.children[self.index], Node::Nothing);
        // self.index += 1;
        // Some((idx, child))
    }
}

pub(crate) struct Node48Iter<'a, V> {
    node48: &'a Node48<V>,
    index: usize,
}

impl<'a, V> Node48Iter<'a, V> {
    pub fn new(node48: &'a Node48<V>) -> Node48Iter<'a, V> {
        Node48Iter { node48, index: 0 }
    }
}

impl<'a, V> Iterator for Node48Iter<'a, V> {
    type Item = (u8, &'a Node<V>);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
        // if self.index == self.node4.count {
        //     return None;
        // }
        // let idx = self.node4.key[self.index];
        // let child = &self.node4.children[self.index];
        // self.index += 1;
        // Some((idx, child))
    }
}
