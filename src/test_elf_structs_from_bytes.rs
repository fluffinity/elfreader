
#[cfg(test)]
mod test {
    use crate::elf::*;

    #[test]
    fn test_file_type_ok() {
        let test_data = [
            ([0x00, 0x00], Endianness::Little, FileType::None),
            ([0x00, 0x00], Endianness::Big, FileType::None),
            ([0x01, 0x00], Endianness::Little, FileType::Relocatable),
            ([0x00, 0x01], Endianness::Big, FileType::Relocatable),
            ([0x02, 0x00], Endianness::Little, FileType::Executable),
            ([0x00, 0x02], Endianness::Big, FileType::Executable),
            ([0x03, 0x00], Endianness::Little, FileType::Shared),
            ([0x00, 0x03], Endianness::Big, FileType::Shared),
            ([0x04, 0x00], Endianness::Little, FileType::Core),
            ([0x00, 0x04], Endianness::Big, FileType::Core),
            ([0x9D, 0xFF], Endianness::Little, FileType::Specific(0xFF9D)),
            ([0xFF, 0x9D], Endianness::Big, FileType::Specific(0xFF9D))
        ];
        for (data, endianness, expected) in test_data.iter() {
            assert_eq!(FileType::parse_bytes(data, *endianness), Ok(*expected));
        }
    }

    #[test]
    fn test_file_type_err() {
        let test_data = [
            ([0xE3, 0xE3], Endianness::Little, ParseError::InvalidFileType(0xE3E3)),
            ([0xFF, 0xFE], Endianness::Little, ParseError::InvalidFileType(0xFEFF)),
            ([0xFE, 0xFF], Endianness::Big, ParseError::InvalidFileType(0xFEFF))
        ];
        for (data, endianness, expected) in test_data.iter() {
            assert_eq!(FileType::parse_bytes(data, *endianness), Err(*expected));
        }
    }

    #[test]
    fn test_word_width_ok() {
        let test_data = [
            (0x01, WordWidth::Width32),
            (0x02, WordWidth::Width64)
        ];
        for (byte, expected) in test_data.iter() {
            assert_eq!(WordWidth::parse_byte(*byte), Ok(*expected));
        }
    }

    #[test]
    fn test_word_width_err() {
        let test_data = [0x00, 0x03, 0xFF, 0x3D];
        for &byte in test_data.iter() {
            assert_eq!(WordWidth::parse_byte(byte), Err(ParseError::InvalidWordWidth(byte)));
        }
    }

    #[test]
    fn test_endianness_ok() {
        let test_data = [
            (0x01, Endianness::Little),
            (0x02, Endianness::Big)
        ];
        for (byte, expected) in test_data.iter() {
            assert_eq!(Endianness::parse_byte(*byte), Ok(*expected));
        }
    }

    #[test]
    fn test_endianness_err() {
        let test_data = [0x00, 0x03, 0xFF, 0x3D];
        for &byte in test_data.iter() {
            assert_eq!(Endianness::parse_byte(byte), Err(ParseError::InvalidEndianness(byte)));
        }
    }

    #[test]
    fn test_abi() {
        use Abi::*;
        let test_data = [
            SysV,
            HpUx,
            NetBSD,
            Linux,
            GnuHurd,
            Unknown,
            Solaris,
            Aix,
            Irix,
            FreeBSD,
            Tru64,
            NovellModesto,
            OpenBSD,
            OpenVMS,
            NonStopKernel,
            Aros,
            FenixOS,
            CloudABI,
            OpenVOS,
            Unknown
        ];
        for (i, expected) in test_data.iter().enumerate() {
            assert_eq!(Abi::from_byte(i as u8), *expected);
        }
    }

    #[test]
    fn test_arch_ok() {
        use Arch::*;
        let test_data = [
            (0x0000_u16, Unspecified),
            (0x0001, WE32100),
            (0x0002, Sparc),
            (0x0003, X86),
            (0x0004, M68k),
            (0x0005, M88k),
            (0x0006, IntelMCU),
            (0x0007, Intel80860),
            (0x0008, MIPS),
            (0x0009, System370),
            (0x000A, RS3000),
            (0x000E, PARISC),
            (0x0013, Intel80960),
            (0x0014, PowerPC),
            (0x0015, PowerPC64),
            (0x0016, S390),
            (0x0028, ARM),
            (0x002A, SuperH),
            (0x0032, IA64),
            (0x003E, X86_64),
            (0x008C, TMS320C6000),
            (0x00B7, AArch64),
            (0x00F3, RISCV),
            (0x00F7, BPF),
            (0x0101, WDC65C816),
            (0x0102, Unknown),
            (0xFFFF, Unknown)
        ];
        for (code, expected) in test_data.iter() {
            let bytes = code.to_le_bytes();
            assert_eq!(Arch::parse_bytes(&bytes, Endianness::Little), Ok(*expected));
        }
    }

    #[test]
    fn test_arch_err() {
        let test_data = [0x01_u8];
        assert_eq!(Arch::parse_bytes(&test_data, Endianness::Little), Err(ParseError::InsufficientPartLength(1)));
        assert_eq!(Arch::parse_bytes(&test_data, Endianness::Big), Err(ParseError::InsufficientPartLength(1)));
    }

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
            (0x7F000F00, ProcessorSpecific(0x7F000F00))
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
        let test_data = [
            0x00000008,
            0x80000000
        ];
        for num in test_data.iter() {
            let bytes = u32::to_le_bytes(*num);
            let result = ProgramHeaderSegmentType::parse_bytes(&bytes, Endianness::Little);
            assert_eq!(result, Err(InvalidProgHeaderType(*num)));
        }
    }

    #[test]
    fn test_word_ok() {
        use Endianness::*;
        use WordWidth::*;
        use Word::*;
        let test_data = [
            ([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], Width64, Little, Word64(0)),
            ([0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], Width32, Little, Word32(0x00000010)),
            ([0xFF, 0x3E, 0x00, 0x00, 0x10, 0x20, 0x00, 0x00], Width64, Big, Word64(0xFF3E000010200000)),
            ([0xFF, 0x3E, 0x00, 0x00, 0x10, 0x20, 0x00, 0x00], Width32, Big, Word32(0xFF3E0000))
        ];

        for (bytes, width, endianness, expected) in test_data.iter() {
            let result = Word::parse_bytes(bytes, *width, *endianness);
            assert!(result.is_ok());
            let result = result.expect("checked is_ok()");
            assert_eq!(result , *expected);
        }
    }

    #[test]
    fn test_valid_magic() {
        let test_data  = [
            ([0x7F, 0x45, 0x4C, 0x46], true),
            ([0x7E, 0x45, 0x4C, 0x46], false),
            ([0x7F, 0x46, 0x4C, 0x46], false)
        ];
        for (bytes, expected) in test_data.iter() {
            assert_eq!(Header::is_valid_magic(*bytes), *expected);
        }
    }

    static VALID_HEADER_DATA_32: [u8;52] = [
        // magic
        0x7F, 0x45, 0x4C, 0x46,
        // word width
        0x01, // 32-bit
        // endianness
        0x01, // little
        // header version
        0x01,
        // OS ABI
        0x03, // Linux
        // ABI version
        0x01,
        // padding
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        // file type
        0x02, 0x00, // executable
        // arch
        0x3E, 0x00, // x86_64
        // version
        0x01, 0xFF, 0x00, 0x00,
        // entry point
        0x00, 0x00, 0x00, 0xF0,
        // program header start
        0x34, 0x00, 0x00, 0x00,
        // section header start
        0x00, 0x00, 0x30, 0x00,
        // flags
        0x4F, 0xF1, 0x97, 0xC4,
        // header size
        0x34, 0x00,
        // program header entry size
        0x20, 0x00,
        // program header entry count
        0x01, 0x00,
        // section header entry size
        0x10, 0x00,
        // section header entry count
        0x00, 0x01,
        // section names index
        0x30, 0x00
    ];

    static VALID_HEADER_DATA_64: [u8;64] = [
        // magic
        0x7F, 0x45, 0x4C, 0x46,
        // word width
        0x02, // 64-bit
        // endianness
        0x01, // little
        // header version
        0x01,
        // OS ABI
        0x03, // Linux
        // ABI version
        0x01,
        // padding
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        // file type
        0x02, 0x00, // executable
        // arch
        0x3E, 0x00, // x86_64
        // version
        0x01, 0xFF, 0x00, 0x00,
        // entry point
        0x00, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00,
        // program header start
        0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        // section header start
        0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00,
        // flags
        0xFF, 0x3A, 0x10, 0x57,
        // header size
        0x40, 0x00,
        // program header entry size
        0x38, 0x00,
        // program header entry count
        0x01, 0x00,
        // section header entry size
        0x00, 0x80,
        // section header entry count
        0x40, 0x00,
        // section names index
        0x2F, 0x00
    ];

    static VALID_HEADER_32: Header = Header::minimal(WordWidth::Width32, Endianness::Little)
        .with_header_version(0x01)
        .with_abi(Abi::Linux)
        .with_abi_version(0x01)
        .with_file_type(FileType::Executable)
        .with_arch(Arch::X86_64)
        .with_version(0x0000FF01)
        .with_entry_point(Word::Word32(0xF0000000))
        .with_program_header_start(Word::Word32(0x00000034))
        .with_section_header_start(Word::Word32(0x00300000))
        .with_flags(0xC497F14F)
        .with_program_header_entry_size(0x0020)
        .with_program_header_entry_count(0x0001)
        .with_section_header_entry_size(0x0010)
        .with_section_header_entry_count(0x0100)
        .with_section_names_index(0x0030);

    static VALID_HEADER_64: Header = Header::minimal(WordWidth::Width64, Endianness::Little)
        .with_header_version(0x01)
        .with_abi(Abi::Linux)
        .with_abi_version(0x01)
        .with_file_type(FileType::Executable)
        .with_arch(Arch::X86_64)
        .with_version(0x0000FF01)
        .with_entry_point(Word::Word64(0x0000008000000000))
        .with_program_header_start(Word::Word64(0x0000000000000040))
        .with_section_header_start(Word::Word64(0x0000000080000000))
        .with_flags(0x57103AFF)
        .with_program_header_entry_size(0x0038)
        .with_program_header_entry_count(0x0001)
        .with_section_header_entry_size(0x8000)
        .with_section_header_entry_count(0x0040)
        .with_section_names_index(0x002F);

    #[test]
    fn test_header_32_ok() {
        let test_data = VALID_HEADER_DATA_32;
        let result = Header::parse_bytes(&test_data);
        assert_eq!(result, Ok(VALID_HEADER_32.clone()));
    }

    #[test]
    fn test_header_64_ok() {
        let test_data = VALID_HEADER_DATA_64;
        let result = Header::parse_bytes(&test_data);
        assert_eq!(result, Ok(VALID_HEADER_64.clone()));
    }

    #[test]
    fn test_header_err_slice_len() {
        let test_data = [];
        let result = Header::parse_bytes(&test_data);
        assert_eq!(result, Err(ParseError::InvalidHeaderLength(0)));
    }

    #[test]
    fn test_header_err_word32_len() {
        let test_data = &VALID_HEADER_DATA_32[..50];
        let result = Header::parse_bytes(test_data);
        assert_eq!(result, Err(ParseError::InvalidHeaderLength(50)));
    }

    #[test]
    fn test_header_err_word64_len() {
        let test_data = &VALID_HEADER_DATA_64[..55];
        let result = Header::parse_bytes(test_data);
        assert_eq!(result, Err(ParseError::InvalidHeaderLength(55)));
    }

    #[test]
    fn test_header_err_magic() {
        let mut test_data = VALID_HEADER_DATA_64.clone();
        test_data[2] = 0x4D;
        let result = Header::parse_bytes(&test_data);
        assert_eq!(result, Err(ParseError::NoELF(u32::to_le(0x464D457F))));
    }

    #[test]
    fn test_header_err_word_width() {
        let mut test_data = VALID_HEADER_DATA_64.clone();
        test_data[4] = 0x03;
        let result = Header::parse_bytes(&test_data);
        assert_eq!(result, Err(ParseError::InvalidWordWidth(0x03)));
    }

    #[test]
    fn test_header_err_endianness() {
        let mut test_data = VALID_HEADER_DATA_64.clone();
        test_data[5] = 0xFF;
        let result = Header::parse_bytes(&test_data);
        assert_eq!(result, Err(ParseError::InvalidEndianness(0xFF)));
    }

    #[test]
    fn test_header_err_file_type() {
        let mut test_data = VALID_HEADER_DATA_64.clone();
        test_data[16] = 0x69;
        test_data[17] = 0x42;
        let result = Header::parse_bytes(&test_data);
        assert_eq!(result, Err(ParseError::InvalidFileType(u16::to_le(0x4269))));
    }

    static VALID_PHEADER_DATA_32_LITTLE: [u8;32] = [
        // segment type
        0x06, 0x00, 0x00, 0x00, // header segment
        // offset
        0x34, 0x00, 0x00, 0x00,
        // virtual address
        0x00, 0x00, 0x5C, 0x44,
        // physical address
        0x00, 0x00, 0x00, 0x00,
        // filesize
        0x00, 0x00, 0x00, 0x00,
        // memsize
        0x00, 0x00, 0x00, 0x00,
        // flags
        0xE3, 0x77, 0x04, 0xF1,
        // alignment
        0x01, 0x00, 0x00, 0x00
    ];

    static VALID_PHEADER_DATA_64_LITTLE: [u8; 56] = [
        // segment type
        0x06, 0x00, 0x00, 0x00, // header segment
        // flags
        0xE3, 0x77, 0x04, 0xF1,
        // offset
        0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        // virtual address
        0x00, 0x00, 0x5C, 0x44, 0x00, 0x00, 0x22, 0x00,
        // physical address
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        // filesize
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        // memsize
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xF3,
        // alignment
        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
    ];

    static VALID_PHEADER_32: ProgramHeader = ProgramHeader::new(
        ProgramHeaderSegmentType::HeaderSegment,
        Word::Word32(0x00000034),
        Word::Word32(0x445C0000),
        Word::Word32(0x00000000),
        Word::Word32(0x00000000),
        Word::Word32(0x00000000),
        0xF10477E3,
        Word::Word32(0x00000001)
    );

    static VALID_PHEADER_64: ProgramHeader = ProgramHeader::new(
        ProgramHeaderSegmentType::HeaderSegment,
        Word::Word64(0x0000000000000040),
        Word::Word64(0x00220000445C0000),
        Word::Word64(0x0000000000000000),
        Word::Word64(0x0000000000000000),
        Word::Word64(0xF300000000000000),
        0xF10477E3,
        Word::Word64(0x0000000000000001)
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
        assert_eq!(result, Err(ParseError::InvalidAlignment(0x000000000000000F)));
    }

    #[test]
    fn test_pheader_err_addr() {
        let mut test_data = VALID_PHEADER_DATA_32_LITTLE.clone();
        test_data[28] = 0x02;
        let result = ProgramHeader::parse_bytes(&test_data, WordWidth::Width32, Endianness::Little);
        assert_eq!(result, Err(ParseError::InvalidVirtualAddress(Word::Word32(0x445C0000))));
    }
}