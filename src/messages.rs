use crate::{dec_formats::DecFormats, hex_formats::HexFormats, settings::SettingsMessage};

#[derive(Debug, Clone)]
pub(crate) enum Message {
    InputChanged(String),
    DecInputChanged(String),
    HexInputChanged(String),
    ShiftLeft,
    ShiftRight,
    Not,
    HexCopy(HexFormats),
    DecCopy(DecFormats),
    SignToggled(bool),
    Settings,
    Main,
    Char3InputChanged(String),
    Char2InputChanged(String),
    Char1InputChanged(String),
    Char0InputChanged(String),
    InputU32Changed(u32),
    SettingsMessage(SettingsMessage),
}
