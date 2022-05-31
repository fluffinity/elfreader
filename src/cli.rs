use std::path::PathBuf;

/// elfreader is a small tool to read the metadata of binary files in the ELF format.
/// This includes the architecture the code is for, whether it is 32- or 64-bits,
/// the endianness of the code and data, the file type of this binary, the ABI it uses
/// and other metedata required by the linker or OS to properly load the binary.
#[derive(clap::Parser)]
#[clap(version, about, long_about = None)]
pub struct Arguments {
    /// The path to the ELF file
    #[clap(parse(from_os_str))]
    pub path: PathBuf,

    /// Print the ELF header
    #[clap(short, long)]
    pub program_header: bool,

    /// Print the program headers
    #[clap(short, long)]
    pub section_header: bool,

    /// Print the section headers
    #[clap(long)]
    pub header: bool,
}
