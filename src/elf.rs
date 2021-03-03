#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ELFFileType {
    None,
    Relocatable,
    Executable,
    Shared,
    Core,
    Specific(u16)
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ELFWordWidth {
    Width32,
    Width64
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ELFEndianness {
    Little,
    Big
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ElfAbi {
    SysV,
    HpUx,
    NetBSD,
    Linux,
    GnuHurd,
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
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ELFArch {
    Unspecified,
    WE32100,
    Sparc,
    X86,
    M68k,
    M88k,
    IntelMCU,
    Intel80860,
    MIPS,
    System370,
    RS3000,
    PARISC,
    Intel80960,
    PowerPC,
    PowerPC64,
    S390,
    ARM,
    SuperH,
    IA64,
    X86_64,
    TMS320C6000,
    AArch64,
    RISCV,
    BPF,
    WDC65C816,
    Unknown
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
pub enum ELFWord {
    Word32(u32),
    Word64(u64)
}

impl ELFFileType {

    fn from_u16(i: u16) -> Result<ELFFileType, ELFParseError> {
        use ELFFileType::*;
        match i {
            0x0000 => Ok(None),
            0x0001 => Ok(Relocatable),
            0x0002 => Ok(Executable),
            0x0003 => Ok(Shared),
            0x0004 => Ok(Core),
            _ if i >= 0xff00 => Ok(Specific(i)),
            _ => Err(ELFParseError::InvalidFileType(i))
        }
    }

    pub(crate) fn from_bytes(bytes: &[u8], endianness: ELFEndianness) -> Result<ELFFileType, ELFParseError> {
        if bytes.len() < 2 {
            Err(ELFParseError::InsufficientPartLength(bytes.len()))
        } else {
            ELFFileType::from_u16(u16::from_bytes(bytes, endianness))
        }
    }
}

impl ELFWordWidth {

    pub(crate) fn from_byte(b: u8) -> Result<ELFWordWidth, ELFParseError> {
        use ELFWordWidth::*;
        match b {
            0x01 => Ok(Width32),
            0x02 => Ok(Width64),
            _ => Err(ELFParseError::InvalidWordWidth(b))
        }
    }
}

impl ELFEndianness {

    pub(crate) fn from_byte(b: u8) -> Result<ELFEndianness, ELFParseError> {
        use ELFEndianness::*;
        match b {
            0x01 => Ok(Little),
            0x02 => Ok(Big),
            _ => Err(ELFParseError::InvalidEndianness(b))
        }
    }
}

impl ElfAbi {

    pub(crate) fn from_byte(b: u8) -> ElfAbi {
        use ElfAbi::*;
        match b {
            0x00 => SysV,
            0x01 => HpUx,
            0x02 => NetBSD,
            0x03 => Linux,
            0x04 => GnuHurd,
            0x06 => Solaris,
            0x07 => Aix,
            0x08 => Irix,
            0x09 => FreeBSD,
            0x0A => Tru64,
            0x0B => NovellModesto,
            0x0C => OpenBSD,
            0x0D => OpenVMS,
            0x0E => NonStopKernel,
            0x0F => Aros,
            0x10 => FenixOS,
            0x11 => CloudABI,
            0x12 => OpenVOS,
            _ => Unknown
        }
    }
}

impl ELFArch {

    fn from_u16(i: u16) -> ELFArch {
        use ELFArch::*;
        match i {
            0x0000 => Unspecified,
            0x0001 => WE32100,
            0x0002 => Sparc,
            0x0003 => X86,
            0x0004 => M68k,
            0x0005 => M88k,
            0x0006 => IntelMCU,
            0x0007 => Intel80860,
            0x0008 => MIPS,
            0x0009 => System370,
            0x000A => RS3000,
            0x000E => PARISC,
            0x0013 => Intel80960,
            0x0014 => PowerPC,
            0x0015 => PowerPC64,
            0x0016 => S390,
            0x0028 => ARM,
            0x002A => SuperH,
            0x0032 => IA64,
            0x003E => X86_64,
            0x008C => TMS320C6000,
            0x00B7 => AArch64,
            0x00F3 => RISCV,
            0x00F7 => BPF,
            0x0101 => WDC65C816,
            _ => Unknown
        }
    }

    pub(crate) fn from_bytes(bytes: &[u8], endianness: ELFEndianness) -> Result<ELFArch, ELFParseError> {
        // allow larger slices as well. The number of read bytes is known statically
        if bytes.len() < 2 {
            Err(ELFParseError::InsufficientPartLength(bytes.len()))
        } else {
            Ok(ELFArch::from_u16(u16::from_bytes(bytes, endianness)))
        }
    }
}

impl ELFWord {

    pub(crate) fn from_bytes(bytes: &[u8], word_width: ELFWordWidth, endianness: ELFEndianness) -> Result<(ELFWord, usize), ELFParseError> {
        match word_width {
            ELFWordWidth::Width32 => {
                if bytes.len() < 4 {
                    Err(ELFParseError::InsufficientPartLength(bytes.len()))
                } else {
                    Ok((ELFWord::Word32(u32::from_bytes(bytes, endianness)), 4))
                }
            },
            ELFWordWidth::Width64 => {
                if bytes.len() < 8 {
                    Err(ELFParseError::InsufficientPartLength(bytes.len()))
                } else {
                    Ok((ELFWord::Word64(u64::from_bytes(bytes, endianness)), 8))
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ELFHeader {
    word_width: ELFWordWidth,
    endianness: ELFEndianness,
    header_version: u8,
    os_abi: ElfAbi,
    abi_version: u8,
    file_type: ELFFileType,
    arch: ELFArch,
    version: u32,
    entry_point: ELFWord,
    program_header_start: ELFWord,
    section_header_start: ELFWord,
    flags: u32,
    pheader_entry_size: u16,
    pheader_entries: u16,
    sheader_entry_size: u16,
    sheader_entries: u16,
    section_names_index: u16
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ELFParseError {
    InvalidHeaderLength(usize),
    NoELF(u32),
    InvalidWordWidth(u8),
    InvalidEndianness(u8),
    InvalidFileType(u16),
    InsufficientPartLength(usize)
}

impl ELFHeader {

    pub(crate) const fn minimal(word_width: ELFWordWidth, endianness: ELFEndianness) -> Self {
        let word = match word_width {
            ELFWordWidth::Width32 => ELFWord::Word32(0),
            _ => ELFWord::Word64(0)
        };
        ELFHeader {
            word_width,
            endianness,
            header_version: 0,
            os_abi: ElfAbi::Unknown,
            abi_version: 0,
            file_type: ELFFileType::None,
            arch: ELFArch::Unspecified,
            version: 0,
            entry_point: word,
            program_header_start: word,
            section_header_start: word,
            flags: 0,
            pheader_entry_size: 0,
            pheader_entries: 0,
            sheader_entry_size: 0,
            sheader_entries: 0,
            section_names_index: 0
        }
    }

    pub(crate) const fn header_version(mut self, header_version: u8) -> Self {
        self.header_version = header_version;
        self
    }

    pub(crate) const fn abi(mut self, os_abi: ElfAbi) -> Self {
        self.os_abi = os_abi;
        self
    }

    pub(crate) const fn abi_version(mut self, abi_version: u8) -> Self {
        self.abi_version = abi_version;
        self
    }

    pub(crate) const fn file_type(mut self, file_type: ELFFileType) -> Self {
        self.file_type = file_type;
        self
    }

    pub(crate) const fn arch(mut self, arch: ELFArch) -> Self {
        self.arch = arch;
        self
    }

    pub(crate) const fn version(mut self, version: u32) -> Self {
        self.version = version;
        self
    }

    pub(crate) const fn entry_point(mut self, entry_point: ELFWord) -> Self {
        self.entry_point = entry_point;
        self
    }

    pub(crate) const fn program_header_start(mut self, word: ELFWord) -> Self {
        self.program_header_start = word;
        self
    }

    pub(crate) const fn section_header_start(mut self, word: ELFWord) -> Self {
        self.section_header_start = word;
        self
    }

    pub(crate) const fn flags(mut self, flags: u32) -> Self {
        self.flags = flags;
        self
    }

    pub(crate) const fn program_header_entry_size(mut self, size: u16) -> Self {
        self.pheader_entry_size = size;
        self
    }

    pub(crate) const fn program_header_entry_count(mut self, count: u16) -> Self {
        self.pheader_entries = count;
        self
    }

    pub(crate) const fn section_header_entry_size(mut self, size: u16) -> Self {
        self.sheader_entry_size = size;
        self
    }

    pub(crate) const fn section_header_entry_count(mut self, count: u16) -> Self {
        self.sheader_entries = count;
        self
    }

    pub(crate) const fn section_names_index(mut self, index: u16) -> Self {
        self.section_names_index = index;
        self
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<ELFHeader, ELFParseError> {
        // we need at least 52 bytes to parse an ELF header. This is the case for 32-bit ELF files
        ELFHeader::check_length(52, bytes.len())?;

        ELFHeader::parse_magic(bytes)?;

        let word_width = ELFWordWidth::from_byte(bytes[4])?;

        // now we can check whether we have enough bytes to parse the header
        let required_bytes = match word_width {
            ELFWordWidth::Width32 => 52,
            ELFWordWidth::Width64 => 64,
        };
        ELFHeader::check_length(required_bytes, bytes.len())?;

        let endianness = ELFEndianness::from_byte(bytes[5])?;
        let header_version = bytes[6];
        let os_abi = ElfAbi::from_byte(bytes[7]);
        let abi_version = bytes[8];

        // the offset given by the padding bytes which are the bytes 9-15
        let offset = 16;
        let file_type = ELFFileType::from_bytes(&bytes[offset..offset +2], endianness)?;
        let arch = ELFArch::from_bytes(&bytes[offset +2..offset +4], endianness)?;
        let version = u32::from_bytes(&bytes[offset +4..offset +8], endianness);

        // now we have fields of varying size so we use a mutable offset to track the current position
        // this way we keep the slicing short and simple
        let mut byte_offset = offset + 8;

        let (entry_point, read) = ELFWord::from_bytes(&bytes[byte_offset..], word_width, endianness)?;
        byte_offset += read;

        let (program_header_start, read) = ELFWord::from_bytes(&bytes[byte_offset..], word_width, endianness)?;
        byte_offset += read;

        let (section_header_start, read) = ELFWord::from_bytes(&bytes[byte_offset..], word_width, endianness)?;
        byte_offset += read;

        let offset = byte_offset;
        let flags = u32::from_bytes(&bytes[offset..offset+4], endianness);
        let header_size = u16::from_bytes(&bytes[offset+4..offset+6], endianness);

        if required_bytes != header_size as usize {
            return Err(ELFParseError::InvalidHeaderLength(header_size as usize));
        }

        let pheader_entry_size = u16::from_bytes(&bytes[offset+6..offset+8], endianness);
        let pheader_entries = u16::from_bytes(&bytes[offset+8..offset+10], endianness);
        let sheader_entry_size = u16::from_bytes(&bytes[offset+10..offset+12], endianness);
        let sheader_entries = u16::from_bytes(&bytes[offset+12..offset+14], endianness);
        let section_names_index = u16::from_bytes(&bytes[offset+14..offset+16], endianness);

        Ok(
            ELFHeader {
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
                section_names_index
            }
        )
    }

    fn check_length(minimum: usize, actual: usize) -> Result<(), ELFParseError> {
        if actual < minimum {
            Err(ELFParseError::InvalidHeaderLength(actual))
        } else {
            Ok(())
        }
    }

    fn parse_magic(bytes: &[u8]) -> Result<(), ELFParseError> {
        let magic_bytes = get_u32_bytes(bytes);
        let magic = u32::from_le_bytes(magic_bytes);
        if !is_valid_magic(magic_bytes) || bytes[0] != 0x7F {
            Err(ELFParseError::NoELF(magic))
        } else {
            Ok(())
        }
    }

}


// ASCII for "ELF"
static ELF_ASCII: [u8;3] = [0x45, 0x4C, 0x46];

pub(crate) fn is_valid_magic(magic: [u8; 4]) -> bool {
    let magic_bytes = &magic[1..];
    (magic[0] == 0x7F) && magic_bytes.eq(&ELF_ASCII)
}


//TODO maybe reimplement this facility with macros to ensure the correct number of bytes at compile time
pub(crate) trait FromBytesEndianned {
    fn from_bytes(bytes: &[u8], endianness: ELFEndianness) -> Self;
}

impl FromBytesEndianned for u16 {
    fn from_bytes(bytes: &[u8], endianness: ELFEndianness) -> Self {
        assert!(bytes.len() >= 2);
        let bytes = get_u16_bytes(bytes);
        match endianness{
            ELFEndianness::Little => u16::from_le_bytes(bytes),
            ELFEndianness::Big => u16::from_be_bytes(bytes)
        }
    }
}

impl FromBytesEndianned for u32 {
    fn from_bytes(bytes: &[u8], endianness: ELFEndianness) -> Self {
        assert!(bytes.len() >= 4);
        let bytes = get_u32_bytes(bytes);
        match endianness{
            ELFEndianness::Little => u32::from_le_bytes(bytes),
            ELFEndianness::Big => u32::from_be_bytes(bytes)
        }
    }
}

impl FromBytesEndianned for u64 {
    fn from_bytes(bytes: &[u8], endianness: ELFEndianness) -> Self {
        assert!(bytes.len() >= 8);
        let bytes = get_u64_bytes(bytes);
        match endianness {
            ELFEndianness::Little => u64::from_le_bytes(bytes),
            ELFEndianness::Big => u64::from_be_bytes(bytes)
        }
    }
}

fn get_u16_bytes(bytes: &[u8]) -> [u8;2] {
    [bytes[0], bytes[1]]
}

fn get_u32_bytes(bytes: &[u8]) -> [u8;4] {
    [bytes[0], bytes[1], bytes[2], bytes[3]]
}

fn get_u64_bytes(bytes: &[u8]) -> [u8;8] {
    [bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]]
}