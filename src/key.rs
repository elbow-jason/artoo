use std::borrow::Borrow;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;

const MEDIUM_LEN: usize = 23;
const SMALL_LIMIT: usize = MEDIUM_LEN - 1;
const SMALL_LIMIT_U8: u8 = (MEDIUM_LEN - 1) as u8;

#[derive(Clone)]
pub enum Key {
    Empty,
    Small(u8, [u8; SMALL_LIMIT]),
    Medium([u8; MEDIUM_LEN]),
    Large(Box<[u8]>),
}

impl fmt::Debug for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let slice = self.as_slice();
        let kind = match self {
            Key::Empty => "Key::from",
            Key::Small(_, _) => "Key::from",
            Key::Medium(_) => "Key::from",
            Key::Large(_) => "Key::from",
        };
        match std::str::from_utf8(slice) {
            Ok(s) => write!(f, "{}({:?})", kind, s),
            Err(_) => write!(f, "{}({:?})", kind, slice),
        }
        // write!(f, "{}({:?})", kind, slice)
    }
}

impl Key {
    fn new_medium(slice: &[u8]) -> Self {
        debug_assert!(slice.len() == MEDIUM_LEN);
        let mut v = [0; MEDIUM_LEN];
        v[..].copy_from_slice(&slice[..]);
        Key::Medium(v)
    }

    fn new_small(slice: &[u8]) -> Self {
        let mut v = [0; SMALL_LIMIT];
        let len = slice.len();
        debug_assert!(len <= SMALL_LIMIT);
        v[..len].copy_from_slice(&slice[..len]);
        Key::Small(len as u8, v)
    }

    #[inline]
    pub fn len(&self) -> usize {
        match self {
            Key::Empty => 0,
            Key::Small(len, _) => *len as usize,
            Key::Medium(_) => MEDIUM_LEN,
            Key::Large(b) => b.len(),
        }
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        match self {
            Key::Empty => &[][..],
            Key::Small(len, b) => &b[0..*len as usize],
            Key::Medium(b) => &b[..],
            Key::Large(b) => &b[..],
        }
    }

    pub fn concat(&self, byte: u8) -> Key {
        match self {
            Key::Empty => {
                let mut v = [0; SMALL_LIMIT];
                v[0] = byte;
                Key::Small(1, v)
            }
            Key::Large(b) => {
                let mut v = b.to_vec();
                v.push(byte);
                let new_b = v.into_boxed_slice();
                Key::Large(new_b)
            }
            Key::Small(len, b) if *len < 15 => {
                let mut b2 = b.clone();
                b2[*len as usize] = byte;
                Key::Small(*len + 1, b2)
            }
            Key::Small(SMALL_LIMIT_U8, b) => {
                let mut v = [0; MEDIUM_LEN];
                v[..SMALL_LIMIT].copy_from_slice(&b[..SMALL_LIMIT]);
                v[SMALL_LIMIT] = byte;
                Key::Medium(v)
            }
            Key::Medium(b) => {
                let mut v = b.to_vec();
                v.push(byte);
                let new_b = v.into_boxed_slice();
                Key::Large(new_b)
            }
            _ => unreachable!(),
        }
    }
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_slice().partial_cmp(other.as_slice())
    }
}

impl Eq for Key {}

impl Ord for Key {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_slice().cmp(other.as_slice())
    }
}

impl Borrow<[u8]> for Key {
    fn borrow(&self) -> &[u8] {
        self.as_slice()
    }
}

impl Deref for Key {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl Hash for Key {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state);
    }
}

// impl Borrow<[u8]> for Key {
//     fn borrow(&self) -> &[u8] {
//         self.as_slice()
//     }
// }

impl From<&str> for Key {
    fn from(s: &str) -> Self {
        Key::from(s.as_bytes())
    }
}

impl From<&[u8]> for Key {
    fn from(s: &[u8]) -> Self {
        match s.len() {
            0 => Key::Empty,
            l if l <= SMALL_LIMIT => Key::new_small(s),
            MEDIUM_LEN => Key::new_medium(s),
            _ => Key::Large(s.to_vec().into_boxed_slice()),
        }
    }
}

impl From<Vec<u8>> for Key {
    fn from(v: Vec<u8>) -> Key {
        match v.len() {
            0 => Key::Empty,
            l if l < SMALL_LIMIT => Key::new_small(&v[..]),
            MEDIUM_LEN => Key::new_medium(&v[..]),
            l if l > MEDIUM_LEN => Key::Large(v.into_boxed_slice()),
            _ => unreachable!(),
        }
    }
}

impl From<Box<[u8]>> for Key {
    fn from(v: Box<[u8]>) -> Key {
        Key::Large(v)
    }
}

macro_rules! from_array_for_key {
    ($sz:expr) => {
        impl From<[u8; $sz]> for Key {
            fn from(v: [u8; $sz]) -> Key {
                Key::from(&v[..])
            }
        }

        impl From<&[u8; $sz]> for Key {
            fn from(v: &[u8; $sz]) -> Key {
                Key::from(&v[..])
            }
        }

        impl From<&&[u8; $sz]> for Key {
            fn from(v: &&[u8; $sz]) -> Key {
                Key::from(&v[..])
            }
        }
    };
}

from_array_for_key!(0);
from_array_for_key!(1);
from_array_for_key!(2);
from_array_for_key!(3);
from_array_for_key!(4);
from_array_for_key!(5);
from_array_for_key!(6);
from_array_for_key!(7);
from_array_for_key!(8);
from_array_for_key!(9);
from_array_for_key!(10);
from_array_for_key!(11);
from_array_for_key!(12);
from_array_for_key!(13);
from_array_for_key!(14);
from_array_for_key!(15);
from_array_for_key!(16);
from_array_for_key!(17);
from_array_for_key!(18);
from_array_for_key!(19);
from_array_for_key!(20);
from_array_for_key!(21);
from_array_for_key!(22);
from_array_for_key!(23);
from_array_for_key!(24);
from_array_for_key!(25);
from_array_for_key!(26);
from_array_for_key!(27);
from_array_for_key!(28);
from_array_for_key!(29);
from_array_for_key!(30);
from_array_for_key!(31);
from_array_for_key!(32);

#[test]
fn test_key_sizeof_is_24() {
    assert_eq!(std::mem::size_of::<Key>(), 24);
}

// #[test]
// fn test_key_as_slice_works_for_empty() {
//     assert_eq!(Key::Empty.as_slice(), &b""[..]);
// }

// #[test]
// fn keys_can_be_looked_up_by_slice_in_a_hashmap() {
//     use hashbrown::HashMap;
//     let mut map = HashMap::new();
//     map.insert(Key::from(b"abc"), 3);
//     assert!(map.contains_key(&b"abc"[..]));
// }
