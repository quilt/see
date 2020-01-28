use bonsai::first_leaf;
use core::marker::PhantomData;
use number::Number;
use oof::Oof;

#[cfg(feature = "generate")]
extern crate alloc;

#[cfg(feature = "generate")]
use alloc::collections::BTreeMap;

#[cfg(feature = "generate")]
use arborist::Tree;

#[cfg(feature = "generate")]
use bonsai::subtree_index_to_general;

pub type Error = usize;
pub const OK: usize = 0;
pub const ERR: usize = 1;

type Index = u128;

#[cfg(feature = "generate")]
pub trait Provable {
    fn to_tree(self) -> Tree;
}

pub trait RefNode {
    fn new(idx: u128, backend: *mut Oof) -> Self;
}

#[cfg(feature = "generate")]
pub struct List<T: Provable, N: Number> {
    backend: BTreeMap<Index, T>,
    _t: PhantomData<T>,
    _n: PhantomData<N>,
}

#[cfg(feature = "generate")]
impl<T: Provable, N: Number> List<T, N> {
    pub fn new() -> Self {
        Self {
            backend: BTreeMap::default(),
            _t: PhantomData,
            _n: PhantomData,
        }
    }

    pub fn get(&self, idx: Index) -> Option<&T> {
        assert!(idx < 2u128.pow(N::val() as u32));
        self.backend.get(&idx)
    }

    pub fn insert(&mut self, idx: Index, val: T) -> Option<T> {
        assert!(idx < 2u128.pow(N::val() as u32));
        println!("inserting: {}", idx);
        self.backend.insert(idx, val)
    }

    pub fn to_proof(self) -> Oof {
        let mut tree = Tree::new();

        for (k, v) in self.backend {
            tree.insert_subtree(k + 2u128.pow(N::val() as u32), v.to_tree());
        }

        tree.fill_subtree(1, N::val() as u32, &[0; 32]);
        let tree = tree.trim();
        println!("{:?}", tree.keys());
        Oof::from_map(tree.into())
    }
}

pub struct RefList<T: RefNode, N: Number> {
    backend: Oof,
    _t: PhantomData<T>,
    _n: PhantomData<N>,
}

impl<T: RefNode, N: Number> RefList<T, N> {
    pub fn from_raw(bytes: &mut [u8]) -> Self {
        Self {
            backend: unsafe { Oof::from_raw(bytes.as_mut_ptr()) },
            _t: PhantomData,
            _n: PhantomData,
        }
    }

    pub fn verify(&mut self, a: &[u8; 32]) -> Error {
        let b = self.backend.root();

        if Ok(a) == b {
            OK
        } else {
            ERR
        }
    }

    pub fn get_mut(&mut self, idx: Index) -> T {
        T::new(
            first_leaf(1, N::val() as u128) + idx,
            (&mut self.backend) as *mut Oof,
        )
    }

    #[cfg(feature = "generate")]
    pub fn insert(&mut self, idx: Index, val: Oof) {
        for (k, v) in val.to_map() {
            let k = subtree_index_to_general(idx, k);
            Oof::set(&mut self.backend, k, v);
        }
    }

    pub fn begin(&mut self) {
        todo!()
    }

    pub fn commit(&mut self) {
        todo!()
    }

    pub fn rollback(&mut self) {
        todo!()
    }
}

pub mod number {
    pub trait Number {
        fn val() -> usize;
    }

    macro_rules! make_num {
        ($name:ident, $val:expr) => {
            pub struct $name;

            impl Number for $name {
                fn val() -> usize {
                    $val
                }
            }
        };
    }

    make_num!(U2, 2);
    make_num!(U3, 3);
    make_num!(U4, 4);
    make_num!(U5, 5);
}
