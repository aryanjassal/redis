use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};

pub type RedisKey = String;
pub type RedisValue = String;
pub type RedisResult<T> = Result<T, String>;

// Store the database in a hashmap in memory
static DATABASE: LazyLock<Mutex<HashMap<RedisKey, RedisValue>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub fn set(key: RedisKey, value: RedisValue) -> RedisResult<()> {
    return match DATABASE.lock() {
        Ok(mut database) => {
            database.insert(key, value);
            Ok(())
        }
        Err(e) => Err(e.to_string()),
    };
}

pub fn get(key: RedisKey) -> RedisResult<Option<RedisValue>> {
    return match DATABASE.lock() {
        Ok(database) => Ok(match database.get(&key) {
            Some(val) => Some(val.clone()),
            None => None,
        }),
        Err(e) => Err(e.to_string()),
    };
}

pub fn delete(key: RedisKey) -> RedisResult<Option<()>> {
    return match DATABASE.lock() {
        Ok(mut database) => Ok(match database.remove(&key) {
            Some(_) => Some(()),
            None => None,
        }),
        Err(e) => Err(e.to_string()),
    };
}
