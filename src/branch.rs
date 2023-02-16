use crate::Node;

pub(crate) struct Branch<V> {
    bytes: Box<[u8]>,
    node: Node<V>,
}

impl<V> std::fmt::Debug for Branch<V>
where
    V: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Branch")
            .field("bytes", &self.bytes)
            .field("node", &self.node)
            .finish()
    }
}
