use std::fs::File;
use std::io::Read;

mod elf;
mod test_elf_structs_from_bytes;
mod test_from_bytes_endianned;
use elf::ELFHeader;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: elfreader <path-to-elf-file>");
        return;
    }
    let filename = args[1].clone();
    println!("parsing ELF header of file {}", filename);
    let mut file = match File::open(filename.as_str()) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("can not open file {}", filename);
            eprintln!("reason: {}", e);
            return;
        }
    };
    let mut buf = [0; 512];
    let bytes_read = file.read(&mut buf);
    if let Err(e) = bytes_read {
        eprintln!("error reading file. Abort");
        eprintln!("reason: {}", e);
        return;
    }
    let bytes = &buf;
    let header =  match ELFHeader::from_bytes(bytes) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("Error parsing ELF header. Reason:");
            eprintln!("{:?}", e);
            return;
        }
    };
    println!("Header parsed successfully");
    println!("the content is: {:?}", header);
}
