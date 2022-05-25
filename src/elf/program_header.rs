use super::*;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ProgramHeaderSegmentType {
    Null,
    Load,
    Dynamic,
    Interp,
    Note,
    SharedLib,
    HeaderSegment,
    ThreadLocalStorage,
    OSSpecific(u32),
    ProcessorSpecific(u32),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ProgramHeader {
    typ: ProgramHeaderSegmentType,
    flags: u32,
    offset: Word,
    vaddress: Word,
    paddress: Word,
    filesize: Word,
    memsize: Word,
    alignment: Word,
}

impl ProgramHeaderSegmentType {
    fn parse_u32(u: u32) -> Result<ProgramHeaderSegmentType> {
        use ProgramHeaderSegmentType::*;
        match u {
            0x00000000 => Ok(Null),
            0x00000001 => Ok(Load),
            0x00000002 => Ok(Dynamic),
            0x00000003 => Ok(Interp),
            0x00000004 => Ok(Note),
            0x00000005 => Ok(SharedLib),
            0x00000006 => Ok(HeaderSegment),
            0x00000007 => Ok(ThreadLocalStorage),
            i if 0x60000000 <= i && i <= 0x6FFFFFFF => Ok(OSSpecific(i)),
            i if 0x70000000 <= i && i <= 0x7FFFFFFF => Ok(ProcessorSpecific(i)),
            _ => Err(ParseError::InvalidProgHeaderType(u)),
        }
    }

    pub(crate) fn parse_bytes(
        bytes: &[u8],
        endianness: Endianness,
    ) -> Result<ProgramHeaderSegmentType> {
        if bytes.len() < 4 {
            Err(ParseError::InsufficientPartLength(bytes.len()))
        } else {
            ProgramHeaderSegmentType::parse_u32(u32::from_bytes(bytes, endianness))
        }
    }
}

#[cfg(test)]
impl ProgramHeader {
    pub(crate) const fn new(
        typ: ProgramHeaderSegmentType,
        offset: Word,
        vaddress: Word,
        paddress: Word,
        filesize: Word,
        memsize: Word,
        flags: u32,
        alignment: Word,
    ) -> ProgramHeader {
        ProgramHeader {
            typ,
            offset,
            vaddress,
            paddress,
            filesize,
            memsize,
            flags,
            alignment,
        }
    }
}

impl ProgramHeader {
    pub(crate) fn parse_bytes(
        bytes: &[u8],
        word_width: WordWidth,
        endianness: Endianness,
    ) -> Result<ProgramHeader> {
        ProgramHeader::check_length(32, bytes.len())?;
        let typ = ProgramHeaderSegmentType::parse_bytes(bytes, endianness)?;
        // these are the word width dependent offsets of the fields:
        // [offset, vaddress, paddress, filesize, memsize, flags, alignment]
        let (offsets, size) = match word_width {
            WordWidth::Width32 => ([4, 8, 12, 16, 20, 24, 28], 32),
            WordWidth::Width64 => ([8, 16, 24, 32, 40, 4, 48], 56),
        };
        ProgramHeader::check_length(size, bytes.len())?;
        let offset = Word::parse_bytes(&bytes[offsets[0]..], word_width, endianness)?;
        let vaddress = Word::parse_bytes(&bytes[offsets[1]..], word_width, endianness)?;
        let paddress = Word::parse_bytes(&bytes[offsets[2]..], word_width, endianness)?;
        let filesize = Word::parse_bytes(&bytes[offsets[3]..], word_width, endianness)?;
        let memsize = Word::parse_bytes(&bytes[offsets[4]..], word_width, endianness)?;
        let flags = u32::from_bytes(&bytes[offsets[5]..], endianness);
        let alignment = Word::parse_bytes(&bytes[offsets[6]..], word_width, endianness)?;
        ProgramHeader::validate_vaddr(offset, vaddress, alignment)?;
        Ok(ProgramHeader {
            typ,
            flags,
            offset,
            vaddress,
            paddress,
            filesize,
            memsize,
            alignment,
        })
    }

    fn check_length(expected: usize, actual: usize) -> Result<()> {
        if actual < expected {
            Err(ParseError::InvalidProgHeaderLength(actual))
        } else {
            Ok(())
        }
    }

    fn validate_vaddr(offset: Word, addr: Word, align: Word) -> Result<()> {
        let align = match align {
            Word::Word64(u) => u,
            Word::Word32(u) => u as u64,
        };
        if align <= 1 {
            Ok(())
        } else if !align.is_power_of_two() {
            Err(ParseError::InvalidAlignment(align))
        } else {
            let offset = match offset {
                Word::Word64(u) => u,
                Word::Word32(u) => u as u64,
            };
            let normalized_addr = match addr {
                Word::Word64(u) => u,
                Word::Word32(u) => u as u64,
            };
            if normalized_addr % align == offset % align {
                Ok(())
            } else {
                println!("Welp, RIP");
                println!(
                    "normalized_addr: {}, offset: {}, align: {}",
                    normalized_addr, offset, align
                );
                Err(ParseError::InvalidVirtualAddress(addr))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pheader_segment_type_ok() {
        use ProgramHeaderSegmentType::*;
        let test_data = [
            (0x00000000, Null),
            (0x00000001, Load),
            (0x00000002, Dynamic),
            (0x00000003, Interp),
            (0x00000004, Note),
            (0x00000005, SharedLib),
            (0x00000006, HeaderSegment),
            (0x00000007, ThreadLocalStorage),
            (0x60000000, OSSpecific(0x60000000)),
            (0x6FFFFFFF, OSSpecific(0x6FFFFFFF)),
            (0x6F000F00, OSSpecific(0x6F000F00)),
            (0x70000000, ProcessorSpecific(0x70000000)),
            (0x7FFFFFFF, ProcessorSpecific(0x7FFFFFFF)),
            (0x7F000F00, ProcessorSpecific(0x7F000F00)),
        ];

        for (num, expected) in test_data.iter() {
            let bytes = u32::to_le_bytes(*num);
            let result = ProgramHeaderSegmentType::parse_bytes(&bytes, Endianness::Little);
            assert_eq!(result, Ok(*expected));
        }
    }

    #[test]
    fn test_pheader_segment_type_err() {
        use ParseError::InvalidProgHeaderType;
        let test_data = [0x00000008, 0x80000000];
        for num in test_data.iter() {
            let bytes = u32::to_le_bytes(*num);
            let result = ProgramHeaderSegmentType::parse_bytes(&bytes, Endianness::Little);
            assert_eq!(result, Err(InvalidProgHeaderType(*num)));
        }
    }

    static VALID_PHEADER_DATA_32_LITTLE: [u8; 32] = [
        // segment type
        0x06, 0x00, 0x00, 0x00, // header segment
        // offset
        0x38, 0x00, 0x00, 0x00, // virtual address
        0x00, 0x00, 0x5C, 0x44, // physical address
        0x00, 0x00, 0x00, 0x00, // filesize
        0x00, 0x00, 0x00, 0x00, // memsize
        0x00, 0x00, 0x00, 0x00, // flags
        0xE3, 0x77, 0x04, 0xF1, // alignment
        0x08, 0x00, 0x00, 0x00,
    ];

    static VALID_PHEADER_DATA_64_LITTLE: [u8; 56] = [
        // segment type
        0x06, 0x00, 0x00, 0x00, // header segment
        // flags
        0xE3, 0x77, 0x04, 0xF1, // offset
        0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // virtual address
        0x00, 0x00, 0x5C, 0x44, 0x00, 0x00, 0x22, 0x00, // physical address
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // filesize
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // memsize
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xF3, // alignment
        0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    static VALID_PHEADER_32: ProgramHeader = ProgramHeader::new(
        ProgramHeaderSegmentType::HeaderSegment,
        Word::Word32(0x00000038),
        Word::Word32(0x445C0000),
        Word::Word32(0x00000000),
        Word::Word32(0x00000000),
        Word::Word32(0x00000000),
        0xF10477E3,
        Word::Word32(0x00000008),
    );

    static VALID_PHEADER_64: ProgramHeader = ProgramHeader::new(
        ProgramHeaderSegmentType::HeaderSegment,
        Word::Word64(0x0000000000000040),
        Word::Word64(0x00220000445C0000),
        Word::Word64(0x0000000000000000),
        Word::Word64(0x0000000000000000),
        Word::Word64(0xF300000000000000),
        0xF10477E3,
        Word::Word64(0x0000000000000008),
    );

    #[test]
    fn test_pheader_32_ok() {
        let test_data = VALID_PHEADER_DATA_32_LITTLE.clone();
        let result = ProgramHeader::parse_bytes(&test_data, WordWidth::Width32, Endianness::Little);
        assert_eq!(result, Ok(VALID_PHEADER_32.clone()));
    }

    #[test]
    fn test_pheader_64_ok() {
        let test_data = VALID_PHEADER_DATA_64_LITTLE.clone();
        let result = ProgramHeader::parse_bytes(&test_data, WordWidth::Width64, Endianness::Little);
        assert_eq!(result, Ok(VALID_PHEADER_64.clone()));
    }

    #[test]
    fn test_pheader_err_type() {
        let mut test_data = VALID_PHEADER_DATA_32_LITTLE.clone();
        test_data[0] = 0x08;
        let result = ProgramHeader::parse_bytes(&test_data, WordWidth::Width32, Endianness::Little);
        assert_eq!(result, Err(ParseError::InvalidProgHeaderType(0x00000008)));
    }

    #[test]
    fn test_pheader_err_align() {
        let mut test_data = VALID_PHEADER_DATA_32_LITTLE.clone();
        test_data[28] = 0x0F;
        let result = ProgramHeader::parse_bytes(&test_data, WordWidth::Width32, Endianness::Little);
        assert_eq!(
            result,
            Err(ParseError::InvalidAlignment(0x000000000000000F))
        );
    }

    #[test]
    fn test_pheader_err_addr() {
        let mut test_data = VALID_PHEADER_DATA_32_LITTLE.clone();
        test_data[8] = 0x01;
        let result = ProgramHeader::parse_bytes(&test_data, WordWidth::Width32, Endianness::Little);
        assert_eq!(
            result,
            Err(ParseError::InvalidVirtualAddress(Word::Word32(0x445C0001)))
        );
    }
}
