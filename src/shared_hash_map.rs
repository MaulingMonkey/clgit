#![allow(dead_code)] // XXX

use crate::generic::Hash;

use std::sync::Mutex;
use std::collections::HashMap;



pub(crate) struct SharedHashMap<K, V> {
    buckets: [Mutex<HashMap<Hash<K>, V>>; 256],
}

impl<K, V> SharedHashMap<K, V> {
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

    pub fn bucket_for<'s>(&'s self, hash: &Hash<K>) -> &'s Mutex<HashMap<Hash<K>, V>> { &self.buckets[usize::from(hash.first_byte())] }

    pub fn contains_key(&self, hash: &Hash<K>) -> bool { self.bucket_for(hash).lock().unwrap().contains_key(hash) }
    pub fn insert(&self, hash: &Hash<K>, value: V) -> Option<V> { self.bucket_for(hash).lock().unwrap().insert(hash.clone(), value) }
    pub fn remove(&self, hash: &Hash<K>) -> Option<V> { self.bucket_for(hash).lock().unwrap().remove(hash) }
    pub fn len_approx(&self) -> usize { self.buckets.iter().map(|b| b.lock().unwrap().len()).sum() }
    pub fn retain(&self, mut f: impl FnMut(&Hash<K>, &mut V) -> bool) { for b in self.buckets.iter() { b.lock().unwrap().retain(|h, v| f(h, v)); } }
}

impl<K, V: Clone> SharedHashMap<K, V> {
    pub fn get_clone(&self, hash: &Hash<K>) -> Option<V> { self.bucket_for(hash).lock().unwrap().get(hash).map(|v| v.clone()) }
}

impl<K, V> Default for SharedHashMap<K, V> { fn default() -> Self { Self::new() }}
