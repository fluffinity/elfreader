use std::fs::File;
use std::io::Read;

mod elf;
mod test_elf_structs_from_bytes;
mod test_from_bytes_endianned;
use elf::Header;

fn main() -> Result<(), i32> {
    println!("Hello, world!");
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: elfreader <path-to-elf-file>");
        return Err(1);
    }
    let filename = args[1].clone();
    println!("parsing ELF header of file {}", filename);
    let mut file = match File::open(filename.as_str()) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("can not open file {}", filename);
            eprintln!("reason: {}", e);
            return Err(1);
        }
    };
    let mut buf = [0; 64];
    let bytes_read = file.read(&mut buf);
    if let Err(e) = bytes_read {
        eprintln!("error reading file. Abort");
        eprintln!("reason: {}", e);
        return Err(1);
    }
    let bytes = &buf;
    let header =  match Header::from_bytes(bytes) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("Error parsing ELF header. Reason:");
            eprintln!("{:?}", e);
            return Err(1);
        }
    };
    println!("Header parsed successfully");
    //TODO implement pretty printing and selective printing of informations
    println!("the content is: {:?}", header);
    Ok(())
}
