use anyhow::{Result, anyhow};
use clap::Parser;
use log::{Level, Metadata, Record};
use log::{LevelFilter, SetLoggerError};
use std::path::PathBuf;
use std::time::Instant;

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init() -> Result<(), SetLoggerError> {
    let log_debug_level = std::env::var("LOG_DEBUG_LEVEL").unwrap_or_else(|_| "info".to_string());
    let level = match log_debug_level.as_str() {
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Info,
    };
    log::set_logger(&LOGGER).map(|()| log::set_max_level(level))
}

#[derive(Parser)]
struct Args {
    day: u8,
    /// Use the sample input instead of the full one
    #[arg(short, long)]
    sample: bool,
    /// Explicit input path (overrides --sample)
    input: Option<PathBuf>,
}

fn main() -> Result<()> {
    let _ = init();

    let args = Args::parse();
    let kind = if args.sample { "sample" } else { "full" };
    let path = args.input.unwrap_or_else(|| {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("inputs")
            .join(format!("day{:02}_{kind}.txt", args.day))
    });
    let solver =
        aoc::days::get(args.day).ok_or_else(|| anyhow!("day {} not implemented", args.day))?;

    let lines = aoc::util::read_lines_as_string(&path)?;

    let start = Instant::now();
    let (p1, p2) = solver(&lines)?;
    let elapsed = start.elapsed();

    println!("Part 1: {p1}\nPart 2: {p2}\n({elapsed:?})");
    Ok(())
}
