# SynthDB 🦀

> **Production-grade synthetic data generator for PostgreSQL.**
> **Zero config. Single binary. Referentially Intact.**

[![Rust](https://img.shields.io/badge/built_with-Rust-d33833.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Open Collective](https://img.shields.io/badge/Sponsor%20us-Open%20Collective-1f87ff.svg)](https://opencollective.com/synthdb)

**SynthDB** is a database seeding engine. It connects to your PostgreSQL database, reads the schema (including foreign keys and types), analyzes the data distribution, and generates statistically realistic synthetic data.

It solves the **"Staging Data Problem"**: How do I get 100k realistic users and orders into my staging environment without copying sensitive production data?

---

## 🚀 Why SynthDB?

Most developers write custom "seeding scripts" using libraries like **Faker.js**. This is slow, brittle, and hard to maintain.

| Feature | Custom Scripts (Faker.js) | SynthDB |
| :--- | :--- | :--- |
| **Setup Time** | Hours (Writing loops, defining models) | **Seconds** (1 Command) |
| **Relationships** | Manual (You must code the logic) | **Automatic** (Topological Sort) |
| **Realism** | Generic (Random words) | **Sampled** (Reads your actual DB values) |
| **Performance** | Slow (Node/Python loops) | **Instant** (Rust Native) |
| **Privacy** | - | **Air-gapped** (Runs locally) |

---

## 📦 Installation

Currently, SynthDB is installed from source via Rust/Cargo.

```bash
# 1. Clone the repo
git clone [https://github.com/YOUR_USERNAME/synthdb.git](https://github.com/YOUR_USERNAME/synthdb.git)
cd synthdb

# 2. Install globally
cargo install --path .
🛠 usage Guide (The Workflow)
SynthDB is designed to populate an existing schema with data. It does not create tables; it fills them.

Scenario: "I want to clone Prod to Staging, but with fake data."
Step 1: Export your Schema (No Data)
Get the structure of your production database, but exclude the actual data to preserve privacy.

Bash

# -s means "Schema Only"
pg_dump -h prod-db.com -U user -d my_app -s > schema.sql
Step 2: Create your Local/Staging DB
Create a clean database and apply the schema.

Bash

createdb my_staging_db
psql -d my_staging_db < schema.sql
Step 3: Run SynthDB
Point SynthDB at your local/staging database. It will read the table structure and generate the insert statements.

Bash

synthdb clone \
  --url postgres://user:pass@localhost:5432/my_staging_db \
  --rows 5000 \
  --output seed.sql
Step 4: Apply the Synthetic Data
Import the generated SQL file into your staging database.

Bash

psql -d my_staging_db < seed.sql
Result: Your staging database now has 5,000 realistic users, orders, and items, all correctly linked together.

🧠 How It Works
1. The "Vibe" Engine (Smart Seeding)
SynthDB uses heuristics to detect what a column is based on its name:

email → Generates jim.halpert@example.com

phone → Generates valid phone formats

sku → Generates product codes like PROD-4812

status → Picks from common defaults like active, pending

created_at → Generates time-decayed timestamps

2. The "Reader" (Data Sampling)
If you have a column like product_category, SynthDB queries your DB to find distinct values (e.g., "Electronics", "Home"). It uses these real values in the synthetic data so your app feels authentic.

3. The Sorter (Topological Sort)
It builds a Directed Acyclic Graph (DAG) of your foreign keys. It guarantees that Users are created before Orders, and Orders before OrderItems, so you never get Foreign Key Violations.

⚠️ Limitations (v0.1 MVP)
Database Support: Currently supports PostgreSQL only. MySQL and SQLite support is coming in v0.2.

Existing Schema: The target database must already have tables created.

Data Appending: SynthDB generates INSERT statements. It does not TRUNCATE tables first.

🤝 Contributing
We love pull requests!

Fork the repo.

Create a feature branch.

Commit your changes.

Push to the branch and open a PR.

💰 Support the Project
If this tool saved you time setting up your staging environment, consider supporting our development!

Become a Sponsor on Open Collective

📜 License
Distributed under the MIT License.