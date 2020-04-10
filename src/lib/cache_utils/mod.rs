use std::collections::HashMap;
use std::hash::Hash;

/// # Cacher
/// The keys of the hash map will be the arg values that are passed in,
/// and the values of the hash map will be the result of calling the closure on that key.
pub struct Cacher<K, F> {
    computation: F,
    value: HashMap<K, K>,
}
impl<K: Eq + Hash + Copy, F: Fn(K) -> K> Cacher<K, F> {
    fn new(computation: F) -> Cacher<K, F> {
        let value: HashMap<K, K> = HashMap::new();
        Cacher { computation, value }
    }
    pub fn value(&mut self, arg: K) -> K {
        if self.value.contains_key(&arg) == true {
            let v = self.value.get(&arg);
            match v {
                Some(v) => *v,
                None => {
                    self.value.insert(arg, (self.computation)(arg));
                    (self.computation)(arg)
                }
            }
        } else {
            let v = (self.computation)(arg);
            self.value.insert(arg, v);
            v
        }
    }
}
/// # cached
/// f: Fn(K) -> K
/// ### Returns
/// K: Eq + Hash + Copy
///
/// ### To Do:
/// 1. Generalize allowed types of F
/// 2. Benchmark
/// 3. Improve API
/// 4. Modularize?
pub fn cached<K: Eq + Hash + Copy, F: Fn(K) -> K>(f: F) -> Cacher<K, F> {
    let result = Cacher::new(f);
    return result;
}
