use std::{
    fmt::format,
    io::{self, Write},
};
fn main() {
    println!("ChatBot CLI - Type 'exit' to quit");
    let mut history: Vec<String> = Vec::new();
    loop {
        let input = prompt("You: ");
        if input.to_lowercase() == "exit" {
            println!("Goodbye!");
            break;
        }
        history.push(format!("You: {}", input));
        let response = bot_reply(&input);
        println!("Bot: {}", response);
        history.push(format!("Bot : {}", response));
    }

    println!("\nChat History: ");
    for chat in history {
        println!("{}", chat);
    }
}
fn bot_reply(message: &str) -> String {
    let msg = message.to_lowercase();
    if msg.contains("hello") {
        "Hi there!".to_string()
    } else if msg.contains("how are you") {
        "I'm just code, but I'm doing fine!".to_string()
    } else if msg.contains("rust") {
        "Rust is memory-safe and fearless!".to_string()
    } else {
        "I don't understand that yet.".to_string()
    }
}
fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
