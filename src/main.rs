mod cli;
mod redis;

use std::io::{self, Write};

fn parse_set(command: &[&str]) -> Result<(), String> {
    return match redis::core::set(
        String::from(*command.get(0).unwrap()),
        String::from(*command.get(1).unwrap()),
    ) {
        Ok(_) => {
            io::stdout().write_all("OK\n".as_bytes()).unwrap();
            io::stdout().flush().unwrap();
            Ok(())
        }
        Err(e) => Err(e.to_string()),
    };
}

fn parse_get(command: &[&str]) -> Result<(), String> {
    return match redis::core::get(String::from(*command.get(0).unwrap())) {
        Ok(value) => {
            match value {
                Some(result) => {
                    io::stdout().write_all(result.as_bytes()).unwrap();
                    io::stdout().write_all("\n".as_bytes()).unwrap();
                    io::stdout().flush().unwrap();
                }
                None => {
                    io::stdout().write_all("(nil)\n".as_bytes()).unwrap();
                    io::stdout().flush().unwrap();
                }
            };
            Ok(())
        }
        Err(e) => Err(e.to_string()),
    };
}

fn parse_del(command: &[&str]) -> Result<(), String> {
    return match redis::core::delete(String::from(*command.get(0).unwrap())) {
        Ok(value) => {
            match value {
                Some(_) => {
                    io::stdout().write_all("1\n".as_bytes()).unwrap();
                    io::stdout().flush().unwrap();
                }
                None => {
                    io::stdout().write_all("0\n".as_bytes()).unwrap();
                    io::stdout().flush().unwrap();
                }
            };
            Ok(())
        }
        Err(e) => Err(e.to_string()),
    };
}

fn parse_save(_command: &[&str]) -> Result<(), String> {
    return match redis::core::save("dump.rdb".to_string()) {
        Ok(_) => {
            io::stdout().write_all("OK\n\n".as_bytes()).unwrap();
            io::stdout().flush().unwrap();
            Ok(())
        }
        Err(e) => Err(e.to_string()),
    };
}

fn main() {
    // Store the input commands
    let mut buffer = String::new();

    // Register commands
    cli::register(cli::parser::Parser {
        name: String::from("set"),
        arity: 2,
        parser: parse_set,
    })
    .unwrap();

    cli::register(cli::parser::Parser {
        name: String::from("get"),
        arity: 1,
        parser: parse_get,
    })
    .unwrap();

    cli::register(cli::parser::Parser {
        name: String::from("del"),
        arity: 1,
        parser: parse_del,
    })
    .unwrap();

    cli::register(cli::parser::Parser {
        name: String::from("save"),
        arity: 0,
        parser: parse_save,
    })
    .unwrap();

    // Attempt loading if possible
    redis::core::load("dump.rdb".to_string()).unwrap();

    // REPL
    loop {
        // Read input
        io::stdout().write_all("\n> ".as_bytes()).unwrap();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buffer).unwrap();
        let _ = buffer.pop().unwrap_or(' '); // Trim newline

        cli::parse(cli::split(&buffer));

        // Write input and clear buffer
        buffer.clear();
    }
}
