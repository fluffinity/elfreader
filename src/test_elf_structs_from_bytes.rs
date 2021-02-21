
#[cfg(test)]
mod test {
    use crate::elf::*;

    #[test]
    fn test_file_type_from_bytes_ok() {
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
    fn test_file_type_from_bytes_err() {
        let test_data = [
            ([0xE3, 0xE3], ELFEndianness::Little, ELFParseError::InvalidFileType(0xE3E3)),
            ([0xFF, 0xFE], ELFEndianness::Little, ELFParseError::InvalidFileType(0xFEFF)),
            ([0xFE, 0xFF], ELFEndianness::Big, ELFParseError::InvalidFileType(0xFEFF))
        ];
        for (data, endianness, expected) in test_data.iter() {
            assert_eq!(ELFFileType::from_bytes(data, *endianness), Err(*expected));
        }
    }
}