#[derive(Debug)]
pub enum ELFFileType {
    None,
    Relocatable,
    Executable,
    Shared,
    Core,
    Specific(u16)
}

#[derive(Debug)]
pub enum ELFWordWidth {
    Word32,
    Word64
}

#[derive(Debug)]
pub enum ELFEndianness {
    Little,
    Big
}

#[derive(Debug)]
pub enum ELFArch {
    Unspecified,
    Sparc,
    X86,
    MIPS,
    PowerPC,
    ARM,
    SuperH,
    IA64,
    X86_64,
    AArch64
}

impl ELFFileType {

    pub fn from_u16(i: u16) -> Result<ELFFileType, ELFParseError> {
        match i {
            0 => Ok(ELFFileType::None),
            1 => Ok(ELFFileType::Relocatable),
            2 => Ok(ELFFileType::Executable),
            3 => Ok(ELFFileType::Shared),
            4 => Ok(ELFFileType::Core),
            _ if i >= 0xff00 => Ok(ELFFileType::Specific(i)),
            _ => Err(ELFParseError::InvalidFileType(i))
        }
    }
}

impl ELFWordWidth {

    pub fn from_byte(b: u8) -> Result<ELFWordWidth, ELFParseError> {
        match b {
            1 => Ok(ELFWordWidth::Word32),
            2 => Ok(ELFWordWidth::Word64),
            _ => Err(ELFParseError::InvalidWordWidth(b))
        }
    }
}

impl ELFEndianness {

    pub fn from_byte(b: u8) -> Result<ELFEndianness, ELFParseError> {
        match b {
            1 => Ok(ELFEndianness::Little),
            2 => Ok(ELFEndianness::Big),
            _ => Err(ELFParseError::InvalidEndianness(b))
        }
    }
}

impl ELFArch {

    pub fn from_u16(i: u16) -> Result<ELFArch, ELFParseError> {
        match i {
            0 => Ok(ELFArch::Unspecified),
            2 => Ok(ELFArch::Sparc),
            3 => Ok(ELFArch::X86),
            8 => Ok(ELFArch::MIPS),
            0x14 => Ok(ELFArch::PowerPC),
            0x28 => Ok(ELFArch::ARM),
            0x2A => Ok(ELFArch::SuperH),
            0x32 => Ok(ELFArch::IA64),
            0x3E => Ok(ELFArch::X86_64),
            0xB7 => Ok(ELFArch::AArch64),
            _ => Err(ELFParseError::InvalidArch(i))
        }
    }

    pub fn to_u16(&self) -> u16 {
        match *self {
            ELFArch::Unspecified => 0,
            ELFArch::Sparc => 2,
            ELFArch::X86 => 3,
            ELFArch::MIPS => 8,
            ELFArch::PowerPC => 0x14,
            ELFArch::ARM => 0x28,
            ELFArch::SuperH => 0x2A,
            ELFArch::IA64 => 0x32,
            ELFArch::X86_64 => 0x3E,
            ELFArch::AArch64 => 0xB7
        }
    }
}

#[derive(Debug)]
pub struct ELFHeader {
    word_width: ELFWordWidth,
    endianness: ELFEndianness,
    header_version: u8,
    os_abi: u8,
    file_type: ELFFileType,
    arch: ELFArch
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum ELFParseError {
    InvalidHeaderLength(usize),
    NoELF(u32),
    InvalidWordWidth(u8),
    InvalidEndianness(u8),
    InvalidFileType(u16),
    InvalidArch(u16)
}

impl ELFHeader {

    pub fn from_bytes(bytes: &[u8]) -> Result<ELFHeader, ELFParseError> {
        if bytes.len() < 12 {
            Err(ELFParseError::InvalidHeaderLength(bytes.len()))
        } else {
            let magic_bytes = [bytes[0], bytes[1], bytes[2], bytes[3]];
            let magic = u32::from_le_bytes(magic_bytes);
            if !is_valid_maigic(magic) {
                return Err(ELFParseError::NoELF(magic));
            }
            let word_width = ELFWordWidth::from_byte(bytes[4])?;
            let endianness = ELFEndianness::from_byte(bytes[5])?;
            let header_version = bytes[6];
            let os_abi = bytes[7];
            const OFFSET: usize = 15;
            let file_type_bytes = [bytes[OFFSET], bytes[OFFSET + 1]];
            let file_type = ELFFileType::from_u16(u16::from_be_bytes(file_type_bytes))?;
            let arch_bytes = [bytes[OFFSET + 2], bytes[OFFSET + 3]];
            let arch = ELFArch::from_u16(u16::from_be_bytes(arch_bytes))?;
            Ok(ELFHeader {
                word_width,
                endianness,
                header_version,
                os_abi,
                file_type,
                arch
            })
        }
    }
}

static ELF_ASCII: [u8;3] = [0x45, 0x4c, 0x46];

fn is_valid_magic(magic: u32) -> bool {
    let bytes = magic.to_le_bytes();
    let magic_bytes = &bytes[1..];
    (magic & 0xff == 0x7f) && magic_bytes.eq(&ELF_ASCII)
}