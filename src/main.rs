mod cli;

use std::{
    collections::HashMap,
    io::{self, Write},
    sync::{LazyLock, Mutex},
};

// Store the database in a hashmap in memory
static DATABASE: LazyLock<Mutex<HashMap<String, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

fn parse_set(command: &Vec<&str>) -> Result<(), String> {
    if command.len() != 2 {
        return Err(String::from("Invalid command"));
    }
    return match DATABASE.lock() {
        Ok(mut database) => {
            database.insert(
                String::from(*command.get(0).unwrap()),
                String::from(*command.get(1).unwrap()),
            );
            io::stdout().write_all("OK\n\n".as_bytes()).unwrap();
            Ok(())
        }
        Err(e) => Err(e.to_string()),
    };
}

fn parse_get(command: &Vec<&str>) -> Result<(), String> {
    if command.len() != 1 {
        return Err(String::from("Invalid command"));
    }
    return match DATABASE.lock() {
        Ok(database) => {
            match database.get(*command.get(0).unwrap()) {
                Some(result) => {
                    io::stdout().write_all(result.as_bytes()).unwrap();
                    io::stdout().write_all("\n\n".as_bytes()).unwrap();
                }
                None => {
                    io::stdout().write_all("(nil)\n\n".as_bytes()).unwrap();
                }
            };
            Ok(())
        }
        Err(e) => Err(e.to_string()),
    };
}

fn parse_del(command: &Vec<&str>) -> Result<(), String> {
    if command.len() != 1 {
        return Err(String::from("Invalid command"));
    }
    return match DATABASE.lock() {
        Ok(mut database) => {
            match database.remove(*command.get(0).unwrap()) {
                Some(_) => {
                    io::stdout().write_all("1\n\n".as_bytes()).unwrap();
                }
                None => {
                    io::stdout().write_all("0\n\n".as_bytes()).unwrap();
                }
            };
            Ok(())
        }
        Err(e) => Err(e.to_string()),
    };
}

fn main() {
    // Store the input commands
    let mut buffer = String::new();

    // Register commands
    cli::parser::register(cli::parser::Parser {
        name: String::from("set"),
        parser: parse_set,
    })
    .unwrap();

    cli::parser::register(cli::parser::Parser {
        name: String::from("get"),
        parser: parse_get,
    })
    .unwrap();

    cli::parser::register(cli::parser::Parser {
        name: String::from("del"),
        parser: parse_del,
    })
    .unwrap();

    // REPL
    loop {
        // Read input
        io::stdout().write_all("> ".as_bytes()).unwrap();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buffer).unwrap();
        let _ = buffer.pop().unwrap_or(' '); // Make sure to trim newline

        cli::parser::parse(buffer.split(' ').collect());

        // Write input and clear buffer
        buffer.clear();
    }
}
