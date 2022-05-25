use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use super::{Header, ParseError, ProgramHeader};

pub struct Metadata {
    header: Header,
    program_headers: Vec<ProgramHeader>,
}

#[derive(Debug)]
pub enum MetadataParseError {
    InvalidELF(ParseError),
    IOError(std::io::Error),
}

impl Metadata {
    pub fn new(header: Header, program_headers: Vec<ProgramHeader>) -> Metadata {
        Metadata {
            header,
            program_headers,
        }
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn program_headers(&self) -> &[ProgramHeader] {
        self.program_headers.as_slice()
    }

    pub fn parse_file(file: &mut File) -> std::result::Result<Metadata, MetadataParseError> {
        use MetadataParseError::*;

        let mut buf = [0; 64];
        if let Err(err) = file.read(&mut buf) {
            return Err(IOError(err));
        }
        let header = match Header::parse_bytes(&buf) {
            Err(err) => return Err(InvalidELF(err)),
            Ok(header) => header,
        };
        let pheader_offset = u64::from(header.program_header_start());
        let pheader_total_size =
            (header.program_header_entry_count() * header.program_header_entry_size()) as usize;
        let mut buf: Vec<_> = std::iter::repeat(0).take(pheader_total_size).collect();
        if let Err(err) = file.seek(SeekFrom::Start(pheader_offset)) {
            return Err(IOError(err));
        }
        if let Err(err) = file.read_exact(buf.as_mut_slice()) {
            return Err(IOError(err));
        }
        let program_headers = Metadata::parse_program_headers(&header, buf.as_slice())?;
        Ok(Metadata::new(header, program_headers))
    }

    fn parse_program_headers(
        header: &Header,
        raw_pheaders: &[u8],
    ) -> Result<Vec<ProgramHeader>, MetadataParseError> {
        use MetadataParseError::*;

        let word_width = header.word_width();
        let endianness = header.endianness();
        (0..header.program_header_entry_count())
            .into_iter()
            .map(|i| {
                let offset = (i * header.program_header_entry_size()) as usize;
                match ProgramHeader::parse_bytes(&raw_pheaders[offset..], word_width, endianness) {
                    Err(err) => Err(InvalidELF(err)),
                    Ok(pheader) => Ok(pheader),
                }
            })
            .collect()
    }
}
