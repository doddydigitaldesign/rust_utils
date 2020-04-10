mod cacher;
use cacher::cache_utils::cached;
use std::thread;
use std::time::Duration;

fn main() {
    let mut memoized_calculation = cached(|identifier| {
        println!(
            "Could not find cached value for '{}'. Running calculation...",
            identifier
        );
        thread::sleep(Duration::from_secs(2));
        identifier
    });
    println!("value: {}", memoized_calculation.value("Hello"));
    println!("value: {}", memoized_calculation.value("Hello"));
    println!("value: {}", memoized_calculation.value("Hello again"));
    println!("value: {}", memoized_calculation.value("Hello again"));
    println!("value: {}", memoized_calculation.value("Hello again"));
    println!("value: {}", memoized_calculation.value("Hello again"));
    println!("value: {}", memoized_calculation.value("Hello again"));
    println!("value: {}", memoized_calculation.value("Hello"));
    println!("value: {}", memoized_calculation.value("Hello?"));
}
