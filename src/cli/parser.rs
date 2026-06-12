use std::{
    io::{self, Write},
    sync::{LazyLock, Mutex},
};

pub struct Parser {
    pub name: String,
    pub arity: i32,
    pub parser: fn(&[&str]) -> Result<(), String>,
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
        Err(_) => Err(String::from("Failed to acquire registry\n")),
    };
}

pub fn parse(command: Vec<&str>) {
    // Remove the element we just parsed before sending it to the parser
    if command.len() < 1 {
        return;
    };
    let keyword = command[0].to_lowercase();
    let args = &command[1..];
    let mut found: bool = false;

    match REGISTRY.lock() {
        Ok(registry) => {
            for parser in registry.iter() {
                if parser.name == keyword {
                    found = true;
                    if (parser.arity > 0
                        && TryInto::<i32>::try_into(args.len()).unwrap() != parser.arity)
                        || (parser.arity < 0
                            && TryInto::<i32>::try_into(args.len()).unwrap() < parser.arity)
                    {
                        io::stdout()
                            .write_all("ERROR: invalid number of arguments\n".as_bytes())
                            .unwrap();
                        io::stdout().flush().unwrap();
                        break;
                    }
                    match (parser.parser)(&args) {
                        Ok(_) => break,
                        Err(e) => {
                            io::stdout()
                                .write_all("ERROR: Command failed: ".as_bytes())
                                .unwrap();
                            io::stdout().write_all(e.to_string().as_bytes()).unwrap();
                            io::stdout().write_all("\n".as_bytes()).unwrap();
                            io::stdout().flush().unwrap();
                        }
                    };
                    break;
                }
            }
        }
        Err(_) => {
            io::stdout()
                .write_all("ERROR: Failed to acquire registry\n".as_bytes())
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
        io::stdout().write_all("\n".as_bytes()).unwrap();
        io::stdout().flush().unwrap();
    }
}
