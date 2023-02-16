pub(crate) struct Leaf<V> {
    pub val: V,
}

impl<V> std::fmt::Debug for Leaf<V>
where
    V: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Leaf(val: {:?})", self.val)
    }
}

impl<V> Leaf<V> {
    pub fn new(val: V) -> Self {
        Leaf { val }
    }

    pub fn insert(&mut self, val: V) -> V {
        std::mem::replace(&mut self.val, val)
    }
}

// use crate::BoxNode;

// /// Using single-value leaves is the most general method,
// /// because it allows keys and values of varying length within one
// /// tree. However, because of the increased tree height, it causes
// /// one additional pointer traversal per lookup. Multi-value leaves
// /// avoid this overhead, but require all keys in a tree to have the
// /// same length. Combined pointer/value slots are efficient and
// /// allow to store keys of varying length. Therefore, this method
// /// should be used if applicable. It is particularly attractive for
// /// secondary database indexes which store tuple identifiers with
// /// the same size as pointers.
// pub(crate) enum Leaf {
//     // Single-value leaves: The values are stored using an additional leaf node type which stores one value.
//     Single(V),
//     // Multi-value leaves: The values are stored in one of four different leaf node types,
//     // which mirror the structure of inner nodes, but contain values instead of pointers.
//     Multi(BoxNode),

//     // Combined pointer/value slots: If values fit into pointers, no separate node types are necessary. Instead, each
//     // pointer storage location in an inner node can either
//     // store a pointer or a value. Values and pointers can be
//     // distinguished using one additional bit per pointer or with
//     // pointer tagging.
//     Combo(BoxNode, V),
// }

// impl Leaf {
//     fn value(&self) -> Option<&V> {
//         match self {
//             Leaf::Single(v) => Some(v),
//             Leaf::Multi(_) => None,
//             Leaf::Combo(_, v) => Some(v),
//         }
//     }

//     fn box_node(&self) -> Option<&BoxNode> {
//         match self {
//             Leaf::Single(_) => None,
//             Leaf::Multi(box_node) => Some(box_node),
//             Leaf::Combo(, v) => Some(v),
//         }
//     }
// }
