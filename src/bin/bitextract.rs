use clap::Parser;
use log::*;
use simple_eyre::eyre::Report;
use std::fmt;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;

pub struct BitAccumulator {
    bits: u8,
    n: u8,
}

impl BitAccumulator {
    pub fn new() -> Self {
        Self {
            bits: 0b00000000_u8,
            n: 0_u8,
        }
    }
    pub fn current_bits(&self) -> u8 {
        self.n
    }
    pub fn bits(&self) -> u8 {
        self.bits
    }
    pub fn is_complete(&self) -> bool {
        self.n == 8
    }

    pub fn clear(&mut self) {
        self.bits = 0b00000000_u8;
        self.n = 0_u8;
    }
    pub fn push(&mut self, bit: bool) {
        if self.n < 8 {
            match bit {
                true => self.bits |= 0x1 << (7 - self.n),
                false => self.bits &= !(0x1 << (7 - self.n)),
            }
            self.n += 1;
        }
    }
}

impl Default for BitAccumulator {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for BitAccumulator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bits_str = (0..self.n)
            .map(|i| match (self.bits >> (7 - i)) & 0x1 == 0x1 {
                true => "1",
                false => "0",
            })
            .collect::<String>();
        write!(f, "{}", bits_str)
    }
}

// Build the CLI:
#[derive(Parser)]
#[command(version)]
/// Show bit counts for binary files
struct Args {
    /// Show log messages. Multiple -v options increase the verbosity
    #[clap(short='v', long="verbose", action=clap::ArgAction::Count)]
    verbose: u8,
    /// Report bit
    #[clap(value_name = "BIT")]
    report_bit: u8,
    /// Input raw data file
    #[clap(value_name = "FILE")]
    input_path: PathBuf,
    /// Output data file
    #[clap(value_name = "FILE")]
    output_path: PathBuf,
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
    // Build the reader buffer:
    let mut input_bytes: Vec<u8> = vec![0_u8; 2];
    // Open the input file:
    info!("reading from {}", args.input_path.to_string_lossy());
    let mut input_buffer = BufReader::new(File::open(&args.input_path)?);
    // Open the output file:
    info!("writing to {}", args.output_path.to_string_lossy());
    let mut output_buffer = BufWriter::new(File::create(&args.output_path)?);
    // Initialise the variables we need for the loop:
    let mut bit_acc = BitAccumulator::new();
    let report_bit = (15 - args.report_bit) as u16;
    debug!(
        "report bit mask is {:016b}",
        0b1000000000000000_u16 >> args.report_bit
    );
    let mut n_blocks: u64 = 0_u64;
    let mut n_output_bytes: u64 = 0_u64;
    // Iterate through the input file:
    while input_buffer.read_exact(&mut input_bytes).is_ok() {
        let (int_bytes, _) = input_bytes.split_at(std::mem::size_of::<u16>());
        let input_value = u16::from_be_bytes(int_bytes.try_into()?);
        let input_bit: bool = (input_value >> report_bit) & 0x1 == 0x1;
        trace!("read value {:016b} ({})", input_value, input_bit);
        bit_acc.push(input_bit);
        trace!("pushed bit (nbits = {})", bit_acc.current_bits());
        n_blocks += 1;
        if bit_acc.is_complete() {
            output_buffer.write_all(&[bit_acc.bits])?;
            // println!("{}", bit_acc);
            bit_acc.clear();
            trace!("wrote output byte (nbits = {})", bit_acc.current_bits());
            n_output_bytes += 1;
        }
    }
    info!("read {} 16bit blocks", n_blocks);
    info!("returned {} bytes", n_output_bytes);
    // Report remaining data:
    match bit_acc.current_bits() {
        0 => info!("all bits returned"),
        1 => info!("1 hanging bit dropped"),
        n => info!("{} hanging bits dropped", n),
    }
    Ok(())
}
