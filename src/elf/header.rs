use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Header {
    word_width: WordWidth,
    endianness: Endianness,
    header_version: u8,
    os_abi: Abi,
    abi_version: u8,
    file_type: FileType,
    arch: Arch,
    version: u32,
    entry_point: Word,
    program_header_start: Word,
    section_header_start: Word,
    flags: u32,
    pheader_entry_size: u16,
    pheader_entries: u16,
    sheader_entry_size: u16,
    sheader_entries: u16,
    section_names_index: u16,
}

#[cfg(test)]
impl Header {
    pub(crate) const fn minimal(word_width: WordWidth, endianness: Endianness) -> Self {
        let word = match word_width {
            WordWidth::Width32 => Word::Word32(0),
            _ => Word::Word64(0),
        };
        Header {
            word_width,
            endianness,
            header_version: 0,
            os_abi: Abi::Unknown,
            abi_version: 0,
            file_type: FileType::None,
            arch: Arch::Unspecified,
            version: 0,
            entry_point: word,
            program_header_start: word,
            section_header_start: word,
            flags: 0,
            pheader_entry_size: 0,
            pheader_entries: 0,
            sheader_entry_size: 0,
            sheader_entries: 0,
            section_names_index: 0,
        }
    }

    pub const fn with_header_version(mut self, header_version: u8) -> Self {
        self.header_version = header_version;
        self
    }

    pub const fn with_abi(mut self, os_abi: Abi) -> Self {
        self.os_abi = os_abi;
        self
    }

    pub const fn with_abi_version(mut self, abi_version: u8) -> Self {
        self.abi_version = abi_version;
        self
    }

    pub const fn with_file_type(mut self, file_type: FileType) -> Self {
        self.file_type = file_type;
        self
    }

    pub const fn with_arch(mut self, arch: Arch) -> Self {
        self.arch = arch;
        self
    }

    pub const fn with_version(mut self, version: u32) -> Self {
        self.version = version;
        self
    }

    pub const fn with_entry_point(mut self, entry_point: Word) -> Self {
        self.entry_point = entry_point;
        self
    }

    pub const fn with_program_header_start(mut self, word: Word) -> Self {
        self.program_header_start = word;
        self
    }

    pub const fn with_section_header_start(mut self, word: Word) -> Self {
        self.section_header_start = word;
        self
    }

    pub const fn with_flags(mut self, flags: u32) -> Self {
        self.flags = flags;
        self
    }

    pub const fn with_program_header_entry_size(mut self, size: u16) -> Self {
        self.pheader_entry_size = size;
        self
    }

    pub const fn with_program_header_entry_count(mut self, count: u16) -> Self {
        self.pheader_entries = count;
        self
    }

    pub const fn with_section_header_entry_size(mut self, size: u16) -> Self {
        self.sheader_entry_size = size;
        self
    }

    pub const fn with_section_header_entry_count(mut self, count: u16) -> Self {
        self.sheader_entries = count;
        self
    }

    pub const fn with_section_names_index(mut self, index: u16) -> Self {
        self.section_names_index = index;
        self
    }
}

impl Header {
    pub fn parse_bytes(bytes: &[u8]) -> Result<Header> {
        // we need at least 52 bytes to parse an ELF header. This is the case for 32-bit ELF files
        Header::check_length(52, bytes.len())?;

        Header::check_magic(bytes)?;

        let word_width = WordWidth::parse_byte(bytes[4])?;

        // now we can check whether we have enough bytes to parse the header
        let required_bytes = match word_width {
            WordWidth::Width32 => 52,
            WordWidth::Width64 => 64,
        };
        Header::check_length(required_bytes, bytes.len())?;

        let endianness = Endianness::parse_byte(bytes[5])?;
        let header_version = bytes[6];
        let os_abi = Abi::from_byte(bytes[7]);
        let abi_version = bytes[8];

        let file_type = FileType::parse_bytes(&bytes[16..18], endianness)?;
        let arch = Arch::parse_bytes(&bytes[18..20], endianness)?;
        let version = u32::from_bytes(&bytes[20..24], endianness);

        // these are the word width dependent offsets of the fields:
        // [entry_point, pheader_start, sheader_start, flags, header_size, pheader_entry_size, pheader_entries, sheader_entry_size, sheader_entries, section_names_index]
        let offsets = match word_width {
            WordWidth::Width32 => [24, 28, 32, 36, 40, 42, 44, 46, 48, 50],
            WordWidth::Width64 => [24, 32, 40, 48, 52, 54, 56, 58, 60, 62],
        };

        let entry_point = Word::parse_bytes(&bytes[offsets[0]..], word_width, endianness)?;
        let program_header_start = Word::parse_bytes(&bytes[offsets[1]..], word_width, endianness)?;
        let section_header_start = Word::parse_bytes(&bytes[offsets[2]..], word_width, endianness)?;

        let flags = u32::from_bytes(&bytes[offsets[3]..], endianness);
        let header_size = u16::from_bytes(&bytes[offsets[4]..], endianness);

        if required_bytes != header_size as usize {
            return Err(ParseError::InsuffcientHeaderLength(header_size as usize));
        }

        let pheader_entry_size = u16::from_bytes(&bytes[offsets[5]..], endianness);
        let pheader_entries = u16::from_bytes(&bytes[offsets[6]..], endianness);
        let sheader_entry_size = u16::from_bytes(&bytes[offsets[7]..], endianness);
        let sheader_entries = u16::from_bytes(&bytes[offsets[8]..], endianness);
        let section_names_index = u16::from_bytes(&bytes[offsets[9]..], endianness);

        Ok(Header {
            word_width,
            endianness,
            header_version,
            os_abi,
            abi_version,
            file_type,
            arch,
            version,
            entry_point,
            program_header_start,
            section_header_start,
            flags,
            pheader_entry_size,
            pheader_entries,
            sheader_entry_size,
            sheader_entries,
            section_names_index,
        })
    }

    fn check_length(minimum: usize, actual: usize) -> Result<()> {
        if actual < minimum {
            Err(ParseError::InsuffcientHeaderLength(actual))
        } else {
            Ok(())
        }
    }

    fn check_magic(bytes: &[u8]) -> Result<()> {
        static MAGIC: u32 = u32::from_le_bytes([0x7F, 0x45, 0x4C, 0x46]);
        let value = u32::from_bytes(bytes, Endianness::Little);
        if value != MAGIC {
            Err(ParseError::NoELF(value))
        } else {
            Ok(())
        }
    }

    pub const fn word_width(&self) -> WordWidth {
        self.word_width
    }

    pub const fn endianness(&self) -> Endianness {
        self.endianness
    }

    pub const fn header_version(&self) -> u8 {
        self.header_version
    }

    pub const fn os_abi(&self) -> Abi {
        self.os_abi
    }

    pub const fn abi_version(&self) -> u8 {
        self.abi_version
    }

    pub const fn file_type(&self) -> FileType {
        self.file_type
    }

    pub const fn arch(&self) -> Arch {
        self.arch
    }

    pub const fn version(&self) -> u32 {
        self.version
    }

    pub const fn entry_point(&self) -> Word {
        self.entry_point
    }

    pub const fn program_header_start(&self) -> Word {
        self.program_header_start
    }

    pub const fn section_header_start(&self) -> Word {
        self.section_header_start
    }

    pub const fn flags(&self) -> u32 {
        self.flags
    }

    pub const fn program_header_entry_size(&self) -> u16 {
        self.pheader_entry_size
    }

    pub const fn program_header_entry_count(&self) -> u16 {
        self.pheader_entries
    }

    pub const fn section_header_entry_size(&self) -> u16 {
        self.sheader_entry_size
    }

    pub const fn section_header_entry_count(&self) -> u16 {
        self.sheader_entries
    }

    pub const fn section_names_index(&self) -> u16 {
        self.section_names_index
    }

    pub const fn size(&self) -> u64 {
        match self.word_width {
            WordWidth::Width32 => 52,
            WordWidth::Width64 => 64,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_valid_magic() {
        let test_data = [
            ([0x7F, 0x45, 0x4C, 0x46], true),
            ([0x7E, 0x45, 0x4C, 0x46], false),
            ([0x7F, 0x46, 0x4C, 0x46], false),
        ];
        for (bytes, expected) in test_data.iter() {
            assert_eq!(Header::check_magic(bytes).is_ok(), *expected);
        }
    }

    static VALID_HEADER_DATA_32: [u8; 52] = [
        // magic
        0x7F, 0x45, 0x4C, 0x46, // word width
        0x01, // 32-bit
        // endianness
        0x01, // little
        // header version
        0x01, // OS ABI
        0x03, // Linux
        // ABI version
        0x01, // padding
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // file type
        0x02, 0x00, // executable
        // arch
        0x3E, 0x00, // x86_64
        // version
        0x01, 0xFF, 0x00, 0x00, // entry point
        0x00, 0x00, 0x00, 0xF0, // program header start
        0x34, 0x00, 0x00, 0x00, // section header start
        0x00, 0x00, 0x30, 0x00, // flags
        0x4F, 0xF1, 0x97, 0xC4, // header size
        0x34, 0x00, // program header entry size
        0x20, 0x00, // program header entry count
        0x01, 0x00, // section header entry size
        0x10, 0x00, // section header entry count
        0x00, 0x01, // section names index
        0x30, 0x00,
    ];

    static VALID_HEADER_DATA_64: [u8; 64] = [
        // magic
        0x7F, 0x45, 0x4C, 0x46, // word width
        0x02, // 64-bit
        // endianness
        0x01, // little
        // header version
        0x01, // OS ABI
        0x03, // Linux
        // ABI version
        0x01, // padding
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // file type
        0x02, 0x00, // executable
        // arch
        0x3E, 0x00, // x86_64
        // version
        0x01, 0xFF, 0x00, 0x00, // entry point
        0x00, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, // program header start
        0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // section header start
        0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, // flags
        0xFF, 0x3A, 0x10, 0x57, // header size
        0x40, 0x00, // program header entry size
        0x38, 0x00, // program header entry count
        0x01, 0x00, // section header entry size
        0x00, 0x80, // section header entry count
        0x40, 0x00, // section names index
        0x2F, 0x00,
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
        assert_eq!(result, Err(ParseError::InsuffcientHeaderLength(0)));
    }

    #[test]
    fn test_header_err_word32_len() {
        let test_data = &VALID_HEADER_DATA_32[..50];
        let result = Header::parse_bytes(test_data);
        assert_eq!(result, Err(ParseError::InsuffcientHeaderLength(50)));
    }

    #[test]
    fn test_header_err_word64_len() {
        let test_data = &VALID_HEADER_DATA_64[..55];
        let result = Header::parse_bytes(test_data);
        assert_eq!(result, Err(ParseError::InsuffcientHeaderLength(55)));
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
}
