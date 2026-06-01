use std::{
    collections::HashMap,
    io::{self, Write},
};

#[derive(PartialEq)]
enum Command {
    Set,
    Get,
    Del,
}

#[derive(PartialEq)]
enum TokenType {
    Command,
    Value,
}

#[derive(PartialEq)]
struct TokenValue {
    command: Option<Command>,
    value: Option<String>,
}

#[derive(PartialEq)]
struct Token {
    token_type: TokenType,
    token_value: TokenValue,
}

const TOKEN_COMMAND_SET: Token = Token {
    token_type: TokenType::Command,
    token_value: TokenValue {
        command: Some(Command::Set),
        value: None,
    },
};

const TOKEN_COMMAND_GET: Token = Token {
    token_type: TokenType::Command,
    token_value: TokenValue {
        command: Some(Command::Get),
        value: None,
    },
};

const TOKEN_COMMAND_DEL: Token = Token {
    token_type: TokenType::Command,
    token_value: TokenValue {
        command: Some(Command::Del),
        value: None,
    },
};

fn tokenise(input: Vec<&str>, output: &mut Vec<Token>, idx: usize) {
    // Early return
    if idx >= input.len() {
        return;
    };

    // Conditional matching
    match input[idx] {
        "SET" => {
            output.push(TOKEN_COMMAND_SET);
            tokenise(input, output, idx + 1);
        }
        "GET" => {
            output.push(TOKEN_COMMAND_GET);
            tokenise(input, output, idx + 1);
        }
        "DEL" => {
            output.push(TOKEN_COMMAND_DEL);
            tokenise(input, output, idx + 1);
        }
        str => {
            output.push(Token {
                token_type: TokenType::Value,
                token_value: TokenValue {
                    command: None,
                    value: Some(str.to_string()),
                },
            });
            tokenise(input, output, idx + 1);
        }
    }
}

fn parse(tokens: &Vec<Token>) -> Option<Command> {
    match tokens[0] {
        TOKEN_COMMAND_SET => {
            // Make sure the next two tokens are values, and there are no more tokens after that
            if tokens.len() == 3
                && tokens.get(1).unwrap().token_type == TokenType::Value
                && tokens.get(2).unwrap().token_type == TokenType::Value
            {
                return Some(Command::Set);
            }
            return None;
        }
        TOKEN_COMMAND_GET => {
            // Make sure the next token is a value
            if tokens.len() == 2 && tokens.get(1).unwrap().token_type == TokenType::Value {
                return Some(Command::Get);
            }
            return None;
        }
        TOKEN_COMMAND_DEL => {
            // Make sure the next token is a value
            if tokens.len() == 2 && tokens.get(1).unwrap().token_type == TokenType::Value {
                return Some(Command::Del);
            }
            return None;
        }
        _ => return None,
    };
}

fn main() {
    // Store the input commands and tokens
    let mut buffer = String::new();
    let mut tokens: Vec<Token> = Vec::new();

    // Store the database in a hashmap in memory
    let mut database: HashMap<String, String> = HashMap::new();

    // REPL
    loop {
        // Read input
        io::stdout().write_all("> ".as_bytes()).unwrap();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buffer).unwrap();
        let _ = buffer.pop().unwrap_or(' '); // Make sure to trim newline

        // Tokenise input
        tokenise(buffer.split(' ').collect(), &mut tokens, 0);

        // Parse tokens
        match parse(&tokens) {
            Some(Command::Set) => {
                database.insert(
                    tokens.get(1).unwrap().token_value.value.clone().unwrap(),
                    tokens.get(2).unwrap().token_value.value.clone().unwrap(),
                );
                io::stdout().write_all("OK\n\n".as_bytes()).unwrap();
            }
            Some(Command::Get) => {
                match database.get(&tokens.get(1).unwrap().token_value.value.clone().unwrap()) {
                    Some(result) => {
                        io::stdout().write_all(result.as_bytes()).unwrap();
                        io::stdout().write_all("\n\n".as_bytes()).unwrap();
                    }
                    None => {
                        io::stdout().write_all("(nil)\n\n".as_bytes()).unwrap();
                    }
                }
            }
            Some(Command::Del) => {
                let value =
                    database.remove(&tokens.get(1).unwrap().token_value.value.clone().unwrap());
                match value {
                    Some(_) => {
                        io::stdout().write_all("1\n\n".as_bytes()).unwrap();
                    }
                    None => {
                        io::stdout().write_all("0\n\n".as_bytes()).unwrap();
                    }
                }
            }
            None => {
                io::stdout()
                    .write_all("Invalid command: ".as_bytes())
                    .unwrap();
                io::stdout().write_all(buffer.as_bytes()).unwrap();
                io::stdout().write_all("\n".as_bytes()).unwrap();
            }
        }

        // Write input and clear buffer
        buffer.clear();
        tokens.clear();
    }
}
