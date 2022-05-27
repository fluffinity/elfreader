use super::{FromBytesEndianned, SectionHeaderType};

use std::{
    ffi::{FromVecWithNulError, IntoStringError},
    fmt::{Binary, Debug, Formatter, LowerHex, UpperHex},
};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum FileType {
    None,
    Relocatable,
    Executable,
    Shared,
    Core,
    Specific(u16),
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum WordWidth {
    Width32,
    Width64,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Endianness {
    Little,
    Big,
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
    Unknown,
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
    Unknown,
}

#[derive(Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
pub enum Word {
    Word32(u32),
    Word64(u64),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ParseError {
    InsuffcientHeaderLength(usize),
    NoELF(u32),
    InvalidWordWidth(u8),
    InvalidEndianness(u8),
    InvalidFileType(u16),
    InsufficientProgramHeaderLength(usize),
    InvalidProgramHeaderType(u32),
    InvalidAlignment(u64),
    InvalidVirtualAddress(Word),
    InsufficientPartLength(usize),
    InsufficientSectionHeaderLength(usize),
    InvalidSectionHeaderType(u32),
    InvalidSectionHeaderFlags(u64),
    UnterminatedString,
    InvalidSectionName(IntoStringError),
    InvalidSectionNameTableType(SectionHeaderType),
}

pub type Result<T> = std::result::Result<T, ParseError>;

impl FileType {
    fn parse_u16(i: u16) -> Result<FileType> {
        use FileType::*;
        match i {
            0x0000 => Ok(None),
            0x0001 => Ok(Relocatable),
            0x0002 => Ok(Executable),
            0x0003 => Ok(Shared),
            0x0004 => Ok(Core),
            _ if i >= 0xff00 => Ok(Specific(i)),
            _ => Err(ParseError::InvalidFileType(i)),
        }
    }

    pub(crate) fn parse_bytes(bytes: &[u8], endianness: Endianness) -> Result<FileType> {
        if bytes.len() < 2 {
            Err(ParseError::InsufficientPartLength(bytes.len()))
        } else {
            FileType::parse_u16(u16::from_bytes(bytes, endianness))
        }
    }
}

impl WordWidth {
    pub(crate) fn parse_byte(b: u8) -> Result<WordWidth> {
        use WordWidth::*;
        match b {
            0x01 => Ok(Width32),
            0x02 => Ok(Width64),
            _ => Err(ParseError::InvalidWordWidth(b)),
        }
    }

    pub fn size(&self) -> usize {
        match *self {
            WordWidth::Width32 => 4,
            WordWidth::Width64 => 8,
        }
    }
}

impl Endianness {
    pub(crate) fn parse_byte(b: u8) -> Result<Endianness> {
        use Endianness::*;
        match b {
            0x01 => Ok(Little),
            0x02 => Ok(Big),
            _ => Err(ParseError::InvalidEndianness(b)),
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
            _ => Unknown,
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
            _ => Unknown,
        }
    }

    pub(crate) fn parse_bytes(bytes: &[u8], endianness: Endianness) -> Result<Arch> {
        // allow larger slices as well. The number of read bytes is known statically
        if bytes.len() < 2 {
            Err(ParseError::InsufficientPartLength(bytes.len()))
        } else {
            Ok(Arch::from_u16(u16::from_bytes(bytes, endianness)))
        }
    }
}

impl Word {
    pub(crate) fn parse_bytes(
        bytes: &[u8],
        word_width: WordWidth,
        endianness: Endianness,
    ) -> Result<Word> {
        match word_width {
            WordWidth::Width32 => {
                if bytes.len() < 4 {
                    Err(ParseError::InsufficientPartLength(bytes.len()))
                } else {
                    Ok(Word::Word32(u32::from_bytes(bytes, endianness)))
                }
            }
            WordWidth::Width64 => {
                if bytes.len() < 8 {
                    Err(ParseError::InsufficientPartLength(bytes.len()))
                } else {
                    Ok(Word::Word64(u64::from_bytes(bytes, endianness)))
                }
            }
        }
    }

    pub fn size(&self) -> usize {
        match *self {
            Word::Word32(_) => 4,
            Word::Word64(_) => 8,
        }
    }

    pub fn zero(word_width: WordWidth) -> Self {
        match word_width {
            WordWidth::Width32 => Word::Word32(0),
            WordWidth::Width64 => Word::Word64(0),
        }
    }
}

impl From<Word> for u64 {
    fn from(word: Word) -> Self {
        match word {
            Word::Word32(i) => i as u64,
            Word::Word64(i) => i,
        }
    }
}

impl Debug for Word {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Word::Word32(u) => write!(f, "Word32({:#010x})", u),
            Word::Word64(u) => write!(f, "Word64({:#018x})", u),
        }
    }
}

impl LowerHex for Word {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Word::Word32(u) => LowerHex::fmt(&u, f),
            Word::Word64(u) => LowerHex::fmt(&u, f),
        }
    }
}

impl UpperHex for Word {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Word::Word32(u) => UpperHex::fmt(&u, f),
            Word::Word64(u) => UpperHex::fmt(&u, f),
        }
    }
}

impl Binary for Word {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Word::Word32(u) => Binary::fmt(&u, f),
            Word::Word64(u) => Binary::fmt(&u, f),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
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
            ([0xFF, 0x9D], Endianness::Big, FileType::Specific(0xFF9D)),
        ];
        for (data, endianness, expected) in test_data.iter() {
            assert_eq!(FileType::parse_bytes(data, *endianness), Ok(*expected));
        }
    }

    #[test]
    fn test_file_type_err() {
        let test_data = [
            (
                [0xE3, 0xE3],
                Endianness::Little,
                ParseError::InvalidFileType(0xE3E3),
            ),
            (
                [0xFF, 0xFE],
                Endianness::Little,
                ParseError::InvalidFileType(0xFEFF),
            ),
            (
                [0xFE, 0xFF],
                Endianness::Big,
                ParseError::InvalidFileType(0xFEFF),
            ),
        ];
        for (data, endianness, expected) in test_data.iter() {
            assert_eq!(
                FileType::parse_bytes(data, *endianness),
                Err(expected.clone())
            );
        }
    }

    #[test]
    fn test_word_width_ok() {
        let test_data = [(0x01, WordWidth::Width32), (0x02, WordWidth::Width64)];
        for (byte, expected) in test_data.iter() {
            assert_eq!(WordWidth::parse_byte(*byte), Ok(*expected));
        }
    }

    #[test]
    fn test_word_width_err() {
        let test_data = [0x00, 0x03, 0xFF, 0x3D];
        for &byte in test_data.iter() {
            assert_eq!(
                WordWidth::parse_byte(byte),
                Err(ParseError::InvalidWordWidth(byte))
            );
        }
    }

    #[test]
    fn test_endianness_ok() {
        let test_data = [(0x01, Endianness::Little), (0x02, Endianness::Big)];
        for (byte, expected) in test_data.iter() {
            assert_eq!(Endianness::parse_byte(*byte), Ok(*expected));
        }
    }

    #[test]
    fn test_endianness_err() {
        let test_data = [0x00, 0x03, 0xFF, 0x3D];
        for &byte in test_data.iter() {
            assert_eq!(
                Endianness::parse_byte(byte),
                Err(ParseError::InvalidEndianness(byte))
            );
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
            Unknown,
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
            (0xFFFF, Unknown),
        ];
        for (code, expected) in test_data.iter() {
            let bytes = code.to_le_bytes();
            assert_eq!(Arch::parse_bytes(&bytes, Endianness::Little), Ok(*expected));
        }
    }

    #[test]
    fn test_arch_err() {
        let test_data = [0x01_u8];
        assert_eq!(
            Arch::parse_bytes(&test_data, Endianness::Little),
            Err(ParseError::InsufficientPartLength(1))
        );
        assert_eq!(
            Arch::parse_bytes(&test_data, Endianness::Big),
            Err(ParseError::InsufficientPartLength(1))
        );
    }

    #[test]
    fn test_word_ok() {
        use Endianness::*;
        use Word::*;
        use WordWidth::*;
        let test_data = [
            (
                [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
                Width64,
                Little,
                Word64(0),
            ),
            (
                [0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
                Width32,
                Little,
                Word32(0x00000010),
            ),
            (
                [0xFF, 0x3E, 0x00, 0x00, 0x10, 0x20, 0x00, 0x00],
                Width64,
                Big,
                Word64(0xFF3E000010200000),
            ),
            (
                [0xFF, 0x3E, 0x00, 0x00, 0x10, 0x20, 0x00, 0x00],
                Width32,
                Big,
                Word32(0xFF3E0000),
            ),
        ];

        for (bytes, width, endianness, expected) in test_data.iter() {
            let result = Word::parse_bytes(bytes, *width, *endianness);
            assert!(result.is_ok());
            let result = result.expect("checked is_ok()");
            assert_eq!(result, *expected);
        }
    }
}
