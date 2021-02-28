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
    Word32,
    Word64
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
            0x01 => Ok(Word32),
            0x02 => Ok(Word64),
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
        //TODO maybe don't allow too large slices as well ?
        if bytes.len() < 2 {
            Err(ELFParseError::InsufficientPartLength(bytes.len()))
        } else {
            Ok(ELFArch::from_u16(u16::from_bytes(bytes, endianness)))
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
    version: u32
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

    pub(crate) const fn new(word_width: ELFWordWidth, endianness: ELFEndianness, header_version: u8,
    os_abi: ElfAbi, abi_version: u8, file_type: ELFFileType, arch: ELFArch, version: u32) -> ELFHeader {
        ELFHeader {
            word_width,
            endianness,
            header_version,
            os_abi,
            abi_version,
            file_type,
            arch,
            version
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<ELFHeader, ELFParseError> {
        if bytes.len() < 24 {
            return Err(ELFParseError::InvalidHeaderLength(bytes.len()));
        }

        let magic_bytes = get_u32_bytes(bytes);
        let magic = u32::from_le_bytes(magic_bytes);
        if !is_valid_magic(magic_bytes) {
            return Err(ELFParseError::NoELF(magic));
        }

        let word_width = ELFWordWidth::from_byte(bytes[4])?;
        let endianness = ELFEndianness::from_byte(bytes[5])?;
        let header_version = bytes[6];
        let os_abi = ElfAbi::from_byte(bytes[7]);
        let abi_version = bytes[8];

        // the offset given by the padding bytes which are the bytes 9-15
        const OFFSET: usize = 16;
        let file_type = ELFFileType::from_bytes(&bytes[OFFSET..OFFSET+2], endianness)?;
        let arch = ELFArch::from_bytes(&bytes[OFFSET+2..OFFSET+4], endianness)?;
        let version = u32::from_bytes(&bytes[OFFSET+4..OFFSET+8], endianness);

        Ok(ELFHeader::new(word_width, endianness, header_version, os_abi, abi_version, file_type, arch, version))
    }
}

static ELF_ASCII: [u8;3] = [0x45, 0x4C, 0x46];

pub(crate) fn is_valid_magic(magic: [u8; 4]) -> bool {
    let magic_bytes = &magic[1..];
    (magic[0] & 0xFF == 0x7F) && magic_bytes.eq(&ELF_ASCII)
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
        assert!(bytes.len() >= 2);
        let bytes = get_u32_bytes(bytes);
        match endianness{
            ELFEndianness::Little => u32::from_le_bytes(bytes),
            ELFEndianness::Big => u32::from_be_bytes(bytes)
        }
    }
}

fn get_u16_bytes(bytes: &[u8]) -> [u8;2] {
    [bytes[0], bytes[1]]
}

fn get_u32_bytes(bytes: &[u8]) -> [u8;4] {
    [bytes[0], bytes[1], bytes[2], bytes[3]]
}