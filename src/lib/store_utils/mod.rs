use uuid;

/// Utils for data storage

/// create_guid
/// # Examples
/// ```
/// assert_eq!(, [1, 4, 9, 16, 25, 36, 49]);
///```
/// To Do:
pub fn create_guid() -> uuid::Uuid {
    uuid::Uuid::new_v4()
}

/// create_store
/// creates a redux store
pub mod store;
