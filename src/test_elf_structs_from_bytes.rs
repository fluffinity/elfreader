
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
            (0x01, ELFWordWidth::Word32),
            (0x02, ELFWordWidth::Word64)
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

    static VALID_HEADER_DATA: [u8;24] = [
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
        0x01, 0xFF, 0x00, 0x00
    ];

    static VALID_HEADER: ELFHeader = ELFHeader::new(
        ELFWordWidth::Word64,
        ELFEndianness::Little,
        0x01,
        ElfAbi::Linux,
        0x01,
        ELFFileType::Executable,
        ELFArch::X86_64,
        0x0000FF01
    );

    #[test]
    fn test_header_ok() {
        let test_data = VALID_HEADER_DATA;
        let result = ELFHeader::from_bytes(&test_data);
        assert!(result.is_ok());
        let header = result.expect("asserted is_ok()");
        let expected = VALID_HEADER.clone();
        assert_eq!(header, expected);
    }

    #[test]
    fn test_header_err_slice_len() {
        let test_data = [];
        let result = ELFHeader::from_bytes(&test_data);
        assert_eq!(result, Err(ELFParseError::InvalidHeaderLength(0)));
    }

    #[test]
    fn test_header_err_magic() {
        let mut test_data = VALID_HEADER_DATA.clone();
        test_data[2] = 0x4D;
        let result = ELFHeader::from_bytes(&test_data);
        assert_eq!(result, Err(ELFParseError::NoELF(u32::to_le(0x464D457F))));
    }

    #[test]
    fn test_header_err_word_width() {
        let mut test_data = VALID_HEADER_DATA.clone();
        test_data[4] = 0x03;
        let result = ELFHeader::from_bytes(&test_data);
        assert_eq!(result, Err(ELFParseError::InvalidWordWidth(0x03)));
    }

    #[test]
    fn test_header_err_endianness() {
        let mut test_data = VALID_HEADER_DATA.clone();
        test_data[5] = 0xFF;
        let result = ELFHeader::from_bytes(&test_data);
        assert_eq!(result, Err(ELFParseError::InvalidEndianness(0xFF)));
    }

    #[test]
    fn test_header_err_file_type() {
        let mut test_data = VALID_HEADER_DATA.clone();
        test_data[16] = 0x69;
        test_data[17] = 0x42;
        let result = ELFHeader::from_bytes(&test_data);
        assert_eq!(result, Err(ELFParseError::InvalidFileType(u16::to_le(0x4269))));
    }
}