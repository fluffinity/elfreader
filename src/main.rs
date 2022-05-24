mod elf;
use crate::elf::{Header, Word, ProgramHeader};

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

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
    let mut buf = [0; 64];
    let bytes_read = file.read(&mut buf);
    if let Err(e) = bytes_read {
        eprintln!("Error reading file. Abort");
        eprintln!("Reason: {}", e);
        return Err(1);
    }
    let bytes = &buf;
    let header =  match Header::parse_bytes(bytes) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("Error parsing ELF header. Reason:");
            eprintln!("{:?}", e);
            return Err(1);
        }
    };
    println!("Header parsed successfully");
    //TODO implement pretty printing and selective printing of informations
    println!("The content is: {:?}", header);

    let pheader_offset: u64 = match header.program_header_start() {
        Word::Word32(i) => i as u64,
        Word::Word64(i) => i
    };

    if let Err(err) = file.seek(SeekFrom::Start(pheader_offset)) {
        eprintln!("Error reading program headers. Reason:");
        eprintln!("{}", err);
        return Err(1)
    }
    let program_header_size = header.program_header_entry_size() * header.program_header_entry_count();
    let mut buf = Vec::with_capacity(program_header_size as usize);
    buf.extend(std::iter::repeat(0).take(program_header_size as usize));
    if let Err(err) = file.read_exact(buf.as_mut_slice()) {
        eprintln!("Error reading program headers. Reason:");
        eprintln!("{}", err);
        return Err(1)
    }
    let program_headers: Vec<_> = (0..header.program_header_entry_count())
        .into_iter()
        .map(|i| {
            let offset = (i * header.program_header_entry_size()) as usize;
            ProgramHeader::parse_bytes(&buf.as_mut_slice()[offset..], header.word_width(), header.endianness())
        })
        .collect();
    if let Some(err) = program_headers.iter().find(|entry| {entry.is_err()}) {
        eprintln!("Error parsing program headers. Reason:");
        eprintln!("{:?}", err);
        return Err(1);
    }
    let program_headers: Vec<_> = program_headers.into_iter().map(|entry| entry.expect("checked for Err")).collect();
    println!("Program headers:");
    program_headers.iter().for_each(|pheader| {
        println!("{:#x?}", pheader);
    });
    Ok(())
}
