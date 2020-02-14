use crate::number::Number;
use crate::Index;
use alloc::collections::BTreeMap;
use arborist::Tree;
use bonsai::first_leaf;
use core::marker::PhantomData;
use oof::Oof;

#[derive(Default)]
pub struct List<T: Provable, N: Number> {
    backend: BTreeMap<Index, T>,
    _t: PhantomData<T>,
    _n: PhantomData<N>,
}

pub trait Provable {
    fn to_tree(self) -> Tree;
}

impl<T: Provable, N: Number> List<T, N> {
    pub fn new() -> Self {
        Self {
            backend: BTreeMap::default(),
            _t: PhantomData,
            _n: PhantomData,
        }
    }

    pub fn get(&self, idx: Index) -> Option<&T> {
        assert!(idx < first_leaf(1, N::val() as u128));
        self.backend.get(&idx)
    }

    pub fn insert(&mut self, idx: Index, val: T) -> Option<T> {
        assert!(idx < first_leaf(1, N::val() as u128));
        self.backend.insert(idx, val)
    }

    pub fn to_proof(self) -> Oof {
        let mut tree = Tree::new();

        for (k, v) in self.backend {
            tree.insert_subtree(first_leaf(1, N::val() as u128) + k, v.to_tree());
        }

        tree.fill_subtree(1, N::val() as u32, &[0; 32]);

        Oof::from_map(tree.trim().into())
    }
}
