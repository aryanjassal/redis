use std::{
    io::{self, Write},
    sync::{LazyLock, Mutex},
};

pub struct Parser {
    pub name: String,
    pub parser: fn(&Vec<&str>) -> Result<(), String>,
}

static REGISTRY: LazyLock<Mutex<Vec<Parser>>> = LazyLock::new(|| Mutex::new(Vec::new()));

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
