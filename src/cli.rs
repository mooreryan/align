use std::path::PathBuf;

use clap::Parser;

/// Returns Ok if the `file_name` is for a existing file.
fn exists(file_name: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(file_name);

    if path.exists() {
        Ok(path)
    } else {
        Err(format!("file {file_name} does not exist"))
    }
}

/// Returns Ok if the `file_name` is NOT an existing file.
fn doesnt_exist(file_name: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(file_name);

    if path.exists() {
        Err(format!("file {file_name} already exists"))
    } else {
        Ok(path)
    }
}

#[derive(Parser)]
#[command(version)]
/// Perform all-vs-all global alignments for the input sequences
pub struct Cli {
    /// FASTA file input
    #[arg(value_parser = exists)]
    pub in_file: PathBuf,

    /// Output file name
    #[arg(value_parser = doesnt_exist)]
    pub out_file: PathBuf,

    /// Number of worker threads for aligning
    ///
    /// The total number of threads used by the program will be threads + 1.
    ///
    /// Pick a number of threads just under your number of CPUs.
    #[arg(short, long, default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..))]
    threads: u8,

    /// Gap open penalty
    #[arg(long, default_value_t = 10)]
    gap_open: u8,

    /// Gap extend penalty
    ///
    /// EMBOSS needle uses -0.5 as gap_extend, but we only take integers.
    #[arg(long, default_value_t = 1)]
    gap_extend: u8,

    /// Show the alignment operations
    #[arg(long, default_value_t = false)]
    pub show_aln_ops: bool,
}

impl Cli {
    pub fn threads(&self) -> usize {
        usize::from(self.threads)
    }
    pub fn gap_open(&self) -> i32 {
        -i32::from(self.gap_open)
    }
    pub fn gap_extend(&self) -> i32 {
        -i32::from(self.gap_extend)
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
