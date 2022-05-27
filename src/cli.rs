use std::path::PathBuf;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// elfreader is a small tool to read the metadata of binary files in the ELF format.
/// This includes the architecture the code is for, whether it is 32- or 64-bits etc.
#[derive(clap::Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Arguments {
    #[clap(short, long)]
    pub version: bool,

    /// The path to the ELF file
    #[clap(parse(from_os_str))]
    pub path: PathBuf,

    /// Print the ELF header
    #[clap(short, long = "program-header")]
    pub program_header: bool,

    /// Print the program headers
    #[clap(short, long = "section-header")]
    pub section_header: bool,

    /// Print the section headers
    #[clap(short, long = "header")]
    pub header: bool,
}
