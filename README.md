# 📝 NoteCLI

A simple, fast, and portable command-line notes app written in Rust 🦀  
Built with clean code, CI/CD, and production-ready Docker support.

---

## 📦 Features

- Add, list, view, delete, and search notes
- Persistent storage via local `notes.json`
- Fast and safe thanks to Rust
- Fully tested, linted, and auto-formatted
- Containerized with Docker (multi-stage build)

---

### 🔧 Prerequisites

- Rust (`cargo`)
- [Docker](https://www.docker.com/) (optional, for container use)

---

### 🧪 Running Locally (with Cargo)

```bash
cargo run -- new "My first note"
cargo run -- list
cargo run -- view 1
cargo run -- delete 1
cargo run -- search rust
```
---

### 🐳 Running with Docker

Build the image:
```bash
docker build -t notecli .
```
Run commands:
```bash
docker run --rm -v $(pwd)/notes.json:/app/notes.json notecli new "Dockerized note"
docker run --rm -v $(pwd)/notes.json:/app/notes.json notecli list
```

### 🛠 Developer Info

#### Format code:
```bash
cargo fmt
```
#### Lint with clippy:
```bash
cargo clippy
```
#### Run tests:
```bash
cargo test
```
#### CI/CD
GitHub Actions workflow: runs format check, linting, and tests on every push

### 🧱 Tech Stack

- Language: Rust
- CLI Parser: clap
- JSON Handling: serde, serde_json
- CI: GitHub Actions
- Docker (multi-stage build)

### 📂 Project Structure
```bash
src/
├── main.rs      # CLI command parsing
├── note.rs      # Note struct and file logic
```

### Author

Built by **Deniz** for learning and future teams.