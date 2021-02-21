#[cfg(test)]
mod test {

    use crate::elf::*;

    #[test]
    fn test_from_bytes_u16_little_zero() {
        let test_data = [0x00, 0x00];
        assert_eq!(u16::from_bytes(&test_data, ELFEndianness::Little), 0);
    }

    #[test]
    fn test_from_bytes_u16_big_zero() {
        let test_data = [0x00, 0x00];
        assert_eq!(u16::from_bytes(&test_data, ELFEndianness::Big), 0);
    }

    #[test]
    fn test_from_bytes_u16_little_single_byte() {
        let test_data = [
            ([0x01, 0x00], 0x0001),
            ([0x10, 0x00], 0x0010),
            ([0xFF, 0x00], 0x00FF)
        ];
        for (data, expected) in test_data.iter() {
            assert_eq!(u16::from_bytes(data, ELFEndianness::Little), *expected);
        }
    }

    #[test]
    fn test_from_bytes_u16_big_single_byte() {
        let test_data = [
            ([0x00, 0x01], 0x0001),
            ([0x00, 0x10], 0x0010),
            ([0x00, 0xFF], 0x00FF)
        ];
        for (data, expected) in test_data.iter() {
            assert_eq!(u16::from_bytes(data, ELFEndianness::Big), *expected);
        }
    }

    #[test]
    fn test_from_bytes_u16_little_multi_byte() {
        let test_data = [
            ([0x00, 0x01], 0x0100),
            ([0x10, 0x01], 0x0110),
            ([0xFF, 0xE3], 0xE3FF)
        ];
        for (data, expected) in test_data.iter() {
            assert_eq!(u16::from_bytes(data, ELFEndianness::Little), *expected);
        }
    }

    #[test]
    fn test_from_bytes_u16_big_multi_byte() {
        let test_data = [
            ([0x10, 0x00], 0x1000),
            ([0x01, 0x10], 0x0110),
            ([0xE3, 0xFF], 0xE3FF)
        ];
        for (data, expected) in test_data.iter() {
            assert_eq!(u16::from_bytes(data, ELFEndianness::Big), *expected);
        }
    }

    #[test]
    fn test_from_bytes_u32_little_zero() {
        let test_data = [0x00, 0x00, 0x00, 0x00];
        assert_eq!(u32::from_bytes(&test_data, ELFEndianness::Little), 0);
    }

    #[test]
    fn test_from_bytes_u32_big_zero() {
        let test_data = [0x00, 0x00, 0x00, 0x00];
        assert_eq!(u32::from_bytes(&test_data, ELFEndianness::Big), 0);
    }

    #[test]
    fn test_from_bytes_u32_little_single_byte() {
        let test_data = [
            ([0x01, 0x00, 0x00, 0x00], 0x00000001),
            ([0x10, 0x00, 0x00, 0x00], 0x00000010),
            ([0xFF, 0x00, 0x00, 0x00], 0x000000FF),
        ];
        for (data, expected) in test_data.iter() {
            assert_eq!(u32::from_bytes(data, ELFEndianness::Little), *expected);
        }
    }

    #[test]
    fn test_from_bytes_u32_big_single_byte() {
        let test_data = [
            ([0x00, 0x00, 0x00, 0x01], 0x00000001),
            ([0x00, 0x00, 0x00, 0x10], 0x00000010),
            ([0x00, 0x00, 0x00, 0xFF], 0x000000FF)
        ];
        for (data, expected) in test_data.iter() {
            assert_eq!(u32::from_bytes(data, ELFEndianness::Big), *expected);
        }
    }

    #[test]
    fn test_from_bytes_u32_little_multi_byte() {
        let test_data = [
            ([0x00, 0x01, 0x00, 0x00], 0x00000100),
            ([0x10, 0x01, 0x00, 0x00], 0x00000110),
            ([0xFF, 0xE3, 0x00, 0x00], 0x0000E3FF),
            ([0xE3, 0xD4, 0x72, 0x1F], 0x1F72D4E3)
        ];
        for (data, expected) in test_data.iter() {
            assert_eq!(u32::from_bytes(data, ELFEndianness::Little), *expected);
        }
    }

    #[test]
    fn test_from_bytes_u32_big_multi_byte() {
        let test_data = [
            ([0x00, 0x00, 0x01, 0x00], 0x00000100),
            ([0x00, 0x00, 0x01, 0x10], 0x00000110),
            ([0x00, 0x00, 0xE3, 0xFF], 0x0000E3FF),
            ([0x1F, 0x72, 0xD4, 0xE3], 0x1F72D4E3)
        ];
        for (data, expected) in test_data.iter() {
            assert_eq!(u32::from_bytes(data, ELFEndianness::Big), *expected);
        }
    }
}