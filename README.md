# 🦀 Rust Mini HTTP Server with Hyper

A simple HTTP server built using the [`hyper`](https://crates.io/crates/hyper) crate.  
The server accepts requests, validates them, and responds with appropriate results.  

## 📌 Features
- Handles **GET** and **POST** requests.
- **Validates incoming JSON** payloads.
- Returns **400 Bad Request** for invalid data.
- Responds with **200 OK** and echo JSON for valid requests.

## 🛠 Tech Stack
- [Rust](https://www.rust-lang.org/)
- [Hyper](https://hyper.rs/) — fast, safe HTTP implementation.
- [Serde](https://serde.rs/) — serialization & deserialization.
- [Tokio](https://tokio.rs/) — async runtime.

## 🚀 Getting Started

### 1. Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Clone the Repo
```bash
git clone https://github.com/YOUR_USERNAME/small_http_server_hyper_on_rust.git
cd small_http_server_hyper_on_rust
```

### 3. Run the Server
```bash
cargo run
```
Server runs on:
```
http://127.0.0.1:3000
```

### 4. Example Requests

Valid POST Request
```bash
curl -X POST http://127.0.0.1:3000/validate \
     -H "Content-Type: application/json" \
     -d '{"username":"alice","age":30}'
```
Response:
```json
{
  "status": "ok",
  "message": "Valid request received",
  "data": {
    "username": "alice",
    "age": 30
  }
}
```
Invalid POST Request
```bash
curl -X POST http://127.0.0.1:3000/validate \
     -H "Content-Type: application/json" \
     -d '{"username":42}'
```
Response:
```json
{
  "status": "error",
  "message": "Invalid request payload"
}
```

### 📂 Project Structure

```bash
src/
├── main.rs   # HTTP server with Hyper
├── lib.rs    # Validation logic
Cargo.toml    # Dependencies
```
