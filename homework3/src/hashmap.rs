use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const INITIAL_CAPACITY: usize = 16;

struct HashMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    buckets: Vec<Vec<(K, V)>>,
}

impl<K, V> HashMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    fn new() -> Self {
        HashMap {
            buckets: vec![Vec::new(); INITIAL_CAPACITY],
        }
    }

    fn insert(&mut self, key: K, value: V) {
        let index = self.get_bucket_index(&key);
        let bucket = &mut self.buckets[index];

        if let Some((_, existing_value)) = bucket.iter_mut().find(|(k, _)| *k == key) {
            *existing_value = value;
        } else {
            bucket.push((key, value));
        }
    }

    fn get(&self, key: &K) -> Option<&V> {
        let index = self.get_bucket_index(key);
        let bucket = &self.buckets[index];

        bucket.iter().find(|(k, _)| *k == *key).map(|(_, v)| v)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        let index = self.get_bucket_index(key);
        let bucket = &mut self.buckets[index];

        if let Some(position) = bucket.iter().position(|(k, _)| *k == *key) {
            Some(bucket.remove(position).1)
        } else {
            None
        }
    }

    fn get_bucket_index(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish();
        hash as usize % self.buckets.len()
    }
}



fn main() {
    let mut map = HashMap::new();
    map.insert(String::from("apple"), 10);
    map.insert(String::from("banana"), 20);
    map.insert(String::from("cherry"), 30);

    println!("{:?}", map.get(&String::from("banana"))); // Some(&20)

    let removed_value = map.remove(&String::from("apple"));
    println!("{:?}", removed_value); // Some(10)
}