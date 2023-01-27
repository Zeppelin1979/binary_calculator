use crate::{dec_formats::DecFormats, hex_formats::HexFormats, settings::SettingsMessage};

#[derive(Debug, Clone)]
pub(crate) enum Message {
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
    InputU32Changed(u32),
    SettingsMessage(SettingsMessage),
}
