# SynthDB 🦀
> The Universal Database Seeder — production-grade synthetic data, zero config, context-aware.

[![Crates.io](https://img.shields.io/crates/v/synthdb.svg)](https://crates.io/crates/synthdb)
[![Built with Rust](https://img.shields.io/badge/built_with-Rust-d33833.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Open Collective](https://img.shields.io/badge/Sponsor-Open%20Collective-1f87ff.svg)](https://opencollective.com/synthdb)

SynthDB reads your PostgreSQL schema and generates statistically realistic, relational data — automatically. No config files, no scripting, just intelligent, context-aware seeding that preserves constraints and relationships.

Why spend hours handcrafting fixtures when SynthDB can:
- infer meaning from column names,
- preserve data types and precision,
- and maintain referential integrity across complex schemas?

Perfect for staging, testing, demos, and load simulations.

---

## Highlights

- 🧠 Universal Heuristic Engine  
  SynthDB goes beyond types. It understands context: names, suffixes, domain concepts and common patterns — then generates natural-looking, coherent values.

- 🔗 Referential Integrity by Default  
  Detects foreign keys and uses topological ordering to insert parent records first so there are no broken references.

- ⚖️ Precision Respect  
  Honors numeric precision, varchar limits, and constraints (EX: NUMERIC(10,2), VARCHAR(10)) so generated values fit schema requirements.

- 🛠 Rich type support  
  Works with UUID, JSONB, ARRAYs, INET, MACADDR and more. Produces valid network addresses, MACs, file extensions, and media paths based on context.

- 🔍 Semantic & Contextual Generation  
  If it sees `first_name` and `email`, it will generate `john.doe@example.com` — not random gibberish. If it sees `nuclear_reactor_name`, it knows to output realistic entity names for that domain.

---

## Feature Overview

- Semantic awareness — contextual generation from column names and patterns
- Dynamic entity recognition across any domain (Healthcare, Aviation, IoT, Finance, …)
- Topological insertion order to maintain referential integrity
- Correct handling of SQL types and constraints
- Fine-grained generation for networks, media, and file paths
- Produces portable SQL seed files that can be applied with psql

---

## Quick Start

Install from crates.io:

```bash
cargo install synthdb
```

Generate seed SQL from an existing PostgreSQL database schema:

```bash
synthdb clone \
  --url "postgres://user:pass@localhost:5432/my_staging_db" \
  --rows 1000 \
  --output seed.sql
```

Apply the generated seed:

```bash
psql -d my_staging_db -f seed.sql
```

Tip: Start with a small number of rows to preview results, then scale up.

---

## How SynthDB Thinks (Examples)

Column name -> Generated value -> Why

- passenger_name -> "John Smith"  
  Detected Person context (names are coherent across related columns)

- contact_email -> "john.smith@gmail.com"  
  Matches first/last name and common email patterns

- airport_facility_name -> "Kirlin Airport"  
  Detected Entity + suffix inference

- flight_code -> "AA-4821"  
  Code pattern detection (alphanumeric formats)

- cockpit_voice_path -> "/uploads/audio/rec.mp3"  
  Audio/media context yields appropriate file extension/path

- mac_address -> "00:0a:95:9d:68:16"  
  Valid hardware ID generation

- wallet_balance -> 402.93  
  Respects NUMERIC precision

---

## Advanced Usage

- Filter which tables to seed
- Customize row count per table
- Export as CSV or JSON for downstream pipelines (coming soon)

Run `synthdb --help` for the full set of flags and options.

---

## Roadmap

- ✅ PostgreSQL Support
- ✅ Universal Heuristic Engine
- ✅ Context-Aware Data Generation
- ☐ MySQL / SQLite Support
- ☐ Direct Database Insertion (skip .sql file)
- ☐ Per-field customization UI

Want a feature? Open an issue or submit a PR — contributions are welcome!

---

## Contributing

We love Rustaceans! 🦀

- Fork the repo
- Create a feature branch
- Write tests and documentation
- Submit a Pull Request

Please follow the repo's code style and include a clear, concise PR description.

Sponsor or support the project on Open Collective if SynthDB saved you time — it helps us keep improvements coming.

---

## License

SynthDB is released under the MIT License. See LICENSE for details.

---

Made with care for developers who want realistic data without the fuss. If you enjoy SynthDB or it saved you time, tell a colleague — and consider sponsoring the project.
