use uuid;

/// Utils for data storage
/// # Examples
/// ```
/// assert_eq!(, [1, 4, 9, 16, 25, 36, 49]);
///```
/// To Do:
pub fn create_guid() -> uuid::Uuid {
    uuid::Uuid::new_v4()
}
