use bitcounts::bitfreq::BitFrequency;
use clap::Parser;
use log::*;
use simple_eyre::eyre::Report;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

// Build the CLI:
#[derive(Parser)]
#[command(version)]
/// Show bit counts for binary files
struct Args {
    /// Show log messages. Multiple -v options increase the verbosity
    #[clap(short='v', long="verbose", action=clap::ArgAction::Count)]
    verbose: u8,
    /// Output bit seperator value
    #[clap(long = "bit-sep", value_name = "SEP", default_value = "\t")]
    bit_sep: String,
    /// Output byte seperator value
    #[clap(long = "byte-sep", value_name = "SEP", default_value = "\t")]
    byte_sep: String,
    /// Block size
    #[clap(
        short = 'b',
        long = "block-bytes",
        value_name = "B",
        default_value = "1"
    )]
    bytes: usize,
    /// Report every N chunks
    #[clap(short = 'c', long = "chunks", value_name = "N")]
    chunks: Option<u32>,
    /// Input raw data file
    #[clap(value_name = "FILE")]
    input_path: PathBuf,
}

fn main() -> Result<(), Report> {
    // Register the Eyre handler:
    simple_eyre::install()?;
    // Parse the CLI arguments:
    let args = Args::parse();
    // Build the logger:
    stderrlog::new()
        .module(module_path!())
        .verbosity(args.verbose as usize)
        .timestamp(stderrlog::Timestamp::Millisecond)
        .init()?;
    // Build the bit frequency counter:
    info!("counter size is {} bytes", args.bytes);
    if let Some(n) = args.chunks {
        info!("reporting counts every {} elements", n);
    }
    let mut bit_counter = BitFrequency::new(args.bytes);
    info!("reading from {}", args.input_path.to_string_lossy());
    let mut input_buffer = BufReader::new(File::open(&args.input_path)?);
    // Initialise the variables we need for the loop:
    let mut n_blocks: u64 = 0;
    let mut input_bytes: Vec<u8> = vec![0_u8; args.bytes];
    // Iterate through the input file:
    while input_buffer.read_exact(&mut input_bytes).is_ok() {
        n_blocks += 1;
        trace!(
            "read {}",
            &input_bytes
                .iter()
                .map(|b| format!("{:08b}", b))
                .collect::<Vec<String>>()
                .join("")
        );
        // Update the bit counter:
        bit_counter.update(&input_bytes);
        if let Some(n) = args.chunks {
            if bit_counter.count() >= n {
                debug!("reporting full block ({} elements)", bit_counter.count());
                println!(
                    "{}",
                    bit_counter.count_string(&args.bit_sep, &args.byte_sep)
                );
                bit_counter.clear();
            }
        }
    }
    // We might have an incomplete block at the end:
    if bit_counter.count() > 0 {
        debug!("reporting partial block ({} elements)", bit_counter.count());
        println!(
            "{}",
            bit_counter.count_string(&args.bit_sep, &args.byte_sep)
        );
    }
    info!(
        "{} blocks ({} bytes) read from file",
        n_blocks,
        n_blocks * bit_counter.n_bytes() as u64
    );
    Ok(())
}
