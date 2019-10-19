const INITIAL_BUCKETS = 1;

struct Bucket<K, V> {
    items: Vec<(K,V)>,
}

pub struct HashMap<K, V> {
    buckets: Vec<Bucket<K,V>>,
}


impl<K, V> HashMap<K, V> {
    pub fn new() -> Self {
        HashMap {
            buckets: Vec::new(),
        }
    }

    pub fn resize(&mut self) {
        let target_size = match self.buckets.len() {
            0 => INITIAL_BUCKETS,
            n => 2 * n,
        };

    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        key.hash() % self.buckets.len()
    }

    fn remove() {

    }

    fn contains_key() {

    }
}
