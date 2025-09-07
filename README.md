# 📡 Rust Websocket Broadcast Server

A simple WebSocket-based broadcast server written in Rust using `tokio` and `tokio-tungstenite`.  
Clients can connect and send messages, which are then broadcast to all connected clients in real-time.

---

## 🚀 Features

- Start a server that accepts WebSocket connections.
- Clients can send messages to the server.
- Server broadcasts each message to **all connected clients**.
- Message timestamps are included in server logs.

---

## 🛠️ Dependencies

This project uses:

```toml
[dependencies]
tokio = { version = "1.45.1", features = ["full"] }
tokio-tungstenite = "0.26.2"
tungstenite = "0.26.2"
futures = "0.3.31"
clap = { version = "4.5.39", features = ["derive"] }
serde = { version = "1.0.219", features = ["derive"] }
serde_json = "1.0.140"
url = "2.5.4"
anyhow = "1.0.98"
chrono = "0.4.41"
```

---

## 📦 Build Instructions

```bash
git clone <this-repo>
cd broadcast_server
cargo build --release
```

---

## 🧑‍💻 Usage

### Start the Server

```bash
cargo run start --port 9001
```

This will start the server on `ws://127.0.0.1:9001` (default port).

### Start a Client

```bash
cargo run connect --addr 127.0.0.1:9001
```

Now you can type messages in the terminal and send them. All other clients connected to the server will receive the broadcast.

---

## 📁 Project Structure

```
broadcast_server/
├── src/
│   ├── main.rs          # CLI entry point
│   ├── server.rs        # WebSocket server logic
│   ├── client.rs        # WebSocket client logic
│   ├── message.rs       # Message structure definitions
│   └── utils.rs         # Timestamp utility
├── Cargo.toml
└── Readme.MD
```

---

## 📷 Example

Terminal 1 (Server):

```
🚀 Server running on ws://127.0.0.1:9001
[14:05:23] 📨 Hello from Client 1
[14:05:30] 📨 Hello from Client 2
```

Terminal 2 (Client 1):

```
🔌 Connecting to server at ws://127.0.0.1:9001
Hello from Client 1
[14:05:23] 📨 Hello from Client 1
[14:05:30] 📨 Hello from Client 2
```

Terminal 3 (Client 2):

```
🔌 Connecting to server at ws://127.0.0.1:9001
[14:05:23] 📨 Hello from Client 1
Hello from Client 2
[14:05:30] 📨 Hello from Client 2
```

---

## 🧪 Notes

* If only the sender sees the message, make sure you're calling `tx.send(text.to_string())` in the server.
* The server prints messages with a timestamp using the `chrono` crate.

---

### Development Setup

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests (`cargo test`)
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

---

## 📬 Contact

For questions, feedback, or contributions, feel free to reach out:

* **Developer:** Roshan Jha
* **Email:** [roshanjha.work@gmail.com](mailto:roshanjha.work@gmail.com)
* **GitHub:** [Roshan0412](https://github.com/Roshan0412)
