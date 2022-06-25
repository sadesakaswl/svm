use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct SFileHeader {
    magic: [u8; 2], //SF
    filetype: u8,
    arch: u8,
    version: u8,
}
pub(crate) struct Magic([u8; 2]);

impl Default for Magic {
    fn default() -> Self {
        Self(*b"SF")
    }
}
impl SFileHeader {
    pub(crate) fn new(magic: Magic, filetype: FileType, arch: Arch, version: u8) -> Self {
        Self {
            magic: magic.0,
            filetype: FileType::into(filetype),
            arch: Arch::into(arch),
            version,
        }
    }
}
pub(crate) enum FileType {
    Executable,
    Shared,
}

pub(crate) enum Arch {
    SS64,
    IA32,
    AMD64,
    ARM32,
    ARM64,
    WASM32,
    WASM64,
}

pub(crate) enum Version {
    V1,
}

impl From<u8> for Version {
    fn from(val: u8) -> Self {
        match val {
            0 => Version::V1,
            _ => Version::V1,
        }
    }
}
impl From<Version> for u8 {
    fn from(val: Version) -> Self {
        match val {
            Version::V1 => 0,
        }
    }
}

impl From<u8> for FileType {
    fn from(val: u8) -> Self {
        match val {
            0 => FileType::Executable,
            1 => FileType::Shared,
            _ => FileType::Shared,
        }
    }
}
impl From<FileType> for u8 {
    fn from(val: FileType) -> Self {
        match val {
            FileType::Executable => 0,
            FileType::Shared => 1,
        }
    }
}

impl From<Arch> for u8 {
    fn from(val: Arch) -> Self {
        match val {
            Arch::SS64 => 0,
            Arch::IA32 => 1,
            Arch::AMD64 => 2,
            Arch::ARM32 => 3,
            Arch::ARM64 => 4,
            Arch::WASM32 => 5,
            Arch::WASM64 => 6,
        }
    }
}
