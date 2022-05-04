use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    pub input: Vec<String>,

    #[clap(short, long, default_value = "test_working.pdf")]
    pub output: String,

    #[clap(short, long)]
    pub pagesize: Option<String>,
}
