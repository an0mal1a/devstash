# DevStash

A tiny CLI snippet manager built in Rust.

DevStash was created mainly as a learning project while exploring the Rust ecosystem, ownership model, enums, pattern matching, file handling, JSON serialization and general CLI application architecture.

The idea is simple:
store useful commands, notes, JSON payloads or small snippets directly from the terminal.

---

## Why I built this

I wanted a small but real project to practice Rust outside of exercises.

Instead of another calculator or todo app, I decided to build something I would actually use daily while working on backend, scraping and automation projects.

This project helped me practice:

- Structs and enums
- Ownership & borrowing
- `Result<T, E>` error handling
- File I/O
- JSON serialization with `serde`
- CLI argument parsing
- Mutable references
- Pattern matching
- Project organization with modules

---

## Features

- Add snippets interactively
- Store commands, notes or JSON payloads
- List saved snippets
- Show snippets by ID
- Delete snippets
- Persistent local storage using JSON

---

## Example

```bash
devstash add
````

```txt
Título del snippet -> PostgreSQL backup
Contenido del snippet -> pg_dump -U postgres mydb > backup.sql
Tags de snippet -> postgres,database,backup
Tipo del snippet -> command
```

---

## Commands

```bash
devstash add
devstash add 3

devstash list
devstash list 20

devstash show 1

devstash delete 1

devstash help
```

---

## Project structure

```txt
src/
├── help_panel.rs
├── json_core.rs
├── main.rs
└── utils.rs
```

---

## Storage format

All snippets are stored locally in:

```txt
snippets.json
```

Example:

```json
[
    {
        "id": 1,
        "title": "PostgreSQL backup",
        "tags": ["postgres", "backup"],
        "content": "pg_dump -U postgres mydb > backup.sql",
        "kind": "command",
        "created_at": 1747150000
    }
]
```

---

## Tech stack

* Rust
* serde
* serde_json

---

## Future ideas

* Search snippets
* Filter by tags
* Colored terminal UI
* Clipboard support
* Import/export
* SQLite backend
* Fuzzy search
* TUI interface

---

## Run locally

```bash
cargo run -- add
```

Build release:

```bash
cargo build --release
```

---

## Notes

This project is intentionally simple.

The goal is not to create the next production-ready snippet manager, but to learn Rust by building something practical and progressively improving it.

---
