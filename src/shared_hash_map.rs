#![allow(dead_code)] // XXX

use crate::Hash;

use std::sync::Mutex;
use std::collections::HashMap;



pub(crate) struct SharedHashMap<V> {
    buckets: [Mutex<HashMap<Hash, V>>; 256],
}

impl<V> SharedHashMap<V> {
    pub fn new() -> Self {
        let hm = || Mutex::new(HashMap::new());
        Self {
            buckets: [ // 16 x 16 = 256
                hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(),
                hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(),
                hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(),
                hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(),
                hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(),
                hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(),
                hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(),
                hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(),
                hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(),
                hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(),
                hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(),
                hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(),
                hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(),
                hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(),
                hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(),
                hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(), hm(),
            ],
        }
    }

    pub fn bucket_for<'s>(&'s self, hash: &Hash) -> &'s Mutex<HashMap<Hash, V>> { &self.buckets[usize::from(hash.first_byte())] }

    pub fn contains_key(&self, hash: &Hash) -> bool { self.bucket_for(hash).lock().unwrap().contains_key(hash) }
    pub fn insert(&self, hash: &Hash, value: V) -> Option<V> { self.bucket_for(hash).lock().unwrap().insert(hash.clone(), value) }
    pub fn remove(&self, hash: &Hash) -> Option<V> { self.bucket_for(hash).lock().unwrap().remove(hash) }
    pub fn len_approx(&self) -> usize { self.buckets.iter().map(|b| b.lock().unwrap().len()).sum() }
    pub fn retain(&self, mut f: impl FnMut(&Hash, &mut V) -> bool) { for b in self.buckets.iter() { b.lock().unwrap().retain(|h, v| f(h, v)); } }
}

impl<V: Clone> SharedHashMap<V> {
    pub fn get_clone(&self, hash: &Hash) -> Option<V> { self.bucket_for(hash).lock().unwrap().get(hash).map(|v| v.clone()) }
}

impl<V> Default for SharedHashMap<V> { fn default() -> Self { Self::new() }}
