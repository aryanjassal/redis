use std::{
    io::{self, Write},
    sync::{LazyLock, Mutex},
};

pub struct Parser {
    pub name: String,
    pub parser: fn(&Vec<&str>) -> Result<(), String>,
}

static REGISTRY: LazyLock<Mutex<Vec<Parser>>> = LazyLock::new(|| Mutex::new(Vec::new()));

pub fn split<'a>(input: &'a str) -> Vec<&'a str> {
    let mut tokens: Vec<&str> = Vec::new();
    let mut start: Option<usize> = None;
    let mut in_quotes = false;

    for (i, c) in input.char_indices() {
        match c {
            '"' => {
                in_quotes = !in_quotes;
                if start.is_none() {
                    start = Some(i);
                } else {
                    tokens.push(&input[start.unwrap() + 1..i]);
                    start = None;
                }
            }
            ' ' if !in_quotes => {
                if start.is_some() {
                    tokens.push(&input[start.unwrap()..i]);
                    start = None;
                }
            }
            _ => {
                if start.is_none() {
                    start = Some(i);
                }
            }
        }
    }

    if start.is_some() {
        tokens.push(&input[start.unwrap()..input.len()]);
    }

    // Assume no end quote means the string continues on to the end

    tokens
}

pub fn register(parser: Parser) -> Result<(), String> {
    return match REGISTRY.lock() {
        Ok(mut registry) => {
            registry.push(parser);
            Ok(())
        }
        Err(_) => Err(String::from("Failed to acquire registry")),
    };
}

pub fn parse(command: Vec<&str>) {
    // Remove the element we just parsed before sending it to the parser
    let mut child_command: Vec<&str> = command.clone();
    let keyword = child_command.remove(0).to_lowercase();
    let mut found: bool = false;

    match REGISTRY.lock() {
        Ok(registry) => {
            for parser in registry.iter() {
                if parser.name == keyword {
                    found = true;
                    match (parser.parser)(&child_command) {
                        Ok(_) => break,
                        Err(e) => {
                            io::stdout()
                                .write_all("ERROR: Command failed: ".as_bytes())
                                .unwrap();
                            io::stdout().write_all(e.to_string().as_bytes()).unwrap();
                            io::stdout().write_all("\n\n".as_bytes()).unwrap();
                            io::stdout().flush().unwrap();
                        }
                    };
                    break;
                }
            }
        }
        Err(_) => {
            io::stdout()
                .write_all("ERROR: Failed to acquire registry\n\n".as_bytes())
                .unwrap();
            io::stdout().flush().unwrap();
        }
    };

    if !found {
        io::stdout()
            .write_all("ERROR: Invalid command: ".as_bytes())
            .unwrap();
        io::stdout()
            .write_all(&command.join(" ").into_bytes())
            .unwrap();
        io::stdout().write_all("\n\n".as_bytes()).unwrap();
        io::stdout().flush().unwrap();
    }
}
