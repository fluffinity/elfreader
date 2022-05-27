use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use crate::elf::SectionHeaderType;

use super::{Header, ParseError, ProgramHeader, SectionHeader, UnnamedSectionHeader};

pub struct Metadata {
    header: Header,
    program_headers: Vec<ProgramHeader>,
    section_headers: Vec<SectionHeader>,
}

#[derive(Debug)]
pub enum MetadataParseError {
    InvalidELF(ParseError),
    IOError(std::io::Error),
}

impl Metadata {
    pub fn new(
        header: Header,
        program_headers: Vec<ProgramHeader>,
        section_headers: Vec<SectionHeader>,
    ) -> Metadata {
        Metadata {
            header,
            program_headers,
            section_headers,
        }
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn program_headers(&self) -> &[ProgramHeader] {
        self.program_headers.as_slice()
    }

    pub fn section_headers(&self) -> &[SectionHeader] {
        self.section_headers.as_slice()
    }

    pub fn parse_file(file: &mut File) -> std::result::Result<Metadata, MetadataParseError> {
        use MetadataParseError::*;

        let mut buf = [0; 64];
        let status = file.read(&mut buf);
        let header_buf = match status {
            Err(err) => return Err(IOError(err)),
            Ok(n) => &buf[..n],
        };
        let header = match Header::parse_bytes(header_buf) {
            Err(err) => return Err(InvalidELF(err)),
            Ok(header) => header,
        };
        let program_headers = Metadata::parse_program_headers_from_file(&header, file)?;
        let section_headers = Metadata::parse_section_headers_from_file(&header, file)?;

        Ok(Metadata::new(header, program_headers, section_headers))
    }

    fn parse_program_headers_from_file(
        header: &Header,
        file: &mut File,
    ) -> Result<Vec<ProgramHeader>, MetadataParseError> {
        use MetadataParseError::*;

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
        Metadata::parse_program_headers(&header, buf.as_slice())
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

    fn parse_section_headers_from_file(
        header: &Header,
        file: &mut File,
    ) -> Result<Vec<SectionHeader>, MetadataParseError> {
        use MetadataParseError::*;

        let sheader_offset = u64::from(header.section_header_start());
        let sheader_total_size =
            (header.section_header_entry_count() * header.section_header_entry_size()) as usize;
        let mut buf: Vec<_> = std::iter::repeat(0).take(sheader_total_size).collect();
        if let Err(err) = file.seek(SeekFrom::Start(sheader_offset)) {
            return Err(IOError(err));
        }
        if let Err(err) = file.read_exact(buf.as_mut_slice()) {
            return Err(IOError(err));
        }
        let unnamed_section_headers = Metadata::parse_section_headers(&header, buf.as_slice())?;
        Metadata::parse_named_section_headers_from_file(header, unnamed_section_headers, file)
    }

    fn parse_named_section_headers_from_file(
        header: &Header,
        unnamed_section_headers: Vec<UnnamedSectionHeader>,
        file: &mut File,
    ) -> Result<Vec<SectionHeader>, MetadataParseError> {
        use MetadataParseError::*;

        let (name_table_offset, name_table_length) =
            match unnamed_section_headers.get(header.section_names_index() as usize) {
                None => return Ok(Vec::new()),
                Some(sheader) => {
                    if sheader.typ() != SectionHeaderType::StringTable {
                        return Err(InvalidELF(ParseError::InvalidSectionNameTableType(
                            sheader.typ(),
                        )));
                    }
                    (
                        u64::from(sheader.offset()),
                        u64::from(sheader.size()) as usize,
                    )
                }
            };
        if let Err(err) = file.seek(SeekFrom::Start(name_table_offset)) {
            return Err(IOError(err));
        }
        let mut buf: Vec<_> = std::iter::repeat(0).take(name_table_length).collect();
        if let Err(err) = file.read_exact(buf.as_mut_slice()) {
            return Err(IOError(err));
        }
        let section_headers: Result<Vec<_>, ParseError> = unnamed_section_headers
            .into_iter()
            .map(|header| header.to_named(buf.as_slice()))
            .collect();
        section_headers.map_err(|err| MetadataParseError::InvalidELF(err))
    }

    fn parse_section_headers(
        header: &Header,
        raw_sheaders: &[u8],
    ) -> Result<Vec<UnnamedSectionHeader>, MetadataParseError> {
        use MetadataParseError::*;

        let word_width = header.word_width();
        let endianness = header.endianness();
        (0..header.section_header_entry_count())
            .into_iter()
            .map(|i| {
                let offset = (i * header.section_header_entry_size()) as usize;
                match UnnamedSectionHeader::parse_bytes(
                    &raw_sheaders[offset..],
                    word_width,
                    endianness,
                ) {
                    Err(err) => Err(InvalidELF(err)),
                    Ok(sheader) => Ok(sheader),
                }
            })
            .collect()
    }
}
