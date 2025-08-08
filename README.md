# 🚀 API Load Tester (DDOS Simulation Tool)

A high-performance Rust-based tool for sending concurrent HTTP requests to test API performance.  
It supports GET, POST, PUT, DELETE methods with request timing, response timing, and TPS calculation.  

---

## 📂 Project Structure

.
├── src  
│   └── main.rs — Main program logic  
├── Cargo.toml — Rust dependencies & metadata  
├── input.csv — Sample input file (API endpoints & configs)  
├── output.csv — Generated results after test  
└── README.md — Project documentation  

---

## 📦 Dependencies

This project uses:

- **tokio** — Asynchronous runtime  
- **reqwest** — HTTP client for making API requests  
- **serde** & **serde_json** — Serialization & deserialization  
- **chrono** — Date & time handling  
- **csv** — CSV read/write  

---

## ⚙️ Installation

Make sure you have **Rust** installed:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Clone the repository and build:

```
git clone https://github.com/bibhutimuni/Api_load_tester-DDOS-tool.git  
cd Api_load_tester-DDOS-tool  
cargo build --release
```

---

## 📥 Input File Format (input.csv)

```
endpoint,method,body,repetition  
https://example.com/api,GET,,5  
https://example.com/post,POST,{"key":"value"},10
```

**Notes:**

- **endpoint** → API URL  
- **method** → HTTP method (GET, POST, PUT, DELETE)  
- **body** → Optional request payload (for POST and PUT)  
- **repetition** → Number of times the request will be sent  

---

## ▶️ Usage

Run the program:

```
./target/release/api_tester input.csv output.csv
```

Example:

```
./target/release/api_tester sample_input.csv results.csv
```

---

## 📤 Output File Format (output.csv)

```
endpoint,request_time,response_time,method,time_taken,status  
https://example.com/api,2025-08-08T10:00:00Z,2025-08-08T10:00:00Z,GET,0.123,200  
https://example.com/post,2025-08-08T10:00:01Z,2025-08-08T10:00:02Z,POST,0.456,201
```

---

## 📊 TPS (Transactions Per Second) Calculation

After execution, the program will print:

```
Total requests processed: 15000  
Total time: 7.52 seconds  
TPS: 1994.68
```

---

## ⚠️ Disclaimer

This tool is for performance testing and educational purposes only.  
Do **NOT** use it to intentionally overload or disrupt services you do not own — doing so may be illegal.

---

## 📜 License

MIT License © 2025 **Bibhutimuni**

---

## 🙌 Contributing

Pull requests are welcome!  
For major changes, please open an issue first to discuss what you would like to change.
