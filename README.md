# 🚀 Rust Static Server

![Rust](https://img.shields.io/badge/Rust-Actix-blue?style=for-the-badge&logo=rust)  
A lightweight and efficient **Rust-based** web server using **Actix Web** to serve static files such as **HTML, CSS, JS, and images**.  
It leverages `include_dir` to embed files within the binary, making it ideal for deployments without external dependencies.

## 📌 Features
- ⚡ **Fast and efficient**: Built with **Actix Web 4.9.0** for high performance.
- 📂 **Embedded static files**: Uses `include_dir` to package files inside the binary.
- 🖥️ **MIME type support**: Detects the correct file types for HTML, CSS, JS, and images using `mime_guess`.
- 🔄 **Built-in compression**: Uses `actix-web::middleware::Compress` for optimized performance.
- 🔧 **Easy deployment**: Ideal for serving frontend applications and static pages.

---

## 🛠️ Installation

### 🔹 1. Clone the repository
```sh
git clone git@github.com:FranklinRuiz/rust-static-server.git
cd rust-static-server
```

### 🔹 2. Build the binary
Make sure you have Rust installed. If not, install it with:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Then, compile the project in release mode:
```sh
cargo build --release
```

### 🔹 3. Run the server
```sh
cargo run
```
By default, the server will be available at `http://127.0.0.1:8080/`.

---

## 📁 Project Structure

```
/rust-static-server
 ├── src/
 │   ├── main.rs
 │   ├── infrastructure/
 │   │   ├── server_handler.rs
 │   └── ...
 ├── static/
 │   ├── index.html
 │   ├── styles.css
 │   ├── script.js
 │   └── images/
 ├── Cargo.toml
 └── README.md
```

---

## 🚀 Usage
- Place your static files in the `static/` folder.
- Open `http://127.0.0.1:8080/` in your browser.
- The server will serve the files directly from the binary.

---

## ⚙️ Server Configuration
By default, the server runs on `127.0.0.1:8080`, but you can change it in `start_server()` inside `server_handler.rs`:

```rust
.bind("0.0.0.0:8080")? // Modify the IP or port here
```

---

## 📦 Dependencies
This project uses the following dependencies in `Cargo.toml`:

```toml
[dependencies]
actix-web = "4.9.0"
include_dir = "0.7.4"
mime_guess = "2.0.5"
```

---

## 🛠️ Development & Contributions
1. **Clone** the repository.
2. **Create a new branch** (`git checkout -b feature-new`).
3. **Make improvements** and **commit changes** (`git commit -m "New feature added"`).
4. **Submit a PR** 🚀.

---

## 📜 License
This project is licensed under the **MIT License**. Feel free to use and modify it! 🎉

