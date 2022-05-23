use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

mod elf;
mod test_elf_structs_from_bytes;
mod test_from_bytes_endianned;
use elf::Header;

use crate::elf::{Word, ProgramHeader};

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
    let header =  match Header::parse_bytes(bytes) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("error parsing ELF header. Reason:");
            eprintln!("{:?}", e);
            return Err(1);
        }
    };
    println!("header parsed successfully");
    //TODO implement pretty printing and selective printing of informations
    println!("the content is: {:?}", header);
    let pheader_offset: u64 = match header.program_header_start() {
        Word::Word32(i) => i as u64,
        Word::Word64(i) => i
    };

    if let Err(err) = file.seek(SeekFrom::Start(pheader_offset)) {
        eprintln!("error reading program headers. Reason:");
        eprintln!("{}", err);
        return Err(1)
    }
    let program_header_size = header.program_header_entry_size() * header.program_header_entry_count();
    let mut buf = Vec::with_capacity(program_header_size as usize);
    buf.extend(std::iter::repeat(0).take(program_header_size as usize));
    if let Err(err) = file.read_exact(buf.as_mut_slice()) {
        eprintln!("error reading program headers. Reason:");
        eprintln!("{}", err);
        return Err(1)
    }
    let program_headers: Vec<_> = (0..header.program_header_entry_count()).into_iter()
    .map(|i| {
        let offset = (i * header.program_header_entry_size()) as usize;
        ProgramHeader::parse_bytes(&buf.as_mut_slice()[offset..], header.word_width(), header.endianness())
    })
    .collect();
    if let Some(err) = program_headers.iter().find(|entry| {entry.is_err()}) {
        eprintln!("error parsing program headers. Reason:");
        eprintln!("{:?}", err);
        return Err(1);
    }
    let program_headers: Vec<_> = program_headers.into_iter().map(|entry| entry.expect("checked for Err")).collect();
    let program_header = ProgramHeader::parse_bytes(buf.as_mut_slice(), header.word_width(), header.endianness());
    if let Err(err) = program_header {
        eprint!("error parsing program headers. Reason:");
        eprintln!("{:?}", err);
        return Err(1)
    }
    let program_header = program_header.expect("checked for Err");
    println!("successfully read program headers.");
    println!("{:?}", program_header);
    println!("program headers:");
    program_headers.iter().for_each(|pheader| {
        println!("{:?}", pheader);
    });
    Ok(())
}
