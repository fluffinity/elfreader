mod elf;
use crate::elf::Metadata;

use std::fs::File;

fn main() -> Result<(), i32> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: elfreader <path-to-elf-file>");
        return Err(2);
    }
    let filename = args[1].clone();
    println!("Parsing ELF header of file {}", filename);
    let mut file = match File::open(filename.as_str()) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Can not open file {} Reason:", filename);
            eprintln!("{}", e);
            return Err(1);
        }
    };
    let metadata = match Metadata::parse_file(&mut file) {
        Ok(metadata) => metadata,
        Err(error) => {
            eprintln!("Error parsing the ELF metadata:");
            eprintln!("{:?}", error);
            return Err(1);
        }
    };
    println!("Successfully parsed ELF metadata");
    println!("Content of the header:");
    println!("{:#x?}", metadata.header());
    println!("Content of the program headers:");
    metadata.program_headers().iter().for_each(|header| {
        println!("{:#x?}", header);
    });
    Ok(())
}
