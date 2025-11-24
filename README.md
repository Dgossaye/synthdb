# SynthDB

> Production-grade synthetic data generator for PostgreSQL  
> Zero-config. Single binary. Referentially intact.

[![Built with Rust](https://img.shields.io/badge/built_with-Rust-d33833.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Open Collective](https://img.shields.io/badge/Sponsor-Open%20Collective-1f87ff.svg)](https://opencollective.com/synthdb)

SynthDB generates realistic, referentially-consistent synthetic data for PostgreSQL from your existing schema — fast, local, and privacy-preserving.

Why use SynthDB?
- Populate staging or CI environments with realistic data without copying production.
- Preserve foreign-key relationships automatically.
- Sample real distributions from existing schemas to produce believable values.
- Run entirely locally — no data leaves your environment.

Quick Start

1. Export your schema (no rows):
```bash
pg_dump -h prod-db.example -U user -d my_app -s > schema.sql
```

2. Create a target database and load the schema:
```bash
createdb my_staging_db
psql -d my_staging_db < schema.sql
```

3. Generate synthetic data:
```bash
synthdb clone \
  --url "postgres://user:pass@localhost:5432/my_staging_db" \
  --rows 5000 \
  --output seed.sql
```

4. Apply the generated data:
```bash
psql -d my_staging_db < seed.sql
```

Result: a fully-seeded staging environment with realistic users, orders, items, etc. — all referentially intact.

Highlights
- Zero-config: sensible defaults for most schemas.
- Referential integrity: builds a dependency DAG and inserts tables in safe order.
- Smart heuristics: infers types and semantic meanings from column names (emails, phones, SKUs, timestamps).
- Value sampling: optionally sample from existing distinct values to preserve realistic distributions.
- Fast: implemented in Rust for native performance.
- Privacy-first: works locally — no telemetry or data exfiltration by default.

Features
- Automatic topological ordering based on foreign keys
- Column "Vibe Engine" for semantic data generation (email, phone, sku, status, timestamps)
- Optional sampling from existing distinct values to mimic real distributions
- Single-binary distribution; cross-platform builds via Cargo
- Pluggable generator heuristics for custom column behaviors

Install

From source (Cargo):
```bash
git clone https://github.com/synthdb/synthdb.git
cd synthdb
cargo install --path .
```

Pre-built binaries may be published on the Releases page.

Usage (CLI)

Common command: clone (inspect schema and generate INSERT statements)
```bash
synthdb clone \
  --url "postgres://user:pass@localhost:5432/my_staging_db" \
  --rows 10000 \
  --output seed.sql \
  --sample-percent 20 \
  --concurrency 4
```

Key flags
- --url (required) — Postgres connection string for the target schema to inspect
- --rows — total rows to generate per primary table (defaults vary by mode)
- --output — path to write SQL INSERTs (stdout if omitted)
- --sample-percent — percent of distinct values to sample from existing columns (0–100)
- --concurrency — number of worker threads for generation and sampling
- --schema — target schema name (defaults to public)
- --dry-run — analyze schema and print plan without generating data
- --help — see all options

How it works (high level)
1. Introspect: read tables, columns, types, and constraints via pg_catalog and information_schema.
2. Build DAG: create a dependency graph from foreign keys and topologically sort tables.
3. Profile (optional): sample distinct values and distributions from columns.
4. Generate: use heuristics and distributions to synthesize values per column.
5. Emit: produce INSERT statements in an order that satisfies foreign keys.

Column heuristics (Vibe Engine)
SynthDB uses column names and types to infer generators:
- email → realistic emails (local + domain)
- phone → formatted phone numbers
- sku, code → uppercase alphanumeric with dashes
- status → small enumerations (active, pending, failed, etc.)
- created_at, updated_at → time-decayed timestamps

Heuristics are pluggable — see Contributing for how to add or tune generators.

Sampling real distributions
When enabled:
- SELECT DISTINCT (with safe limits) to collect candidate values
- Build categorical distributions and sample accordingly
This preserves realistic proportions for categorical columns (e.g., product_category, country).

Performance notes
- Generation is CPU-bound. Increase --concurrency for faster runs.
- Sampling large distinct sets can be heavy on the DB; use --sample-percent to control load.

Limitations (v0.1)
- PostgreSQL only (MySQL/SQLite planned for v0.2)
- Requires an existing schema (does not create tables)
- Produces SQL INSERT statements only (no truncation or migrations)
- Very large sampling may be slow or memory-intensive — use limiting flags

Security & Privacy
- SynthDB is designed for local use. It does not exfiltrate database contents.
- Prefer role-limited DB users for introspection when running against production-like hosts.

Troubleshooting
- Permission denied during introspection: ensure the DB user can read pg_catalog and information_schema.
- Out of memory during sampling: reduce --sample-percent or run on a larger machine.
- Referential cycles: if your schema has cycles without deferred constraints, run --dry-run to inspect the plan and handle cycles manually.

Examples
- CI-friendly: generate small dataset and output to stdout
```bash
synthdb clone --url "$CI_DATABASE_URL" --rows 1000 > /tmp/ci_seed.sql
```

- Generate without sampling real values:
```bash
synthdb clone --url "postgres://..." --rows 2000 --sample-percent 0
```

Development & Contributing
Contributions welcome! Typical workflow:
```bash
git checkout -b feature/my-feature
# implement and test
git commit -m "Add feature"
git push origin feature/my-feature
# open a pull request
```

Guidelines
- Write tests for new generators and sampling logic
- Keep CLI flags backward-compatible
- Document heuristics and new flags in README

Suggested repository files to add (if you want me to generate them):
- CONTRIBUTING.md (contribution guidelines, testing, code style)
- PR_TEMPLATE.md (pull request template)
- CODE_OF_CONDUCT.md

Roadmap
- v0.2: MySQL & SQLite support
- Native write mode: write directly to a target DB (in addition to SQL files)
- GUI and VSCode extension for schema previews
- More generator plugins and community templates

Support & Sponsorship
If SynthDB saves you time, consider sponsoring development:
- Open Collective: https://opencollective.com/synthdb

License
Distributed under the MIT License — see LICENSE.

Contact
- Issues & PRs: https://github.com/synthdb/synthdb/issues
- Sponsorship: https://opencollective.com/synthdb
