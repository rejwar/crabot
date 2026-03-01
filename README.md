# 🤖 Rusty Chatbot

A simple, lightweight terminal chatbot built with **Rust**. No external dependencies — just pure Rust standard library.

## Features

- Interactive terminal-based chat interface
- Pattern-matching responses using Rust's `match` expressions
- Supports greetings, jokes, and basic conversation
- Clean ASCII-art UI with emoji support
- Zero dependencies — compiles with just `std`

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.56+ recommended)

## Getting Started

```bash
# Clone or extract the project
cd chatbot

# Build and run
cargo run
```

## Usage

Once running, type any message and press Enter. Here are some things Rusty understands:

| Command | Example Input | What Rusty Does |
|---------|--------------|-----------------|
| Greeting | `hello`, `hi`, `hey` | Greets you back |
| Status | `how are you` | Tells you how it's doing |
| Identity | `name`, `who are you` | Introduces itself |
| Humor | `joke`, `funny` | Tells a Rust joke |
| Time | `time`, `date` | Responds about time |
| Thanks | `thanks`, `thank you` | Acknowledges gratitude |
| Help | `help` | Lists available commands |
| Exit | `quit`, `exit` | Ends the chat |

## Example Session

```
╔══════════════════════════════════════╗
║      🤖 Rusty Chatbot — v1.0         ║
║      Type 'quit' to exit              ║
╚══════════════════════════════════════╝

You ➤ hello
Rusty ➤ Hey there! How's it going? 😊

You ➤ tell me a joke
Rusty ➤ 🤣 Why does Rust have no bugs? Because the borrow checker catches them all!

You ➤ quit
Rusty ➤ Bye bye! Come back soon! 👋
```

## Project Structure

```
chatbot/
├── Cargo.toml      # Project manifest
└── src/
    └── main.rs     # All source code
```

## Customization

Want to add more responses? Edit the `get_response()` function in `src/main.rs`. Just add a new pattern:

```rust
s if s.contains("your keyword") => {
    "Your custom response here!"
}
```

## License

MIT — feel free to use and modify however you like.
