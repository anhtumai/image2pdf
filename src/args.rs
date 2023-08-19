use std::path::PathBuf;

use clap::Parser;

use crate::pagesize::PageSizeInMm;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The input file(s)
    pub input: Vec<PathBuf>,
    /// The output file
    #[arg(short, long, default_value = "output.pdf")]
    pub output: PathBuf,
    /// The page size of the output file
    #[arg(short, long)]
    pub pagesize: Option<PageSizeInMm>,
    /// Sort the input files by human rules (e.g. 1, 2, 10 instead of 1, 10, 2)
    #[arg(short = 's', long = "human-sorting")]
    pub human_sorting: bool,
}
