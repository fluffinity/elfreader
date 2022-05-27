#![deny(clippy::all)]

mod cli;
mod elf;
use clap::Parser;

use crate::elf::Metadata;

use std::fs::File;

fn main() -> Result<(), i32> {
    let arguments = cli::Arguments::parse();
    if arguments.version {
        println!("{}", cli::VERSION);
        return Ok(());
    }
    let filename = arguments.path;
    println!("Parsing ELF header of file {:?}", filename);
    let mut file = match File::open(filename.as_path()) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Can not open file {:?} Reason:", filename);
            eprintln!("{}", e);
            return Err(1);
        }
    };
    let metadata = match Metadata::parse_file(&mut file) {
        Ok(metadata) => metadata,
        Err(error) => {
            eprintln!("Error parsing the ELF metadata:");
            eprintln!("{:#x?}", error);
            return Err(1);
        }
    };
    println!("Successfully parsed ELF metadata");
    if arguments.header {
        println!("Content of the header:");
        println!("{:#x?}", metadata.header());
    }
    if arguments.program_header {
        println!("Content of the program headers:");
        metadata.program_headers().iter().for_each(|header| {
            println!("{:#018x?}", header);
        });
    }
    if arguments.section_header {
        println!("Content of the section headers:");
        metadata.section_headers().iter().for_each(|header| {
            println!("{:#018x?}", header);
        });
    }
    Ok(())
}
