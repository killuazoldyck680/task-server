# Secure Persistent Task Management Microservice

A production-ready, highly concurrent asynchronous CRUD microservice built with Rust using the Axum web framework, SQLx, and a local SQLite relational database.

This project demonstrates advanced backend engineering patterns including multi-threaded asynchronous runtimes, persistent relational data management, custom request validation interceptors, and strict API-key header middleware authorization.

---

## 🛠️ Architecture & Core Features

- **Asynchronous Engine:** Powered by `tokio` and `axum` (v0.7+) for scalable, multi-threaded request handling.
- **Persistent Disk Storage:** Integrated with `sqlx` and `sqlite` utilizing connection pools. Automatically initializes schema migration on boot without external scripting hooks.
- **Advanced Request Validation:** Employs a custom generic `FromRequest` extractor payload wrapper (`ValidatedJson`) utilizing the `validator` crate to reject structurally defective payloads before hitting business logic.
- **Strict Authorization Gateways:** Utilizes a custom `FromRequestParts` extractor (`ApiKey`) to protect state-altering routes (`POST`, `DELETE`) via strict incoming `X-API-Key` HTTP header inspection.

---

## 🚀 Tech Stack Dependencies

- **Language:** Rust (Stable)
- **Web Framework:** [Axum v0.7](https://crates.io/crates/axum)
- **Runtime:** [Tokio](https://crates.io/crates/tokio)
- **Database Driver/ORM:** [SQLx](https://crates.io/crates/sqlx) with SQLite and Tokio runtime features.
- **Serialization:** [Serde](https://crates.io/crates/serde)
- **Data Guarding:** [Validator](https://crates.io/crates/validator)

---

## 🛑 Prerequisites

Before running this service, ensure you have the following installed:

- [Rust & Cargo](https://www.rust-lang.org/tools/install)
- `curl` (for testing endpoints)

---

## ⚙️ Quick Start Installation

1. **Clone your repository:**
   ```bash
   git clone https://github.com/killuazoldyck680/task-server
   cd task-server
   Compile and spin up the server:Bashcargo run
   Note: Upon first launch, SQLx will automatically create the relational file tasks.db?mode=rwc in your root layout and safely prepare the tasks schema.🛣️ API Endpoint Reference & Testing1. Root HandshakeEndpoint: GET /Access: PublicTest Command:Bashcurl [http://127.0.0.1:3000/](http://127.0.0.1:3000/)
   ```
2. Fetch All TasksEndpoint: GET /taskAccess: PublicTest Command:Bashcurl [http://127.0.0.1:3000/task](http://127.0.0.1:3000/task)
3. Create a Task (Protected & Validated)Endpoint: POST /taskAccess: Protected (Requires X-API-Key Header)Validation Constraints: title cannot be empty string and must be $\le 100$ characters.Test Command (Success Path):Bashcurl -X POST [http://127.0.0.1:3000/task](http://127.0.0.1:3000/task) \
    -H "Content-Type: application/json" \
    -H "X-API-Key: my-super-secret-freelance-key-123" \
    -d '{"title": "Implement API Security Architecture"}'
   Test Command (Validation Failure - Empty String):Bashcurl -X POST [http://127.0.0.1:3000/task](http://127.0.0.1:3000/task) \
    -H "Content-Type: application/json" \
    -H "X-API-Key: my-super-secret-freelance-key-123" \
    -d '{"title": ""}'
   Expected Status: 422 Unprocessable EntityTest Command (Authorization Failure - Missing Header):Bashcurl -X POST [http://127.0.0.1:3000/task](http://127.0.0.1:3000/task) \
    -H "Content-Type: application/json" \
    -d '{"title": "Unauthorized task injection"}'
   Expected Status: 401 Unauthorized4. Toggle Task Completion StatusEndpoint: PATCH /task/{id}Access: PublicTest Command:Bashcurl -X PATCH [http://127.0.0.1:3000/task/1](http://127.0.0.1:3000/task/1)
4. Remove Task (Protected)Endpoint: DELETE /task/{id}Access: Protected (Requires X-API-Key Header)Test Command:Bashcurl -X DELETE [http://127.0.0.1:3000/task/1](http://127.0.0.1:3000/task/1) \
   -H "X-API-Key: my-super-secret-freelance-key-123" \
