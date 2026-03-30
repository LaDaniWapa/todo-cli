# todo-cli

A simple CLI todo app built in Rust

## Usage
```bash
todo add "Buy tomatoes 🍅"
todo list
todo done 1
todo undone 1
todo delete 1
todo clear
```

## Built with
- `rusqlite` - SQLite storage
- `clap` - CLI parsing
- `thiserror` - error handling