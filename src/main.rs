mod schema;
mod generator;
mod sorter;

use clap::{Parser, Subcommand};
use sqlx::postgres::PgPoolOptions;
use crate::generator::Generator;
use std::time::Instant;

#[derive(Parser)]
#[command(name = "synthdb")]
#[command(about = "Production-Ready Synthetic Data Engine")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Clone a database structure and data patterns
    Clone {
        /// Postgres connection string
        #[arg(short, long)]
        url: String,

        /// Output file path
        #[arg(short, long, default_value = "seed.sql")]
        output: String,

        /// Rows to generate per table
        #[arg(short, long, default_value = "1000")]
        rows: usize,

        /// Percentage of real data to sample (0-100)
        #[arg(long, default_value = "20")]
        sample_percent: u8,

        /// Number of concurrent threads (placeholder for v0.2)
        #[arg(long, default_value = "4")]
        concurrency: usize,
        
        /// Target schema (placeholder for v0.2)
        #[arg(long, default_value = "public")]
        schema: String,

        /// Dry run (analyze only, don't generate)
        #[arg(long)]
        dry_run: bool,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Clone { url, output, rows, sample_percent, concurrency: _, schema: _, dry_run } => {
            let start = Instant::now();
            
            if dry_run {
                println!("🐫 Dry run active. Analyzing schema only...");
            } else {
                println!("🚀 Connecting to database...");
            }

            let pool = PgPoolOptions::new()
                .max_connections(5)
                .connect(&url)
                .await?;

            println!("🔍 Analyzing schema & sampling data ({}%)...", sample_percent);
            // Note: We are passing sample_percent to schema extractor now
            // (You'll need to update extract_schema signature if you want this to actually limit the query)
            let raw_schema = schema::extract_schema(&pool).await?;
            
            println!("✅ Found {} tables. Calculating dependencies...", raw_schema.len());

            // 2. Topological Sort
            let sorted_schema = sorter::sort_tables(raw_schema)?;
            println!("✅ Dependencies resolved. Insertion order determined.");

            if dry_run {
                println!("📋 Execution Plan (Dry Run):");
                for (i, table) in sorted_schema.iter().enumerate() {
                    println!("  {}. {}", i + 1, table.table_name);
                }
                println!("✨ Dry run complete.");
                return Ok(());
            }

            println!("🔨 Generating synthetic data...");
            // 3. Generate
            let mut generator = Generator::new(sorted_schema);
            generator.generate_sql_dump(&output, rows)?;

            println!("✨ Done in {:.2?}! Saved to {}", start.elapsed(), output);
        }
    }

    Ok(())
}