# Redis

This is my attempt at making an application with a feature set similar to Redis.

The goal is to build a simplified Redis clone in Rust to learn fundamentals. The
project will be implemented incrementally, beginning with a local command-line
application and later evolving into a networked server.

## Overview

- [Core Database](#core-database)
- [Persistence](#persistence)
- [Expiration](#expiration)
- [TCP Server](#tcp-server)
- [Redis Protocol](#redis-protocol)
- [Capability Extension](#capability-extension)

### Core Database

Phase 1 of implementation if the core protocol for an in-memory key-value store.
The database only exists while the program is running. No persistence required.
Only the following commands are supported.

#### SET

Store a value under a key.

Input: `SET name bob`

Expected Result: `OK`

#### GET

Retrieve a value by key.

Input: `GET name`

Expected Result: `bob`

If key does not exist: `(nil)`

#### DEL

Delete a key.

Input: `DEL name`

Expected Result: `1`

If key does not exist: `0`

#### Technical Requirements

- **Storage Layer**: Create a database component responsible for storing and
  managing key-value pairs. Implementation should use an in-memory hash map.
- **Command Parser**: Converts user input into structured commands.
- **Command Representation**: Represent commands using an enum. Future command
  types will be added later.
- **Executor**: Execute parsed commands against the database.
- **REPL**: Implement an interactive command loop, like the `python` or `node`
  interactive shell.

### Persistence

Persist database contents to disk. Database contents must survive application
restarts. Serialisation format may be JSON, or a specialised binary format as
needed.

#### SAVE

Writes current database state to a file.

Input: `SAVE`

Expected Result: `OK`

On error: `ERR: {error message}`

#### LOAD

Loads database state from a file.

Input: `LOAD`

Expected Result: `OK`

On error: `ERR: {error message}`

### Expiration

Support expiring keys. Expired keys should not be returned by `GET`, and should
be eventually removed from storage.

#### EX

Set expiry on creation: `SET name bob EX 60`

> Stores value and expires it after 60 seconds.

#### TTL

Returns remaining lifetime.

Input: `TTL name`

Expected Result: `60` (the actual TTL)

Invalid key or expired key: `(nil)`

No expiry: `0`

#### Expire

Assigns expiration to an existing key.

Input: `EXPIRE name 60`

Expected Result: `OK`

Invalid key or expired key: `(nil)`

### TCP Server

Move from local CLI interaction to network communication. This stage must create
a TCP server that listens on a port and accepts connections to then read and
execute commands and return results. Must support concurrent clients
simultaneously to share the same database by using synchronisation primitives.

### Redis Protocol

Implement a simplified version of Redis’ RESP protocol. This should aim to
maintain compatibility with simple Redis clients where possible.

<!-- prettier-ignore-start -->
> [!NOTE] 
> I haven't researched RESP yet, so this section needs to be updated
> according to the protocol specifications.
<!-- prettier-ignore-end -->

### Capability Extension

Support more than string values, like storing lists, hashes, or sets as values.
The database should support storing multiple value types under different keys.

<!-- prettier-ignore-start -->
> [!NOTE]
> I haven't researched this yet, so this section needs to be updated according 
> to the protocol specifications.
<!-- prettier-ignore-end -->

- **List operations**: `LPUSH`, `RPUSH`, `LPOP`
- **Hashes**: `HSET`, `HGET`
- **Sets**: `SADD`, `SMEMBERS`
