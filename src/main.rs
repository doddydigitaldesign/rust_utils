use std::thread;
use std::time::Duration;

mod lib;

fn main() {
    // Cache utils
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
    assert_eq!(memoized_calculation.value("test"), "test");
    assert_eq!(memoized_calculation.value("test"), "test");
    assert_eq!(memoized_calculation.value("test"), "test");
    assert_eq!(memoized_calculation.value("test"), "test");
    assert_eq!(memoized_calculation.value("test"), "test");

    // FP utils
    let some_data = vec![1, 2, 3, 4, 5, 6, 7];
    let mapped_data = lib::fp_utils::map(some_data, |n: &usize| -> usize {
        println!("{}", n * n);
        n * n
    });
    assert_eq!(mapped_data, [1, 4, 9, 16, 25, 36, 49]);
    let some_dirty_data = vec![1, 2, 3, 4, 5, 6, 7];
    let filtered_data = lib::fp_utils::filter(some_dirty_data, |n: &usize| -> bool {
        println!("{} % 4: {}", n, n % 4);
        n % 4 == 0
    });
    assert_eq!(filtered_data, [4]);
}
