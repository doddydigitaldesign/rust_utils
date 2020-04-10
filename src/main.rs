use std::thread;
use std::time::Duration;

mod lib;

fn main() {
    let expensive_calculation = |identifier| {
        println!(
            "Could not find cached value for '{}'. Running calculation...",
            identifier
        );
        thread::sleep(Duration::from_secs(2));
        identifier
    };
    let mut memoized_calculation = lib::cache_utils::cached(expensive_calculation);
    println!("value: {}", memoized_calculation.value("Hello")); // Not memoized
    println!("value: {}", memoized_calculation.value("Hello")); // Memoized
    println!("value: {}", memoized_calculation.value("Hello again")); // Not memoized
    println!("value: {}", memoized_calculation.value("Hello again")); // Memoized
    println!("value: {}", memoized_calculation.value("Hello")); // Not memoized
    println!("value: {}", memoized_calculation.value("Hello?")); // Memoized
}
