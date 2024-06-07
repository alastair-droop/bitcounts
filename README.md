# Count Bit Frequencies in Files

The `bitcounts` script tabulates the frequency of bits.
Files are read in as blocks of B bytes and the frequency of bits in each position collated.
If specified (with the `-c` argument), then collated bit counts are reported after every N input blocks.

## Usage

~~~plain
Usage: bitcounts [OPTIONS] <FILE>

Arguments:
  <FILE>  Input raw data file

Options:
  -v, --verbose...       Show log messages. Multiple -v options increase the verbosity
      --bit-sep <SEP>    Output bit seperator value [default: "\t"]
      --byte-sep <SEP>   Output byte seperator value [default: "\t"]
  -b, --block-bytes <B>  Block size [default: 1]
  -c, --chunks <N>       Report every N chunks
  -h, --help             Print help
  -V, --version          Print version
~~~

## Licence

These tools are released under the [MIT License](https://opensource.org/licenses/MIT).

## Building

Before installation, you'll need to install [Rust](https://www.rust-lang.org/).

~~~bash
git clone https://github.com/alastair-droop/bitcounts.git
cd bitcounts
cargo install --path .
~~~
