
#[cfg(test)]
mod test {
    use crate::elf::*;

    #[test]
    fn test_file_type_ok() {
        let test_data = [
            ([0x00, 0x00], ELFEndianness::Little, ELFFileType::None),
            ([0x00, 0x00], ELFEndianness::Big, ELFFileType::None),
            ([0x01, 0x00], ELFEndianness::Little, ELFFileType::Relocatable),
            ([0x00, 0x01], ELFEndianness::Big, ELFFileType::Relocatable),
            ([0x02, 0x00], ELFEndianness::Little, ELFFileType::Executable),
            ([0x00, 0x02], ELFEndianness::Big, ELFFileType::Executable),
            ([0x03, 0x00], ELFEndianness::Little, ELFFileType::Shared),
            ([0x00, 0x03], ELFEndianness::Big, ELFFileType::Shared),
            ([0x04, 0x00], ELFEndianness::Little, ELFFileType::Core),
            ([0x00, 0x04], ELFEndianness::Big, ELFFileType::Core),
            ([0x9D, 0xFF], ELFEndianness::Little, ELFFileType::Specific(0xFF9D)),
            ([0xFF, 0x9D], ELFEndianness::Big, ELFFileType::Specific(0xFF9D))
        ];
        for (data, endianness, expected) in test_data.iter() {
            assert_eq!(ELFFileType::from_bytes(data, *endianness), Ok(*expected));
        }
    }

    #[test]
    fn test_file_type_err() {
        let test_data = [
            ([0xE3, 0xE3], ELFEndianness::Little, ELFParseError::InvalidFileType(0xE3E3)),
            ([0xFF, 0xFE], ELFEndianness::Little, ELFParseError::InvalidFileType(0xFEFF)),
            ([0xFE, 0xFF], ELFEndianness::Big, ELFParseError::InvalidFileType(0xFEFF))
        ];
        for (data, endianness, expected) in test_data.iter() {
            assert_eq!(ELFFileType::from_bytes(data, *endianness), Err(*expected));
        }
    }

    #[test]
    fn test_word_width_ok() {
        let test_data = [
            (0x01, ELFWordWidth::Width32),
            (0x02, ELFWordWidth::Width64)
        ];
        for (byte, expected) in test_data.iter() {
            assert_eq!(ELFWordWidth::from_byte(*byte), Ok(*expected));
        }
    }

    #[test]
    fn test_word_width_err() {
        let test_data = [0x00, 0x03, 0xFF, 0x3D];
        for &byte in test_data.iter() {
            assert_eq!(ELFWordWidth::from_byte(byte), Err(ELFParseError::InvalidWordWidth(byte)));
        }
    }

    #[test]
    fn test_endianness_ok() {
        let test_data = [
            (0x01, ELFEndianness::Little),
            (0x02, ELFEndianness::Big)
        ];
        for (byte, expected) in test_data.iter() {
            assert_eq!(ELFEndianness::from_byte(*byte), Ok(*expected));
        }
    }

    #[test]
    fn test_endianness_err() {
        let test_data = [0x00, 0x03, 0xFF, 0x3D];
        for &byte in test_data.iter() {
            assert_eq!(ELFEndianness::from_byte(byte), Err(ELFParseError::InvalidEndianness(byte)));
        }
    }

    #[test]
    fn test_abi() {
        use ElfAbi::*;
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
            assert_eq!(ElfAbi::from_byte(i as u8), *expected);
        }
    }

    #[test]
    fn test_arch_ok() {
        use ELFArch::*;
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
            assert_eq!(ELFArch::from_bytes(&bytes, ELFEndianness::Little), Ok(*expected));
        }
    }

    #[test]
    fn test_arch_err() {
        let test_data = [0x01_u8];
        assert_eq!(ELFArch::from_bytes(&test_data, ELFEndianness::Little), Err(ELFParseError::InsufficientPartLength(1)));
        assert_eq!(ELFArch::from_bytes(&test_data, ELFEndianness::Big), Err(ELFParseError::InsufficientPartLength(1)));
    }

    #[test]
    fn test_word_ok() {
        use ELFEndianness::*;
        use ELFWordWidth::*;
        use ELFWord::*;
        let test_data = [
            ([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], Width64, Little, Word64(0)),
            ([0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], Width32, Little, Word32(0x00000010)),
            ([0xFF, 0x3E, 0x00, 0x00, 0x10, 0x20, 0x00, 0x00], Width64, Big, Word64(0xFF3E000010200000)),
            ([0xFF, 0x3E, 0x00, 0x00, 0x10, 0x20, 0x00, 0x00], Width32, Big, Word32(0xFF3E0000))
        ];

        for (bytes, width, endianness, expected) in test_data.iter() {
            let result = ELFWord::from_bytes(bytes, *width, *endianness);
            assert!(result.is_ok());
            let (result, _) = result.expect("checked is_ok()");
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
            assert_eq!(is_valid_magic(*bytes), *expected);
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
        0x00, 0x00, 0x64, 0x00,
        // section header start
        0x00, 0x00, 0x30, 0x00,
        // flags
        0x4F, 0xF1, 0x97, 0xC4,
        // header size
        0x34, 0x00,
        // program header entry size
        0x20, 0x00,
        // program header entry count
        0x40, 0x00,
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
        0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00,
        // section header start
        0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00,
        // flags
        0xFF, 0x3A, 0x10, 0x57,
        // header size
        0x40, 0x00,
        // program header entry size
        0x40, 0x00,
        // program header entry count
        0x30, 0x00,
        // section header entry size
        0x00, 0x80,
        // section header entry count
        0x40, 0x00,
        // section names index
        0x2F, 0x00
    ];

    static VALID_HEADER_32: ELFHeader = ELFHeader::minimal(ELFWordWidth::Width32, ELFEndianness::Little)
        .header_version(0x01)
        .abi(ElfAbi::Linux)
        .abi_version(0x01)
        .file_type(ELFFileType::Executable)
        .arch(ELFArch::X86_64)
        .version(0x0000FF01)
        .entry_point(ELFWord::Word32(0xF0000000))
        .program_header_start(ELFWord::Word32(0x00640000))
        .section_header_start(ELFWord::Word32(0x00300000))
        .flags(0xC497F14F)
        .program_header_entry_size(0x0020)
        .program_header_entry_count(0x0040)
        .section_header_entry_size(0x0010)
        .section_header_entry_count(0x0100)
        .section_names_index(0x0030);

    static VALID_HEADER_64: ELFHeader = ELFHeader::minimal(ELFWordWidth::Width64, ELFEndianness::Little)
        .header_version(0x01)
        .abi(ElfAbi::Linux)
        .abi_version(0x01)
        .file_type(ELFFileType::Executable)
        .arch(ELFArch::X86_64)
        .version(0x0000FF01)
        .entry_point(ELFWord::Word64(0x0000008000000000))
        .program_header_start(ELFWord::Word64(0x0000000040000000))
        .section_header_start(ELFWord::Word64(0x0000000080000000))
        .flags(0x57103AFF)
        .program_header_entry_size(0x0040)
        .program_header_entry_count(0x0030)
        .section_header_entry_size(0x8000)
        .section_header_entry_count(0x0040)
        .section_names_index(0x002F);

    #[test]
    fn test_header_32_ok() {
        let test_data = VALID_HEADER_DATA_32;
        let result = ELFHeader::from_bytes(&test_data);
        assert_eq!(result, Ok(VALID_HEADER_32.clone()));
    }

    #[test]
    fn test_header_64_ok() {
        let test_data = VALID_HEADER_DATA_64;
        let result = ELFHeader::from_bytes(&test_data);
        assert_eq!(result, Ok(VALID_HEADER_64.clone()));
    }

    #[test]
    fn test_header_err_slice_len() {
        let test_data = [];
        let result = ELFHeader::from_bytes(&test_data);
        assert_eq!(result, Err(ELFParseError::InvalidHeaderLength(0)));
    }

    #[test]
    fn test_header_err_word32_len() {
        let test_data = &VALID_HEADER_DATA_32[..50];
        let result = ELFHeader::from_bytes(test_data);
        assert_eq!(result, Err(ELFParseError::InvalidHeaderLength(50)));
    }

    #[test]
    fn test_header_err_word64_len() {
        let test_data = &VALID_HEADER_DATA_64[..55];
        let result = ELFHeader::from_bytes(test_data);
        assert_eq!(result, Err(ELFParseError::InvalidHeaderLength(55)));
    }

    #[test]
    fn test_header_err_magic() {
        let mut test_data = VALID_HEADER_DATA_64.clone();
        test_data[2] = 0x4D;
        let result = ELFHeader::from_bytes(&test_data);
        assert_eq!(result, Err(ELFParseError::NoELF(u32::to_le(0x464D457F))));
    }

    #[test]
    fn test_header_err_word_width() {
        let mut test_data = VALID_HEADER_DATA_64.clone();
        test_data[4] = 0x03;
        let result = ELFHeader::from_bytes(&test_data);
        assert_eq!(result, Err(ELFParseError::InvalidWordWidth(0x03)));
    }

    #[test]
    fn test_header_err_endianness() {
        let mut test_data = VALID_HEADER_DATA_64.clone();
        test_data[5] = 0xFF;
        let result = ELFHeader::from_bytes(&test_data);
        assert_eq!(result, Err(ELFParseError::InvalidEndianness(0xFF)));
    }

    #[test]
    fn test_header_err_file_type() {
        let mut test_data = VALID_HEADER_DATA_64.clone();
        test_data[16] = 0x69;
        test_data[17] = 0x42;
        let result = ELFHeader::from_bytes(&test_data);
        assert_eq!(result, Err(ELFParseError::InvalidFileType(u16::to_le(0x4269))));
    }
}