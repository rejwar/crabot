use std::io::{self, Write};

fn get_response(input: &str) -> &str {
    let input = input.trim().to_lowercase();
    let input = input.as_str();

    match input {
        s if s.contains("hello") || s.contains("hi") || s.contains("hey") => {
            "Hey there! How's it going? 😊"
        }
        s if s.contains("how are you") || s.contains("how do you do") => {
            "I'm doing great, thanks for asking! How about you?"
        }
        s if s.contains("name") || s.contains("who are you") => {
            "I'm Rusty 🤖 — a tiny chatbot built with Rust!"
        }
        s if s.contains("time") || s.contains("date") => {
            "I can't check the clock 😅 but your taskbar can!"
        }
        s if s.contains("joke") || s.contains("funny") || s.contains("laugh") => {
            "🤣 Why does Rust have no bugs? Because the borrow checker catches them all!"
        }
        s if s.contains("thanks") || s.contains("thank you") => "You're welcome! Happy to help! 😄",
        s if s.contains("bye") || s.contains("goodbye") || s.contains("see you") => {
            "Goodbye! See you next time! 👋"
        }
        s if s.contains("help") => {
            "I can chat about simple things! Try: hello, name, joke, time, bye"
        }
        _ => "I didn't quite get that 🤔 Type 'help' to see what I can do!",
    }
}

fn main() {
    println!("╔══════════════════════════════════════╗");
    println!("║      🤖 Rusty Chatbot — v1.0         ║");
    println!("║      Type 'quit' to exit             ║");
    println!("╚══════════════════════════════════════╝");
    println!();

    loop {
        print!("You ➤ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let trimmed = input.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.eq_ignore_ascii_case("quit") || trimmed.eq_ignore_ascii_case("exit") {
            println!("Rusty ➤ Bye bye! Come back soon! 👋\n");
            break;
        }

        let response = get_response(trimmed);
        println!("Rusty ➤ {}\n", response);
    }
}
