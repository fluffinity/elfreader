use std::fmt::{LowerHex, Formatter, UpperHex, Binary, Debug};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum FileType {
    None,
    Relocatable,
    Executable,
    Shared,
    Core,
    Specific(u16)
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum WordWidth {
    Width32,
    Width64
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Endianness {
    Little,
    Big
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Abi {
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
pub enum Arch {
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

#[derive(Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
pub enum Word {
    Word32(u32),
    Word64(u64)
}

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
    section_names_index: u16
}

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
    ProcessorSpecific(u32)
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
    alignment: Word
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ParseError {
    InvalidHeaderLength(usize),
    NoELF(u32),
    InvalidWordWidth(u8),
    InvalidEndianness(u8),
    InvalidFileType(u16),
    InvalidProgHeaderLength(usize),
    InvalidProgHeaderType(u32),
    InvalidAlignment(u64),
    InvalidVirtualAddress(Word),
    InsufficientPartLength(usize)
}

pub type ELFResult<T> = Result<T, ParseError>;

impl FileType {

    fn parse_u16(i: u16) -> ELFResult<FileType> {
        use FileType::*;
        match i {
            0x0000 => Ok(None),
            0x0001 => Ok(Relocatable),
            0x0002 => Ok(Executable),
            0x0003 => Ok(Shared),
            0x0004 => Ok(Core),
            _ if i >= 0xff00 => Ok(Specific(i)),
            _ => Err(ParseError::InvalidFileType(i))
        }
    }

    pub(crate) fn parse_bytes(bytes: &[u8], endianness: Endianness) -> ELFResult<FileType> {
        if bytes.len() < 2 {
            Err(ParseError::InsufficientPartLength(bytes.len()))
        } else {
            FileType::parse_u16(u16::from_bytes(bytes, endianness))
        }
    }
}

impl WordWidth {

    pub(crate) fn parse_byte(b: u8) -> ELFResult<WordWidth> {
        use WordWidth::*;
        match b {
            0x01 => Ok(Width32),
            0x02 => Ok(Width64),
            _ => Err(ParseError::InvalidWordWidth(b))
        }
    }
}

impl Endianness {

    pub(crate) fn parse_byte(b: u8) -> ELFResult<Endianness> {
        use Endianness::*;
        match b {
            0x01 => Ok(Little),
            0x02 => Ok(Big),
            _ => Err(ParseError::InvalidEndianness(b))
        }
    }
}

impl Abi {

    pub(crate) fn from_byte(b: u8) -> Abi {
        use Abi::*;
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

impl Arch {

    fn from_u16(i: u16) -> Arch {
        use Arch::*;
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

    pub(crate) fn parse_bytes(bytes: &[u8], endianness: Endianness) -> ELFResult<Arch> {
        // allow larger slices as well. The number of read bytes is known statically
        if bytes.len() < 2 {
            Err(ParseError::InsufficientPartLength(bytes.len()))
        } else {
            Ok(Arch::from_u16(u16::from_bytes(bytes, endianness)))
        }
    }
}

impl ProgramHeaderSegmentType {

    fn parse_u32(u: u32) -> ELFResult<ProgramHeaderSegmentType> {
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
            _ => Err(ParseError::InvalidProgHeaderType(u))
        }
    }

    pub(crate) fn parse_bytes(bytes: &[u8], endianness: Endianness) -> ELFResult<ProgramHeaderSegmentType> {
        if bytes.len() < 4 {
            Err(ParseError::InsufficientPartLength(bytes.len()))
        } else{
            ProgramHeaderSegmentType::parse_u32(u32::from_bytes(bytes, endianness))
        }
    }
}

impl Word {

    pub(crate) fn parse_bytes(bytes: &[u8], word_width: WordWidth, endianness: Endianness) -> ELFResult<Word> {
        match word_width {
            WordWidth::Width32 => {
                if bytes.len() < 4 {
                    Err(ParseError::InsufficientPartLength(bytes.len()))
                } else {
                    Ok(Word::Word32(u32::from_bytes(bytes, endianness)))
                }
            },
            WordWidth::Width64 => {
                if bytes.len() < 8 {
                    Err(ParseError::InsufficientPartLength(bytes.len()))
                } else {
                    Ok(Word::Word64(u64::from_bytes(bytes, endianness)))
                }
            }
        }
    }
}

impl Debug for Word {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Word::Word32(u) => write!(f, "Word32({:#x})", u),
            Word::Word64(u) => write!(f, "Word64({:#x})", u)
        }
    }
}

impl LowerHex for Word {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Word::Word32(u) => LowerHex::fmt(&u, f),
            Word::Word64(u) => LowerHex::fmt(&u, f)
        }
    }
}

impl UpperHex for Word {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Word::Word32(u) => UpperHex::fmt(&u, f),
            Word::Word64(u) => UpperHex::fmt(&u, f)
        }
    }
}

impl Binary for Word {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Word::Word32(u) => Binary::fmt(&u, f),
            Word::Word64(u) => Binary::fmt(&u, f)
        }
    }
}

#[cfg(test)]
impl Header {

    pub(crate) const fn minimal(word_width: WordWidth, endianness: Endianness) -> Self {
        let word = match word_width {
            WordWidth::Width32 => Word::Word32(0),
            _ => Word::Word64(0)
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
            section_names_index: 0
        }
    }

    pub(crate) const fn with_header_version(mut self, header_version: u8) -> Self {
        self.header_version = header_version;
        self
    }

    pub(crate) const fn with_abi(mut self, os_abi: Abi) -> Self {
        self.os_abi = os_abi;
        self
    }

    pub(crate) const fn with_abi_version(mut self, abi_version: u8) -> Self {
        self.abi_version = abi_version;
        self
    }

    pub(crate) const fn with_file_type(mut self, file_type: FileType) -> Self {
        self.file_type = file_type;
        self
    }

    pub(crate) const fn with_arch(mut self, arch: Arch) -> Self {
        self.arch = arch;
        self
    }

    pub(crate) const fn with_version(mut self, version: u32) -> Self {
        self.version = version;
        self
    }

    pub(crate) const fn with_entry_point(mut self, entry_point: Word) -> Self {
        self.entry_point = entry_point;
        self
    }

    pub(crate) const fn with_program_header_start(mut self, word: Word) -> Self {
        self.program_header_start = word;
        self
    }

    pub(crate) const fn with_section_header_start(mut self, word: Word) -> Self {
        self.section_header_start = word;
        self
    }

    pub(crate) const fn with_flags(mut self, flags: u32) -> Self {
        self.flags = flags;
        self
    }

    pub(crate) const fn with_program_header_entry_size(mut self, size: u16) -> Self {
        self.pheader_entry_size = size;
        self
    }

    pub(crate) const fn with_program_header_entry_count(mut self, count: u16) -> Self {
        self.pheader_entries = count;
        self
    }

    pub(crate) const fn with_section_header_entry_size(mut self, size: u16) -> Self {
        self.sheader_entry_size = size;
        self
    }

    pub(crate) const fn with_section_header_entry_count(mut self, count: u16) -> Self {
        self.sheader_entries = count;
        self
    }

    pub(crate) const fn with_section_names_index(mut self, index: u16) -> Self {
        self.section_names_index = index;
        self
    }
}

impl Header {

    pub fn parse_bytes(bytes: &[u8]) -> ELFResult<Header> {
        // we need at least 52 bytes to parse an ELF header. This is the case for 32-bit ELF files
        Header::check_length(52, bytes.len())?;

        Header::parse_magic(bytes)?;

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
            WordWidth::Width32 => [24, 28, 32, 36, 40, 42, 44, 46, 48, 50, 52],
            WordWidth::Width64 => [24, 32, 40, 48, 52, 54, 56, 58, 60, 62, 64]
        } ;

        let entry_point = Word::parse_bytes(&bytes[offsets[0]..], word_width, endianness)?;
        let program_header_start = Word::parse_bytes(&bytes[offsets[1]..], word_width, endianness)?;
        let section_header_start = Word::parse_bytes(&bytes[offsets[2]..], word_width, endianness)?;

        let flags = u32::from_bytes(&bytes[offsets[3]..offsets[4]], endianness);
        let header_size = u16::from_bytes(&bytes[offsets[4]..offsets[5]], endianness);

        if required_bytes != header_size as usize {
            return Err(ParseError::InvalidHeaderLength(header_size as usize));
        }

        let pheader_entry_size = u16::from_bytes(&bytes[offsets[5]..offsets[6]], endianness);
        let pheader_entries = u16::from_bytes(&bytes[offsets[6]..offsets[7]], endianness);
        let sheader_entry_size = u16::from_bytes(&bytes[offsets[7]..offsets[8]], endianness);
        let sheader_entries = u16::from_bytes(&bytes[offsets[8]..offsets[9]], endianness);
        let section_names_index = u16::from_bytes(&bytes[offsets[9]..offsets[10]], endianness);

        Ok(
            Header {
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

    fn check_length(minimum: usize, actual: usize) -> ELFResult<()> {
        if actual < minimum {
            Err(ParseError::InvalidHeaderLength(actual))
        } else {
            Ok(())
        }
    }

    fn parse_magic(bytes: &[u8]) -> ELFResult<()> {
        let magic_bytes = get_u32_bytes(bytes);
        let magic = u32::from_le_bytes(magic_bytes);
        if !Header::is_valid_magic(magic_bytes) || bytes[0] != 0x7F {
            Err(ParseError::NoELF(magic))
        } else {
            Ok(())
        }
    }

    pub(crate) fn is_valid_magic(magic: [u8; 4]) -> bool {
        let magic_bytes = &magic[1..];
        (magic[0] == 0x7F) && magic_bytes.eq(&ELF_ASCII)
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
             WordWidth::Width64 => 64
         }
    }
}

// ASCII for "ELF"
static ELF_ASCII: [u8;3] = [0x45, 0x4C, 0x46];

#[cfg(test)]
impl ProgramHeader {
    
    pub(crate) const fn new(typ: ProgramHeaderSegmentType, offset: Word, vaddress: Word, paddress: Word,
        filesize: Word, memsize: Word, flags: u32, alignment: Word) -> ProgramHeader {
            ProgramHeader {
                typ,
                offset,
                vaddress,
                paddress,
                filesize,
                memsize,
                flags,
                alignment
            }
        }
}

impl ProgramHeader {

    pub(crate) fn parse_bytes(bytes: &[u8], word_width: WordWidth, endianness: Endianness) -> ELFResult<ProgramHeader> {
        ProgramHeader::check_length(32, bytes.len())?;
        let typ = ProgramHeaderSegmentType::parse_bytes(bytes, endianness)?;
        // these are the word width dependent offsets of the fields:
        // [offset, vaddress, paddress, filesize, memsize, flags, alignment]
        let (offsets, size) = match word_width {
            WordWidth::Width32 => ([4, 8, 12, 16, 20, 24, 28], 32),
            WordWidth::Width64 => ([8, 16, 24, 32, 40, 4, 48], 56)
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
            alignment
        })
    }

    fn check_length(expected: usize, actual: usize) -> ELFResult<()> {
        if actual < expected {
            Err(ParseError::InvalidProgHeaderLength(actual))
        } else {
            Ok(())
        }
    }

    fn validate_vaddr(offset: Word, addr: Word, align: Word) -> ELFResult<()> {
        let align = match align {
            Word::Word64(u) => u,
            Word::Word32(u) => u as u64
        };
        if align <= 1 {
            Ok(())
        } else if !align.is_power_of_two() {
            Err(ParseError::InvalidAlignment(align))
        } else {
            let offset = match offset {
                Word::Word64(u) => u,
                Word::Word32(u) => u as u64
            };
            let normalized_addr = match addr {
                Word::Word64(u) => u,
                Word::Word32(u) => u as u64
            };
            if normalized_addr % align == offset % align {
                Ok(())
            } else {
                println!("Welp, RIP");
                println!("normalized_addr: {}, offset: {}, align: {}", normalized_addr, offset, align);
                Err(ParseError::InvalidVirtualAddress(addr))
            }
        }
    }
}
//TODO maybe reimplement this facility with macros to ensure the correct number of bytes at compile time
pub(crate) trait FromBytesEndianned {
    fn from_bytes(bytes: &[u8], endianness: Endianness) -> Self;
}

impl FromBytesEndianned for u16 {
    fn from_bytes(bytes: &[u8], endianness: Endianness) -> Self {
        assert!(bytes.len() >= 2);
        let bytes = get_u16_bytes(bytes);
        match endianness{
            Endianness::Little => u16::from_le_bytes(bytes),
            Endianness::Big => u16::from_be_bytes(bytes)
        }
    }
}

impl FromBytesEndianned for u32 {
    fn from_bytes(bytes: &[u8], endianness: Endianness) -> Self {
        assert!(bytes.len() >= 4);
        let bytes = get_u32_bytes(bytes);
        match endianness{
            Endianness::Little => u32::from_le_bytes(bytes),
            Endianness::Big => u32::from_be_bytes(bytes)
        }
    }
}

impl FromBytesEndianned for u64 {
    fn from_bytes(bytes: &[u8], endianness: Endianness) -> Self {
        assert!(bytes.len() >= 8);
        let bytes = get_u64_bytes(bytes);
        match endianness {
            Endianness::Little => u64::from_le_bytes(bytes),
            Endianness::Big => u64::from_be_bytes(bytes)
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