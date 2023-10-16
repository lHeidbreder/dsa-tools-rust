use clap::Parser;
use dsa_tools_rust::Format;

#[derive(Parser)]
struct Cli {
    #[arg(short = 'v', long = "verbose", default_value_t = false)]
    verbose: bool,
    #[arg(short = 'f', long = "format", default_value_t = Format::TEXT, ignore_case = true)]
    format: Format,
    #[arg(short = 'o', long = "output", default_value = "/dev/null")]
    outfile: std::path::PathBuf,
    #[arg(short = 'x', long = "seed", default_value_t = 0)]
    seed: i64,
    
}

impl std::fmt::Display for Cli {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Seed: {}; Output: {}; Format: {}", self.seed, self.outfile.display(), self.format)
    }
}

fn main () {
    panic!("Not implemented")
}