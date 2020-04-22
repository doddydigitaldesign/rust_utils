use std::thread;
use std::time::Duration;

mod lib;

fn main() {
    // Cache utils
    println!("------- cache_utils::memoized_closure -------");
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
    println!("------- fp_utils::map -------");
    let some_data = vec![1, 2, 3, 4, 5, 6, 7];
    let mapped_data = lib::fp_utils::map(some_data, |n: &usize| -> usize {
        println!("{}", n * n);
        n * n
    });
    assert_eq!(mapped_data, [1, 4, 9, 16, 25, 36, 49]);
    let some_dirty_data = vec![1, 2, 3, 4, 5, 6, 7];

    println!("------- fp_utils::filter -------");
    let filtered_data = lib::fp_utils::filter(some_dirty_data, |n: &usize| -> bool {
        println!("{} % 4: {}", n, n % 4);
        n % 4 == 0
    });
    assert_eq!(filtered_data, [4]);

    // Store utils
    println!("------- store_utils::create_guid -------");
    let my_guid = lib::store_utils::create_guid();
    println!("my_guid: {}", my_guid);
    let my_second_guid = lib::store_utils::create_guid();
    println!("my_second_guid: {}", my_second_guid);

    println!("------- store_utils::create_store -------");
    #[derive(Eq, PartialEq, PartialOrd, Debug)]
    enum ActionType {
        Hello = 0,
        Goodbye = 1,
    }

    #[derive(Default)]
    struct State {
        message: &'static str,
    }

    fn reducer(_state: &State, action_type: &ActionType) -> State {
        match action_type {
            ActionType::Hello => State {
                message: "Hello there!",
            },
            ActionType::Goodbye => State {
                message: "Goodbye for now...",
            },
        }
    }
    let mut my_store = lib::store_utils::store::Store::new(reducer, State::default());
    fn my_middleware(state: &State, action: &ActionType) {};
    // my_store.apply_middleware(my_middleware);
    my_store.subscribe(|state: &State| {
        println!("State changed: {}", state.message);
    });

    my_store.dispatch(ActionType::Hello);
    my_store.dispatch(ActionType::Goodbye);
}
