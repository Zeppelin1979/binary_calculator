#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HexFormats {
    MotorolaSmall1Block,
    MotorolaSmall2Blocks,
    MotorolaSmall4Blocks,
    MotorolaSmall1BlockWithX,
    MotorolaSmall2BlocksWithX,
    MotorolaSmall4BlocksWithX,
    MotorolaSmall4BlocksWithXWithBrackets,
    MotorolaArray,
    Intel4Blocks,
    Intel4BlocksWithX,
    Intel4BlocksWithXWitchBrackets,
    IntelArray,
}

impl HexFormats {
    pub const ALL: [HexFormats; 12] = [
        HexFormats::MotorolaSmall1Block,
        HexFormats::MotorolaSmall2Blocks,
        HexFormats::MotorolaSmall4Blocks,
        HexFormats::MotorolaSmall1BlockWithX,
        HexFormats::MotorolaSmall2BlocksWithX,
        HexFormats::MotorolaSmall4BlocksWithX,
        HexFormats::MotorolaSmall4BlocksWithXWithBrackets,
        HexFormats::MotorolaArray,
        HexFormats::Intel4Blocks,
        HexFormats::Intel4BlocksWithX,
        HexFormats::Intel4BlocksWithXWitchBrackets,
        HexFormats::IntelArray,
    ];
}

impl Default for HexFormats {
    fn default() -> Self {
        HexFormats::MotorolaSmall1Block
    }
}

impl std::fmt::Display for HexFormats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HexFormats::MotorolaSmall1Block => "0011eeff",
                HexFormats::MotorolaSmall2Blocks => "0011 ee ff",
                HexFormats::MotorolaSmall4Blocks => "00 11 ee ff",
                HexFormats::MotorolaSmall1BlockWithX => "0x0011eeff",
                HexFormats::MotorolaSmall2BlocksWithX => "0x0011 0xeeff",
                HexFormats::MotorolaSmall4BlocksWithX => "0x00 0x11 0xee 0xff",
                HexFormats::MotorolaSmall4BlocksWithXWithBrackets => "[0x00] [0x11] [0xee] [0xff]",
                HexFormats::MotorolaArray => "[0x00, 0x11, 0xee, 0xff]",
                HexFormats::Intel4Blocks => "ff ee 11 00",
                HexFormats::Intel4BlocksWithX => "0xff 0xee 0x11 0x00",
                HexFormats::Intel4BlocksWithXWitchBrackets => "[0xff] [0xee] [0x11] [0x00]",
                HexFormats::IntelArray => "[0xff, 0xee, 0x11, 0x00]",
            }
        )
    }
}
