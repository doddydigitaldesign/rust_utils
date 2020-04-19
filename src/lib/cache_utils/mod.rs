use std::collections::HashMap;
use std::hash::Hash;

/// Caches returned values from closures in a HashMap
pub struct CachedClosure<K, F> {
    computation: F,
    value: HashMap<K, K>,
}
impl<K: Eq + Hash + Copy, F: Fn(K) -> K> CachedClosure<K, F> {
    fn new(computation: F) -> CachedClosure<K, F> {
        let value: HashMap<K, K> = HashMap::new();
        CachedClosure { computation, value }
    }
    pub fn value(&mut self, arg: K) -> K {
        if self.value.contains_key(&arg) {
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
/// Memoization for closures
/// # Examples
/// ```
///     let expensive_calculation = |identifier| {
///         println!(
///             "Could not find cached value for '{}'. Running calculation...",
///             identifier
///         );
///         thread::sleep(Duration::from_secs(2));
///         identifier
///     };
///     let mut memoized_calculation = lib::cache_utils::memoized_closure(expensive_calculation);
///     assert_eq!(memoized_calculation.value("test"), "test");
///```
/// To Do:
/// 1. Generalize allowed types of F
/// 2. Benchmark
/// 3. Improve API
/// 4. Modularize?
pub fn memoized_closure<K: Eq + Hash + Copy, F: Fn(K) -> K>(f: F) -> CachedClosure<K, F> {
    CachedClosure::new(f)
}
