#![feature(portable_simd)]

mod node;
use node::{BoxNode, Node};

mod node4;
use node4::{Node4, Node4IntoIter, Node4Iter};

mod node16;
use node16::{Node16, Node16IntoIter, Node16Iter};

mod node48;
use node48::{Node48, Node48IntoIter, Node48Iter};

mod node256;
use node256::{Node256, Node256IntoIter, Node256Iter};

mod leaf;
use leaf::Leaf;

mod branch;
use branch::Branch;

mod key;
pub use key::Key;

// mod branch;
// use branch::Branch;

mod tree;
pub use tree::Tree;

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
