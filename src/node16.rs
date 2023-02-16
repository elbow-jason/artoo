use crate::{Node, Node48, Seek};

use std::cmp::Ordering;

pub(crate) struct Node16<V> {
    // pub prefix: Vec<u8>,
    pub key: [u8; 16],
    pub children: [Node<V>; 16],
    pub count: u8,
}

impl<V> std::fmt::Debug for Node16<V>
where
    V: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node16")
            // .field("prefix", &self.prefix)
            .field("key", &self.key)
            .field("children", &self.children)
            .field("count", &self.count)
            .finish()
    }
}

impl<V> Node16<V> {
    pub fn new() -> Node16<V> {
        Node16 {
            // prefix: vec![],
            key: [0; 16],
            children: array_init::array_init(|_| Node::None),
            count: 0,
        }
    }

    pub fn is_full(&self) -> bool {
        debug_assert!(self.count <= 16);
        self.count == 16
    }

    pub fn find_child(&self, seek: Seek<'_>) -> Option<&Node<V>> {
        // TODO: simd

        // if node.type==Node16 // SSE comparison
        // key=_mm_set1_epi8(byte)
        // cmp=_mm_cmpeq_epi8(key, node.key)
        // mask=(1<<node.count)-1
        // bitfield=_mm_movemask_epi8(cmp)&mask
        // if bitfield
        // return node.child[ctz(bitfield)]
        // else
        // return NULL

        match (&self.key[..self.count as usize]).binary_search_by(|probe| probe.cmp(&seek.byte)) {
            Ok(index) => Some(&self.children[index]),
            Err(_) => None,
        }
    }

    pub fn add_child(&mut self, seek: Seek<'_>, child: Node<V>) -> &mut Node<V> {
        match (&self.key[..self.count as usize]).binary_search_by(|probe| probe.cmp(&seek.byte)) {
            Ok(index) => {
                panic!("Node16::add_child: child already exists {seek:?}");
            }
            Err(index) => {
                self.move_items_right_of(index);
                self.key[index] = seek.byte;
                self.children[index] = child;
                self.count += 1;
                &mut self.children[index]
            }
        }
    }

    fn move_items_right_of(&mut self, index: usize) {
        // .rev() - start at the rightmost i and move leftward
        for i in (index..self.count as usize).rev() {
            swap_unchecked(&mut self.key, i, i + 1);
            swap_unchecked(&mut self.children, i, i + 1);
        }
    }

    pub fn grow(mut self) -> Node48<V> {
        debug_assert!(self.count == 16);
        let mut node48 = Node48::new();
        // std::mem::swap(&mut self.prefix, &mut node48.prefix);
        for (i, k) in (&self.key[..self.count as usize]).into_iter().enumerate() {
            node48.children_index[*k as usize] = (i + 1) as u8;
            std::mem::swap(&mut node48.children[i], &mut self.children[i]);
            node48.count = self.count;
        }
        node48
    }
}

#[inline(always)]
fn swap<T>(x: &mut [T], i: usize, j: usize) {
    let (lo, hi) = match i.cmp(&j) {
        // no swapping necessary
        Ordering::Equal => return,

        // get the smallest and largest of the two indices
        Ordering::Less => (i, j),
        Ordering::Greater => (j, i),
    };
    if x.len() <= hi {
        return;
    }
    swap_unchecked(x, lo, hi)
}

#[inline(always)]
fn swap_unchecked<T>(x: &mut [T], lo: usize, hi: usize) {
    let (init, tail) = x.split_at_mut(hi);
    std::mem::swap(&mut init[lo], &mut tail[0]);
}

pub(crate) struct Node16IntoIter<V> {
    node16: Node16<V>,
    index: usize,
}
pub(crate) struct Node16Iter<'a, V> {
    node16: &'a Node16<V>,
    index: usize,
}
