use std::{
    collections::HashMap,
    fs::File,
    io::Write,
    os::unix::fs::FileExt,
    sync::{LazyLock, Mutex},
};

pub type RedisKey = String;
pub type RedisValue = String;
pub type RedisResult<T> = Result<T, String>;

const DUMP_MAGIC: &str = "RSTREDIS";
const DUMP_V1: &str = "V1\0\0\0\0\0\0";

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

fn serialise_value(value: &RedisValue) -> RedisResult<(usize, &[u8])> {
    Ok((value.as_bytes().len(), value.as_bytes()))
}

pub fn save(path: String) -> RedisResult<()> {
    let mut db = match File::create(path) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };

    // Write the file header to disk.
    // [MAGIC]: RSTREDIS
    // [VER]: V1\0\0\0\0\0\0
    // Total: 16 bytes

    db.write_all(DUMP_MAGIC.as_bytes()).unwrap();
    db.write_all(DUMP_V1.as_bytes()).unwrap();

    // Write all entries to disk.
    // Iterate over all entries and store them sequentially.
    // [FLAGS]: 0b00000111 (first three bytes for the type, rest unused)
    // [KEY_LEN]: u64
    // [VAL_LEN]: u64
    // [KEY]: dynamic length data
    // [VAL]: dynamic length data
    // Note: only 3 bits are used in the flags. 0 means string, while other
    // states are reserved for now.

    let database = match DATABASE.lock() {
        Ok(database) => database,
        Err(e) => return Err(e.to_string()),
    };

    for (key, value) in database.iter() {
        let (len, val) = serialise_value(value).unwrap();
        db.write_all(&(0b00000000u8).to_le_bytes()).unwrap();
        db.write_all(&key.as_bytes().len().to_le_bytes()).unwrap();
        db.write_all(&len.to_le_bytes()).unwrap();
        db.write_all(key.as_bytes()).unwrap();
        db.write_all(val).unwrap();
    }

    // Write 0xff as a sentinel value
    db.write_all(&(255u8).to_le_bytes()).unwrap();

    // Ensure data is written to disk
    return match db.sync_data() {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    };
}

fn read_array_at<const N: usize>(db: &std::fs::File, cursor: &mut u64) -> RedisResult<[u8; N]> {
    let mut buf = [0u8; N];
    let len = db.read_at(&mut buf, *cursor).map_err(|e| e.to_string())?;

    if len != N {
        return Err(format!("Invalid read length: expected {N}, got {len}"));
    }

    *cursor += N as u64;
    Ok(buf)
}

fn read_vec_at(db: &std::fs::File, cursor: &mut u64, len: usize) -> RedisResult<Vec<u8>> {
    let mut buf = vec![0u8; len];
    let read = db.read_at(&mut buf, *cursor).map_err(|e| e.to_string())?;

    if read != len {
        return Err(format!(
            "Invalid read length: expected {}, got {}",
            len, read
        ));
    }

    *cursor += len as u64;
    Ok(buf)
}

pub fn load(path: String) -> RedisResult<()> {
    // Try loading the dump. If the dump doesn't exist then exit early.
    let db = match File::open(path) {
        Ok(v) => v,
        Err(_) => return Ok(()),
    };

    let mut cursor: u64 = 0;

    // Read magic
    let magic = read_array_at::<8>(&db, &mut cursor)?;
    if magic != DUMP_MAGIC.as_bytes() {
        return Err("Invalid magic".to_string());
    }

    // Read version
    let version = read_array_at::<8>(&db, &mut cursor)?;
    if version != DUMP_V1.as_bytes() {
        return Err("Invalid version".to_string());
    }

    // Acquire db
    let mut database = match DATABASE.lock() {
        Ok(database) => database,
        Err(e) => return Err(e.to_string()),
    };

    loop {
        // Check for sentinel value
        let flags = read_array_at::<1>(&db, &mut cursor)?[0];
        if flags == 0xff {
            break;
        };

        // Flags currently do nothing, so they can be ignored.

        // Read key and value lengths
        let key_len = u64::from_le_bytes(read_array_at::<8>(&db, &mut cursor)?);
        let value_len = u64::from_le_bytes(read_array_at::<8>(&db, &mut cursor)?);

        // Read key and value
        let key = String::from_utf8(read_vec_at(&db, &mut cursor, key_len as usize)?).unwrap();
        let value = String::from_utf8(read_vec_at(&db, &mut cursor, value_len as usize)?).unwrap();

        // Insert into db
        database.insert(key, value);
    }

    Ok(())
}
