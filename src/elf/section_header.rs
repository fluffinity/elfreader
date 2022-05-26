use std::ffi::{CString, OsString};

use super::{Endianness, FromBytesEndianned, ParseError, Result, Word, WordWidth};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionHeaderType {
    Null,
    ProgramBits,
    SymbolTable,
    StringTable,
    RelocationWithAddends,
    Hash,
    Dynamic,
    Note,
    NoData,
    Relocation,
    SharedLib,
    DynamicSymbolTable,
    ConstructorArray,
    DestructorArray,
    PreConstructorArray,
    Group,
    SectionIndices,
    Num,
    OsSpecific(u32),
}

bitflags::bitflags! {
    pub struct SectionHeaderFlags: u64{
        const WRITE = 0x1;
        const ALLOC = 0x2;
        const EXEC = 0x4;
        const MERGE = 0x10;
        const STRINGS = 0x20;
        const INFO_LINK = 0x40;
        const LINK_ORDER = 0x80;
        const OS_NONCONFORMING = 0x100;
        const GROUP = 0x200;
        const THREAD_LOCAL = 0x400;
        const MASK_OS = 0x0FF00000;
        const MASK_PROCESSOR = 0xF0000000;
        const ORDERED = 0x4000000;
        const EXCLUDE = 0x8000000;
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct UnnamedSectionHeader {
    name_index: u32,
    typ: SectionHeaderType,
    flags: SectionHeaderFlags,
    address: Word,
    pub(super) offset: Word,
    pub(super) size: Word,
    link: u32,
    info: u32,
    align: Word,
    entry_size: Word,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SectionHeader {
    name: String,
    typ: SectionHeaderType,
    flags: SectionHeaderFlags,
    address: Word,
    offset: Word,
    size: Word,
    link: u32,
    info: u32,
    align: Word,
    entry_size: Word,
}

impl SectionHeaderType {
    pub fn parse_bytes(bytes: &[u8], endianness: Endianness) -> Result<Self> {
        SectionHeaderType::check_length(4, bytes.len())?;
        SectionHeaderType::parse_u32(u32::from_bytes(bytes, endianness))
    }

    pub fn parse_u32(raw: u32) -> Result<Self> {
        match raw {
            0x0 => Ok(SectionHeaderType::Null),
            0x1 => Ok(SectionHeaderType::ProgramBits),
            0x2 => Ok(SectionHeaderType::SymbolTable),
            0x3 => Ok(SectionHeaderType::StringTable),
            0x4 => Ok(SectionHeaderType::RelocationWithAddends),
            0x5 => Ok(SectionHeaderType::Hash),
            0x6 => Ok(SectionHeaderType::Dynamic),
            0x7 => Ok(SectionHeaderType::Note),
            0x8 => Ok(SectionHeaderType::NoData),
            0x9 => Ok(SectionHeaderType::Relocation),
            0xA => Ok(SectionHeaderType::SharedLib),
            0xB => Ok(SectionHeaderType::DynamicSymbolTable),
            0xE => Ok(SectionHeaderType::ConstructorArray),
            0xF => Ok(SectionHeaderType::DestructorArray),
            0x10 => Ok(SectionHeaderType::PreConstructorArray),
            0x11 => Ok(SectionHeaderType::Group),
            0x12 => Ok(SectionHeaderType::SectionIndices),
            0x13 => Ok(SectionHeaderType::Num),
            _ if raw >= 0x60000000 => Ok(SectionHeaderType::OsSpecific(raw)),
            _ => Err(ParseError::InvalidSectionHeaderType(raw)),
        }
    }

    fn check_length(expected: usize, actual: usize) -> Result<()> {
        if actual < expected {
            Err(ParseError::InsufficientPartLength(actual))
        } else {
            Ok(())
        }
    }
}

impl SectionHeaderFlags {
    pub fn parse_bytes(
        bytes: &[u8],
        word_width: WordWidth,
        endianness: Endianness,
    ) -> Result<Self> {
        SectionHeaderFlags::check_length(word_width.size(), bytes.len())?;
        let raw = match word_width {
            WordWidth::Width32 => u32::from_bytes(bytes, endianness) as u64,
            WordWidth::Width64 => u64::from_bytes(bytes, endianness),
        };
        SectionHeaderFlags::parse_u64(raw)
    }

    pub fn parse_u64(raw: u64) -> Result<Self> {
        Self::from_bits(raw).ok_or(ParseError::InvalidSectionHeaderFlags(raw))
    }

    fn check_length(expected: usize, actual: usize) -> Result<()> {
        if actual < expected {
            Err(ParseError::InsufficientPartLength(actual))
        } else {
            Ok(())
        }
    }
}

impl UnnamedSectionHeader {
    pub fn parse_bytes(
        bytes: &[u8],
        word_width: WordWidth,
        endianness: Endianness,
    ) -> Result<Self> {
        let expected_length = match word_width {
            WordWidth::Width32 => 40,
            WordWidth::Width64 => 64,
        };
        UnnamedSectionHeader::check_length(expected_length, bytes.len())?;
        // these are the word width dependent offsets of the fields:
        // [name_index, typ, flags, address, offset, size, link, info, align, entry_size]
        let offsets = match word_width {
            WordWidth::Width32 => [0, 4, 8, 12, 16, 20, 24, 28, 32, 36],
            WordWidth::Width64 => [0, 4, 8, 16, 24, 32, 40, 44, 48, 56],
        };

        let name_index = u32::from_bytes(&bytes[offsets[0]..], endianness);
        let typ = SectionHeaderType::parse_bytes(&bytes[offsets[1]..], endianness)?;
        let flags = SectionHeaderFlags::parse_bytes(&bytes[offsets[2]..], word_width, endianness)?;
        let address = Word::parse_bytes(&bytes[offsets[3]..], word_width, endianness)?;
        let offset = Word::parse_bytes(&bytes[offsets[4]..], word_width, endianness)?;
        let size = Word::parse_bytes(&bytes[offsets[5]..], word_width, endianness)?;
        let link = u32::from_bytes(&bytes[offsets[6]..], endianness);
        let info = u32::from_bytes(&bytes[offsets[7]..], endianness);
        let align = Word::parse_bytes(&bytes[offsets[8]..], word_width, endianness)?;
        let align_num = u64::from(align);
        if align_num != 0 && !align_num.is_power_of_two() {
            return Err(ParseError::InvalidAlignment(align.into()));
        }
        let entry_size = Word::parse_bytes(&bytes[offsets[9]..], word_width, endianness)?;
        Ok(UnnamedSectionHeader {
            name_index,
            typ,
            flags,
            address,
            offset,
            size,
            link,
            info,
            align,
            entry_size,
        })
    }

    fn check_length(expected: usize, actual: usize) -> Result<()> {
        if actual < expected {
            Err(ParseError::InsufficientSectionHeaderLength(actual))
        } else {
            Ok(())
        }
    }

    pub fn to_named(self, names_table: &[u8]) -> Result<SectionHeader> {
        let index = self.name_index as usize;
        let name_bytes = &names_table[index..];
        let null_index = name_bytes
            .iter()
            .position(|byte| *byte == 0)
            .ok_or(ParseError::UnterminatedString)?;
        let name = CString::from_vec_with_nul(name_bytes[..=null_index].to_vec())
            .expect("checked for null byte");
        let name = name
            .into_string()
            .map_err(|err| ParseError::InvalidSectionName(err))?;
        Ok(SectionHeader {
            name,
            typ: self.typ,
            flags: self.flags,
            address: self.address,
            offset: self.offset,
            size: self.size,
            link: self.link,
            info: self.info,
            align: self.align,
            entry_size: self.entry_size,
        })
    }
}
