use std::thread;
use std::time::Duration;

mod lib;

fn main() {
    let expensive_calculation = |identifier| {
        println!(
            "Could not find '{}' in cache. Running calculation...",
            identifier
        );
        thread::sleep(Duration::from_secs(2));
        identifier
    };
    let mut memoized_calculation = lib::cache_utils::memoized_closure(expensive_calculation);
    assert_eq!(memoized_calculation.value("test"), "test");
}
