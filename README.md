# 🚀 Rust Static File Server

![Rust](https://img.shields.io/badge/Rust-Actix-blue?style=for-the-badge&logo=rust)  
A **lightweight and efficient** static file server built with **Rust** and **Actix Web**.  
This server dynamically serves static files **without recompiling**, ensuring **UTF-8 support, compression, and caching** for improved performance.

---

## 📌 Features
✅ **Fast and Lightweight** – Uses **Actix Web** for high-performance asynchronous file serving.  
✅ **Dynamic File Serving** – Serves static files from disk **without recompiling** the binary.  
✅ **MIME Type Detection** – Automatically detects file types (`HTML, CSS, JS, images`) using `mime_guess`.  
✅ **Compression Support** – Uses `actix-web::middleware::Compress` for **gzip and brotli** compression.  
✅ **Efficient Caching** – Implements `Cache-Control` and `Last-Modified` headers to reduce unnecessary requests.  
✅ **Cross-Platform** – Runs on **Linux, Windows, and macOS**.  
✅ **Minimal Dependencies** – Small and lightweight, ideal for deploying frontend applications and static assets.

---

## 🛠️ Installation

### 🔹 1. Clone the repository
```sh
git clone https://github.com/your-username/lab.git  
cd lab  
```

### 🔹 2. Install Rust (if not installed)
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh  
```

### 🔹 3. Build the binary
Compile the project in **release mode** for better performance:
```sh
cargo build --release  
```

### 🔹 4. Run the server
```sh
cargo run --release  
```
By default, the server will be available at **http://127.0.0.1:8080/**.

---

## 📁 Project Structure
```
/lab  
 ├── src/  
 │   ├── main.rs  
 │   ├── handlers/  
 │   │   ├── mod.rs  
 │   │   ├── server.rs  
 │   └── ...  
 ├── static/  
 │   ├── index.html  
 │   ├── styles.css  
 │   ├── script.js  
 │   └── images/  
 ├── Cargo.toml  
 └── README.md  
```

📂 **`static/`** → Directory where static files are stored (HTML, CSS, JS, images).  
📜 **`main.rs`** → The main application entry point.  
🛠️ **`server.rs`** → Contains the Actix Web configuration and request handling logic.

---

## 🚀 Usage
1. Place your static files in the **`static/`** directory.
2. Open **http://127.0.0.1:8080/** in your browser.
3. The server will **automatically** detect and serve updated files **without requiring a restart**.

---

## ⚙️ Server Configuration
By default, the server runs on **127.0.0.1:8080**, but you can modify it in **`server.rs`**:
```rust
.bind("0.0.0.0:8080")? // Change IP or port here  
```
To serve files from a different directory, update this line:
```rust
.service(fs::Files::new("/", "./static").index_file("index.html"))  
```

---

## 📦 Dependencies
The project uses the following dependencies in **Cargo.toml**:
```toml
[dependencies]  
actix-web = "4.9.0"  
actix-files = "0.6.6"  
num_cpus = "1.16.0"  
```

---

## 🛠️ Development & Contributions
Want to contribute? Follow these steps:
1. **Fork the repository**.
2. **Create a new branch** (`git checkout -b feature-new`).
3. **Make your improvements** and **commit changes** (`git commit -m "Added new feature"`).
4. **Submit a pull request (PR)** 🚀.

---

## 📜 License
This project is licensed under the **MIT License**. Feel free to use, modify, and distribute it! 🎉