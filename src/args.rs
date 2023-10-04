use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, about, version)]
pub struct Args {
    // == Main Input ==
    pub input: String,

    // == Arguments ==
    /// Shows the internal conversion steps between units.
    #[clap(long)]
    pub debug: bool,
    /// Print the dimensions of the input and output units.
    #[clap(short, long)]
    pub dimensions: bool,
}
