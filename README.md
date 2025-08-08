# Api_load_tester-DDOS-tool
A high-performance concurrent API testing tool written in Rust that can send thousands of HTTP requests per second while tracking response times, statuses, and results. It’s designed for load testing, performance benchmarking, and reliability checks of REST APIs.

# 🚀 Rust Concurrent API Tester

A high-performance **concurrent API testing tool** written in Rust, designed to send thousands of HTTP requests per second (TPS) to multiple endpoints.  
It supports **GET, POST, PUT, DELETE** methods, measures **request & response times**, calculates **TPS**, and writes results to a CSV file.

---

## 📌 Features

- **Concurrent Request Execution** — Send thousands of requests in parallel.
- **Multiple HTTP Methods** — GET, POST, PUT, DELETE supported.
- **Custom Payloads** — Send body data for POST/PUT requests.
- **Request & Response Timestamps** — Track exact request start and response arrival times.
- **Time Taken per Request** — Measure API latency.
- **Repetition Support** — Repeat the same request multiple times.
- **TPS Calculation** — Calculate transactions per second.
- **CSV Output** — Save detailed results for analysis.

---

## 📂 Project Structure

.
├── src
│ └── main.rs # Main program logic
├── Cargo.toml # Rust dependencies & metadata
├── input.csv # Sample input file (API endpoints & configs)
├── output.csv # Generated results after test
└── README.md # Project documentation


---

## 📦 Dependencies

This project uses:

- [tokio](https://crates.io/crates/tokio) — Asynchronous runtime.
- [reqwest](https://crates.io/crates/reqwest) — HTTP client for making API requests.
- [serde](https://crates.io/crates/serde) & [serde_json](https://crates.io/crates/serde_json) — Serialization & deserialization.
- [chrono](https://crates.io/crates/chrono) — Date & time handling.
- [csv](https://crates.io/crates/csv) — CSV read/write.

---

## ⚙️ Installation

Make sure you have **Rust** installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

git clone https://github.com/bibhutimuni/Api_load_tester-DDOS-tool.git
cd rust-concurrent-api-tester
cargo build --release

📥 Input File Format (input.csv)
endpoint	method	body	repetition
https://example.com/api	GET		5
https://example.com/post	POST	{"key":"value"}	10

Notes:

endpoint → API URL.

method → HTTP method (GET, POST, PUT, DELETE).

body → Optional request payload (for POST and PUT).

repetition → Number of times the request will be sent.

▶️ Usage
Run the program:

bash
Copy
Edit
./target/release/api_tester input.csv output.csv
Example:

bash
Copy
Edit
./target/release/api_tester sample_input.csv results.csv
📤 Output File Format (output.csv)
endpoint	request_time	response_time	method	time_taken	status
https://example.com/api	2025-08-08T10:00:00Z	2025-08-08T10:00:00Z	GET	0.123	200
https://example.com/post	2025-08-08T10:00:01Z	2025-08-08T10:00:02Z	POST	0.456	201

📊 TPS (Transactions Per Second) Calculation
After execution, the program will print:

yaml
Copy
Edit
Total requests processed: 15000
Total time: 7.52 seconds
TPS: 1994.68
⚠️ Disclaimer
This tool is for performance testing and educational purposes only.
Do NOT use it to intentionally overload or disrupt services you do not own — doing so may be illegal.

📜 License
MIT License © 2025 [Bibhutimuni]

🙌 Contributing
Pull requests are welcome!
For major changes, please open an issue first to discuss what you would like to change.

yaml
Copy
Edit

---

If you want, I can also **add a `Cargo.toml`** with all dependencies and a **sample `input.csv`** so your GitHub upload is immediately runnable. That way, anyone cloning your repo can just run `cargo run` and test it instantly.
