use crate::Node;

pub(crate) struct Node256<V> {
    // pub prefix: Vec<u8>,
    pub children: [Node<V>; 256],
    pub count: u16,
}

impl<V> std::fmt::Debug for Node256<V>
where
    V: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node256")
            // .field("prefix", &self.prefix)
            .field("children", &self.children)
            .field("count", &self.count)
            .finish()
    }
}

impl<V> Node256<V> {
    pub fn new() -> Node256<V> {
        Node256 {
            // prefix: Vec::new(),
            children: array_init::array_init(|_| Node::None),
            count: 0,
        }
    }
    pub fn find_child(&self, key_i: usize, key: &[u8]) -> Option<&Node<V>> {
        self.children.get(key[key_i] as usize)
    }

    pub fn is_full(&self) -> bool {
        debug_assert!(self.count <= 256);
        self.count == 256
    }

    pub fn add_child(&mut self, key_i: usize, key: &[u8], child: Node<V>) -> &mut Node<V> {
        let byte = key[key_i];
        debug_assert!(self.count <= 256);
        debug_assert!(self.children[byte as usize].is_none());
        self.count += 1;
        let out = &mut self.children[byte as usize];
        *out = child;
        out
    }
}

pub(crate) struct Node256IntoIter<V> {
    node4: Node256<V>,
    index: usize,
}

impl<V> Node256IntoIter<V> {
    pub fn new(node4: Node256<V>) -> Node256IntoIter<V> {
        Node256IntoIter { node4, index: 0 }
    }
}

impl<V> Iterator for Node256IntoIter<V> {
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

pub(crate) struct Node256Iter<'a, V> {
    node256: &'a Node256<V>,
    index: usize,
}

impl<'a, V> Node256Iter<'a, V> {
    pub fn new(node256: &'a Node256<V>) -> Node256Iter<'a, V> {
        Node256Iter { node256, index: 0 }
    }
}

impl<'a, V> Iterator for Node256Iter<'a, V> {
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

#[test]
fn test_sizeof_node256() {
    assert!(std::mem::size_of::<Node256<u32>>() <= page_size::get());
}
