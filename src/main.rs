use cli_clipboard::{ClipboardContext, ClipboardProvider};
use iced::theme::{self, Theme};
use iced::widget::tooltip::Position;
use iced::widget::{
    button, checkbox, column, container, horizontal_rule, pick_list, radio, row, text, text_input,
    toggler, tooltip, vertical_rule,
};
use iced::{Alignment, Color, Element, Length, Sandbox, Settings};
use num_format::{Locale, ToFormattedString};

pub fn main() -> iced::Result {
    BinaryCalculator::run(Settings::default())
}

struct BinaryCalculator {
    theme: Theme,
    input_value: String,
    value: u32,
    signed: bool,
    page: Pages,
}

impl Default for BinaryCalculator {
    fn default() -> Self {
        Self {
            theme: Theme::Dark,
            input_value: Default::default(),
            value: Default::default(),
            signed: false,
            page: Pages::default(),
        }
    }
}

impl BinaryCalculator {
    fn put_bit(&mut self, bit: bool, offset: u8) {
        if bit {
            let mask = 1_u32 << offset;
            self.value |= mask;
        } else {
            let mask = !(1_u32 << offset);
            self.value &= mask;
        }
    }

    fn get_bit(&self, offset: u8) -> bool {
        let mask = 1_u32 << offset;
        self.value & mask > 0
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ThemeType {
    Light,
    Dark,
    Custom,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Pages {
    Main,
    Settings,
}

impl Default for Pages {
    fn default() -> Self {
        Self::Main
    }
}

#[derive(Debug, Clone)]
enum Message {
    ThemeChanged(ThemeType),
    InputChanged(String),
    DecInputChanged(String),
    HexInputChanged(String),
    ShiftLeft,
    ShiftRight,
    Not,
    Hex0,
    Hex1,
    Hex2,
    Hex3,
    Hex4,
    Hex5,
    Hex6,
    Hex7,
    Hex8,
    Hex9,
    HexA,
    HexB,
    HexC,
    HexD,
    HexE,
    HexF,
    Dec0,
    Dec1,
    Dec2,
    Dec3,
    Dec4,
    Dec5,
    Dec6,
    Dec7,
    Dec8,
    Dec9,
    DecSign,
    Bsp,
    Bit0CheckboxToggled(bool),
    Bit1CheckboxToggled(bool),
    Bit2CheckboxToggled(bool),
    Bit3CheckboxToggled(bool),
    Bit4CheckboxToggled(bool),
    Bit5CheckboxToggled(bool),
    Bit6CheckboxToggled(bool),
    Bit7CheckboxToggled(bool),
    Bit8CheckboxToggled(bool),
    Bit9CheckboxToggled(bool),
    Bit10CheckboxToggled(bool),
    Bit11CheckboxToggled(bool),
    Bit12CheckboxToggled(bool),
    Bit13CheckboxToggled(bool),
    Bit14CheckboxToggled(bool),
    Bit15CheckboxToggled(bool),
    Bit16CheckboxToggled(bool),
    Bit17CheckboxToggled(bool),
    Bit18CheckboxToggled(bool),
    Bit19CheckboxToggled(bool),
    Bit20CheckboxToggled(bool),
    Bit21CheckboxToggled(bool),
    Bit22CheckboxToggled(bool),
    Bit23CheckboxToggled(bool),
    Bit24CheckboxToggled(bool),
    Bit25CheckboxToggled(bool),
    Bit26CheckboxToggled(bool),
    Bit27CheckboxToggled(bool),
    Bit28CheckboxToggled(bool),
    Bit29CheckboxToggled(bool),
    Bit30CheckboxToggled(bool),
    Bit31CheckboxToggled(bool),
    HexCopy(HexFormats),
    DecCopy(DecFormats),
    SignToggled(bool),
    Settings,
    Main,
    Char3InputChanged(String),
    Char2InputChanged(String),
    Char1InputChanged(String),
    Char0InputChanged(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HexFormats {
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
    const ALL: [HexFormats; 12] = [
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DecFormats {
    Plain,
    PointSeperator,
    CommaSeperator,
}

impl DecFormats {
    const ALL: [Self; 3] = [Self::Plain, Self::PointSeperator, Self::CommaSeperator];
}

impl Default for DecFormats {
    fn default() -> Self {
        Self::Plain
    }
}

impl std::fmt::Display for DecFormats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Plain => "123456",
                Self::CommaSeperator => "123,456",
                Self::PointSeperator => "123.456",
            }
        )
    }
}

impl Sandbox for BinaryCalculator {
    type Message = Message;

    fn new() -> Self {
        BinaryCalculator::default()
    }

    fn title(&self) -> String {
        String::from("BinCalc")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ThemeChanged(theme) => {
                self.theme = match theme {
                    ThemeType::Light => Theme::Light,
                    ThemeType::Dark => Theme::Dark,
                    ThemeType::Custom => Theme::custom(theme::Palette {
                        background: Color::from_rgb(1.0, 0.9, 1.0),
                        text: Color::BLACK,
                        primary: Color::from_rgb(0.5, 0.5, 0.0),
                        success: Color::from_rgb(0.0, 1.0, 0.0),
                        danger: Color::from_rgb(1.0, 0.0, 0.0),
                    }),
                }
            }
            Message::InputChanged(value) => self.input_value = value,
            Message::Bit0CheckboxToggled(value) => self.put_bit(value, 0),
            Message::Bit1CheckboxToggled(value) => self.put_bit(value, 1),
            Message::Bit2CheckboxToggled(value) => self.put_bit(value, 2),
            Message::Bit3CheckboxToggled(value) => self.put_bit(value, 3),
            Message::Bit4CheckboxToggled(value) => self.put_bit(value, 4),
            Message::Bit5CheckboxToggled(value) => self.put_bit(value, 5),
            Message::Bit6CheckboxToggled(value) => self.put_bit(value, 6),
            Message::Bit7CheckboxToggled(value) => self.put_bit(value, 7),
            Message::Bit8CheckboxToggled(value) => self.put_bit(value, 8),
            Message::Bit9CheckboxToggled(value) => self.put_bit(value, 9),
            Message::Bit10CheckboxToggled(value) => self.put_bit(value, 10),
            Message::Bit11CheckboxToggled(value) => self.put_bit(value, 11),
            Message::Bit12CheckboxToggled(value) => self.put_bit(value, 12),
            Message::Bit13CheckboxToggled(value) => self.put_bit(value, 13),
            Message::Bit14CheckboxToggled(value) => self.put_bit(value, 14),
            Message::Bit15CheckboxToggled(value) => self.put_bit(value, 15),
            Message::Bit16CheckboxToggled(value) => self.put_bit(value, 16),
            Message::Bit17CheckboxToggled(value) => self.put_bit(value, 17),
            Message::Bit18CheckboxToggled(value) => self.put_bit(value, 18),
            Message::Bit19CheckboxToggled(value) => self.put_bit(value, 19),
            Message::Bit20CheckboxToggled(value) => self.put_bit(value, 20),
            Message::Bit21CheckboxToggled(value) => self.put_bit(value, 21),
            Message::Bit22CheckboxToggled(value) => self.put_bit(value, 22),
            Message::Bit23CheckboxToggled(value) => self.put_bit(value, 23),
            Message::Bit24CheckboxToggled(value) => self.put_bit(value, 24),
            Message::Bit25CheckboxToggled(value) => self.put_bit(value, 25),
            Message::Bit26CheckboxToggled(value) => self.put_bit(value, 26),
            Message::Bit27CheckboxToggled(value) => self.put_bit(value, 27),
            Message::Bit28CheckboxToggled(value) => self.put_bit(value, 28),
            Message::Bit29CheckboxToggled(value) => self.put_bit(value, 29),
            Message::Bit30CheckboxToggled(value) => self.put_bit(value, 30),
            Message::Bit31CheckboxToggled(value) => self.put_bit(value, 31),
            Message::ShiftLeft => self.value <<= 1,
            Message::ShiftRight => self.value >>= 1,
            Message::Not => self.value = !self.value,
            Message::Hex0 => self.value <<= 4,
            Message::Hex1 => self.value = (self.value << 4).wrapping_add(1),
            Message::Hex2 => self.value = (self.value << 4).wrapping_add(2),
            Message::Hex3 => self.value = (self.value << 4).wrapping_add(3),
            Message::Hex4 => self.value = (self.value << 4).wrapping_add(4),
            Message::Hex5 => self.value = (self.value << 4).wrapping_add(5),
            Message::Hex6 => self.value = (self.value << 4).wrapping_add(6),
            Message::Hex7 => self.value = (self.value << 4).wrapping_add(7),
            Message::Hex8 => self.value = (self.value << 4).wrapping_add(8),
            Message::Hex9 => self.value = (self.value << 4).wrapping_add(9),
            Message::HexA => self.value = (self.value << 4).wrapping_add(10),
            Message::HexB => self.value = (self.value << 4).wrapping_add(11),
            Message::HexC => self.value = (self.value << 4).wrapping_add(12),
            Message::HexD => self.value = (self.value << 4).wrapping_add(13),
            Message::HexE => self.value = (self.value << 4).wrapping_add(14),
            Message::HexF => self.value = (self.value << 4).wrapping_add(15),
            Message::Dec0 => self.value = self.value.wrapping_mul(10),
            Message::Dec1 => self.value = self.value.wrapping_mul(10).wrapping_add(1),
            Message::Dec2 => self.value = self.value.wrapping_mul(10).wrapping_add(2),
            Message::Dec3 => self.value = self.value.wrapping_mul(10).wrapping_add(3),
            Message::Dec4 => self.value = self.value.wrapping_mul(10).wrapping_add(4),
            Message::Dec5 => self.value = self.value.wrapping_mul(10).wrapping_add(5),
            Message::Dec6 => self.value = self.value.wrapping_mul(10).wrapping_add(6),
            Message::Dec7 => self.value = self.value.wrapping_mul(10).wrapping_add(7),
            Message::Dec8 => self.value = self.value.wrapping_mul(10).wrapping_add(8),
            Message::Dec9 => self.value = self.value.wrapping_mul(10).wrapping_add(9),
            Message::DecSign => todo!(),
            Message::Bsp => self.value /= 10,
            Message::DecInputChanged(value) => {
                if value.is_empty() {
                    self.value = 0
                } else if let Ok(val) = value.replace(['.', ',', ' '], "").parse() {
                    self.value = val;
                }
            }
            Message::HexInputChanged(value) => {
                if value.is_empty() {
                    self.value = 0
                } else if let Ok(val) = u32::from_str_radix(&value.replace(['.', ',', ' '], ""), 16)
                {
                    self.value = val;
                }
            }
            Message::HexCopy(format) => {
                if let Ok(mut clipboard) = ClipboardContext::new() {
                    match format {
                        HexFormats::MotorolaSmall1Block => {
                            clipboard
                                .set_contents(format!("{:08x}", self.value))
                                .unwrap();
                        }
                        HexFormats::MotorolaSmall2Blocks => clipboard
                            .set_contents(format!(
                                "{:04x} {:04x}",
                                (self.value >> 16) & 0xFFFF,
                                self.value & 0xFFFF
                            ))
                            .unwrap(),
                        HexFormats::MotorolaSmall4Blocks => clipboard
                            .set_contents(format!(
                                "{:02x} {:02x} {:02x} {:02x}",
                                (self.value >> 24) & 0xFF,
                                (self.value >> 16) & 0xFF,
                                (self.value >> 8) & 0xFF,
                                self.value & 0xFF
                            ))
                            .unwrap(),
                        HexFormats::MotorolaSmall1BlockWithX => clipboard
                            .set_contents(format!("{:#010x}", self.value))
                            .unwrap(),
                        HexFormats::MotorolaSmall2BlocksWithX => clipboard
                            .set_contents(format!(
                                "{:#06x} {:#06x}",
                                (self.value >> 16) & 0xFFFF,
                                self.value & 0xFFFF
                            ))
                            .unwrap(),
                        HexFormats::MotorolaSmall4BlocksWithX => clipboard
                            .set_contents(format!(
                                "{:#04x} {:#04x} {:#04x} {:#04x}",
                                (self.value >> 24) & 0xFF,
                                (self.value >> 16) & 0xFF,
                                (self.value >> 8) & 0xFF,
                                self.value & 0xFF
                            ))
                            .unwrap(),
                        HexFormats::MotorolaSmall4BlocksWithXWithBrackets => clipboard
                            .set_contents(format!(
                                "[{:#04x}] [{:#04x}] [{:#04x}] [{:#04x}]",
                                (self.value >> 24) & 0xFF,
                                (self.value >> 16) & 0xFF,
                                (self.value >> 8) & 0xFF,
                                self.value & 0xFF
                            ))
                            .unwrap(),
                        HexFormats::MotorolaArray => clipboard
                            .set_contents(format!(
                                "[{:#04x}, {:#04x}, {:#04x}, {:#04x}]",
                                (self.value >> 24) & 0xFF,
                                (self.value >> 16) & 0xFF,
                                (self.value >> 8) & 0xFF,
                                self.value & 0xFF
                            ))
                            .unwrap(),
                        HexFormats::Intel4Blocks => clipboard
                            .set_contents(format!(
                                "{:04x} {:04x} {:04x} {:04x}",
                                self.value & 0xFF,
                                (self.value >> 8) & 0xFF,
                                (self.value >> 16) & 0xFF,
                                (self.value >> 24) & 0xFF
                            ))
                            .unwrap(),
                        HexFormats::Intel4BlocksWithX => clipboard
                            .set_contents(format!(
                                "{:#04x} {:#04x} {:#04x} {:#04x}",
                                self.value & 0xFF,
                                (self.value >> 8) & 0xFF,
                                (self.value >> 16) & 0xFF,
                                (self.value >> 24) & 0xFF
                            ))
                            .unwrap(),
                        HexFormats::Intel4BlocksWithXWitchBrackets => clipboard
                            .set_contents(format!(
                                "[{:#04x}] [{:#04x}] [{:#04x}] [{:#04x}]",
                                self.value & 0xFF,
                                (self.value >> 8) & 0xFF,
                                (self.value >> 16) & 0xFF,
                                (self.value >> 24) & 0xFF
                            ))
                            .unwrap(),
                        HexFormats::IntelArray => clipboard
                            .set_contents(format!(
                                "[{:#04x}, {:#04x}, {:#04x}, {:#04x}]",
                                self.value & 0xFF,
                                (self.value >> 8) & 0xFF,
                                (self.value >> 16) & 0xFF,
                                (self.value >> 24) & 0xFF
                            ))
                            .unwrap(),
                    };
                }
            }
            Message::DecCopy(format) => {
                if let Ok(mut clipboard) = ClipboardContext::new() {
                    match format {
                        DecFormats::Plain => {
                            clipboard.set_contents(self.value.to_string()).unwrap()
                        }
                        DecFormats::PointSeperator => clipboard
                            .set_contents(self.value.to_formatted_string(&Locale::de))
                            .unwrap(),
                        DecFormats::CommaSeperator => clipboard
                            .set_contents(self.value.to_formatted_string(&Locale::en))
                            .unwrap(),
                    }
                }
            }
            Message::SignToggled(value) => self.signed = value,
            Message::Settings => self.page = Pages::Settings,
            Message::Main => self.page = Pages::Main,
            Message::Char3InputChanged(value) => {
                if value.len() == 1 {
                    let char = (value.bytes().next().unwrap() as u32) << 24;
                    self.value &= 0x00FFFFFF;
                    self.value |= char;
                }
            }
            Message::Char2InputChanged(value) => {
                if value.len() == 1 {
                    let char = (value.bytes().next().unwrap() as u32) << 16;
                    self.value &= 0xFF00FFFF;
                    self.value |= char;
                }
            }
            Message::Char1InputChanged(value) => {
                if value.len() == 1 {
                    let char = (value.bytes().next().unwrap() as u32) << 8;
                    self.value &= 0xFFFF00FF;
                    self.value |= char;
                }
            }
            Message::Char0InputChanged(value) => {
                if value.len() == 1 {
                    let char = (value.bytes().next().unwrap() as u32);
                    self.value &= 0xFFFFFF00;
                    self.value |= char;
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        match self.page {
            Pages::Main => {
                let settings_button = button("Settings").on_press(Message::Settings);
                let shift_left_button = button("<<").on_press(Message::ShiftLeft);
                let binary_text_input = text_input(
                    "",
                    format!("{:032b}", self.value).as_str(),
                    Message::InputChanged,
                );
                let shift_right_button = button(">>").on_press(Message::ShiftRight);
                let not_button = button("Not").on_press(Message::Not);
                let char3_text_input = text_input(
                    "",
                    format!("{}", char::from_u32((self.value >> 24) & 0xFF).unwrap()).as_str(),
                    Message::Char3InputChanged,
                )
                .width(Length::Units(20));
                let char2_text_input = text_input(
                    "",
                    format!("{}", char::from_u32((self.value >> 16) & 0xFF).unwrap()).as_str(),
                    Message::Char2InputChanged,
                )
                .width(Length::Units(20));
                let char1_text_input = text_input(
                    "",
                    format!("{}", char::from_u32((self.value >> 8) & 0xFF).unwrap()).as_str(),
                    Message::Char1InputChanged,
                )
                .width(Length::Units(20));
                let char0_text_input = text_input(
                    "",
                    format!("{}", char::from_u32((self.value) & 0xFF).unwrap()).as_str(),
                    Message::Char0InputChanged,
                )
                .width(Length::Units(20));
                let bit0_checkbox =
                    checkbox("", self.get_bit(0), Message::Bit0CheckboxToggled).spacing(0);
                let bit1_checkbox =
                    checkbox("", self.get_bit(1), Message::Bit1CheckboxToggled).spacing(0);
                let bit2_checkbox =
                    checkbox("", self.get_bit(2), Message::Bit2CheckboxToggled).spacing(0);
                let bit3_checkbox =
                    checkbox("", self.get_bit(3), Message::Bit3CheckboxToggled).spacing(0);
                let bit4_checkbox =
                    checkbox("", self.get_bit(4), Message::Bit4CheckboxToggled).spacing(0);
                let bit5_checkbox =
                    checkbox("", self.get_bit(5), Message::Bit5CheckboxToggled).spacing(0);
                let bit6_checkbox =
                    checkbox("", self.get_bit(6), Message::Bit6CheckboxToggled).spacing(0);
                let bit7_checkbox =
                    checkbox("", self.get_bit(7), Message::Bit7CheckboxToggled).spacing(0);
                let bit8_checkbox =
                    checkbox("", self.get_bit(8), Message::Bit8CheckboxToggled).spacing(0);
                let bit9_checkbox =
                    checkbox("", self.get_bit(9), Message::Bit9CheckboxToggled).spacing(0);
                let bit10_checkbox =
                    checkbox("", self.get_bit(10), Message::Bit10CheckboxToggled).spacing(0);
                let bit11_checkbox =
                    checkbox("", self.get_bit(11), Message::Bit11CheckboxToggled).spacing(0);
                let bit12_checkbox =
                    checkbox("", self.get_bit(12), Message::Bit12CheckboxToggled).spacing(0);
                let bit13_checkbox =
                    checkbox("", self.get_bit(13), Message::Bit13CheckboxToggled).spacing(0);
                let bit14_checkbox =
                    checkbox("", self.get_bit(14), Message::Bit14CheckboxToggled).spacing(0);
                let bit15_checkbox =
                    checkbox("", self.get_bit(15), Message::Bit15CheckboxToggled).spacing(0);
                let bit16_checkbox =
                    checkbox("", self.get_bit(16), Message::Bit16CheckboxToggled).spacing(0);
                let bit17_checkbox =
                    checkbox("", self.get_bit(17), Message::Bit17CheckboxToggled).spacing(0);
                let bit18_checkbox =
                    checkbox("", self.get_bit(18), Message::Bit18CheckboxToggled).spacing(0);
                let bit19_checkbox =
                    checkbox("", self.get_bit(19), Message::Bit19CheckboxToggled).spacing(0);
                let bit20_checkbox =
                    checkbox("", self.get_bit(20), Message::Bit20CheckboxToggled).spacing(0);
                let bit21_checkbox =
                    checkbox("", self.get_bit(21), Message::Bit21CheckboxToggled).spacing(0);
                let bit22_checkbox =
                    checkbox("", self.get_bit(22), Message::Bit22CheckboxToggled).spacing(0);
                let bit23_checkbox =
                    checkbox("", self.get_bit(23), Message::Bit23CheckboxToggled).spacing(0);
                let bit24_checkbox =
                    checkbox("", self.get_bit(24), Message::Bit24CheckboxToggled).spacing(0);
                let bit25_checkbox =
                    checkbox("", self.get_bit(25), Message::Bit25CheckboxToggled).spacing(0);
                let bit26_checkbox =
                    checkbox("", self.get_bit(26), Message::Bit26CheckboxToggled).spacing(0);
                let bit27_checkbox =
                    checkbox("", self.get_bit(27), Message::Bit27CheckboxToggled).spacing(0);
                let bit28_checkbox =
                    checkbox("", self.get_bit(28), Message::Bit28CheckboxToggled).spacing(0);
                let bit29_checkbox =
                    checkbox("", self.get_bit(29), Message::Bit29CheckboxToggled).spacing(0);
                let bit30_checkbox =
                    checkbox("", self.get_bit(30), Message::Bit30CheckboxToggled).spacing(0);
                let bit31_checkbox =
                    checkbox("", self.get_bit(31), Message::Bit31CheckboxToggled).spacing(0);
                let hexadecimal_text_input = text_input(
                    "",
                    format!(
                        "{:04x} {:04x}",
                        (self.value >> 16) & 0xFFFF,
                        self.value & 0xFFFF
                    )
                    .as_str(),
                    Message::HexInputChanged,
                )
                .width(Length::Units(200));
                let hex_pick_list = pick_list(&HexFormats::ALL[..], None, Message::HexCopy)
                    .placeholder("Copy to clipboard")
                    .width(Length::Units(200));
                let hex_0_button = button("0").on_press(Message::Hex0).width(Length::Units(40));
                let hex_1_button = button("1").on_press(Message::Hex1).width(Length::Units(40));
                let hex_2_button = button("2").on_press(Message::Hex2).width(Length::Units(40));
                let hex_3_button = button("3").on_press(Message::Hex3).width(Length::Units(40));
                let hex_4_button = button("4").on_press(Message::Hex4).width(Length::Units(40));
                let hex_5_button = button("5").on_press(Message::Hex5).width(Length::Units(40));
                let hex_6_button = button("6").on_press(Message::Hex6).width(Length::Units(40));
                let hex_7_button = button("7").on_press(Message::Hex7).width(Length::Units(40));
                let hex_8_button = button("8").on_press(Message::Hex8).width(Length::Units(40));
                let hex_9_button = button("9").on_press(Message::Hex9).width(Length::Units(40));
                let hex_a_button = button("A").on_press(Message::HexA).width(Length::Units(40));
                let hex_b_button = button("B").on_press(Message::HexB).width(Length::Units(40));
                let hex_c_button = button("C").on_press(Message::HexC).width(Length::Units(40));
                let hex_d_button = button("D").on_press(Message::HexD).width(Length::Units(40));
                let hex_e_button = button("E").on_press(Message::HexE).width(Length::Units(40));
                let hex_f_button = button("F").on_press(Message::HexF).width(Length::Units(40));

                let signed_toogler = toggler(
                    if self.signed {
                        "signed".to_string()
                    } else {
                        "unsigned".to_string()
                    },
                    self.signed,
                    Message::SignToggled,
                )
                .width(Length::Shrink);
                let value = if self.signed {
                    (self.value as i32).to_formatted_string(&Locale::de)
                } else {
                    self.value.to_formatted_string(&Locale::de)
                };
                let decimal_text_input =
                    text_input("", &value, Message::DecInputChanged).width(Length::Units(150));
                let dec_pick_list = pick_list(&DecFormats::ALL[..], None, Message::DecCopy)
                    .placeholder("Copy to clipboard")
                    .width(Length::Units(200));
                let dec_0_button = button("0").on_press(Message::Dec0).width(Length::Units(40));
                let dec_1_button = button("1").on_press(Message::Dec1).width(Length::Units(40));
                let dec_2_button = button("2").on_press(Message::Dec2).width(Length::Units(40));
                let dec_3_button = button("3").on_press(Message::Dec3).width(Length::Units(40));
                let dec_4_button = button("4").on_press(Message::Dec4).width(Length::Units(40));
                let dec_5_button = button("5").on_press(Message::Dec5).width(Length::Units(40));
                let dec_6_button = button("6").on_press(Message::Dec6).width(Length::Units(40));
                let dec_7_button = button("7").on_press(Message::Dec7).width(Length::Units(40));
                let dec_8_button = button("8").on_press(Message::Dec8).width(Length::Units(40));
                let dec_9_button = button("9").on_press(Message::Dec9).width(Length::Units(40));
                let dec_sign_button = button("±")
                    .on_press(Message::DecSign)
                    .width(Length::Units(40));
                let dec_res_button = button("<-").on_press(Message::Bsp).width(Length::Units(40));

                /*        let toggler = toggler(
                    String::from("Toggle me!"),
                    self.toggler_value,
                    Message::TogglerToggled,
                )
                .width(Length::Shrink)
                .spacing(10);*/

                let content = column![
                    settings_button,
                    row![
                        shift_left_button,
                        binary_text_input,
                        shift_right_button,
                        not_button,
                        text("Char"),
                        char3_text_input,
                        char2_text_input,
                        char1_text_input,
                        char0_text_input,
                    ]
                    .spacing(4)
                    .align_items(Alignment::Center),
                    row![
                        row![
                            column![
                                text("31"),
                                tooltip(bit31_checkbox, "31 [2G]", Position::FollowCursor)
                            ]
                            .align_items(Alignment::Center),
                            tooltip(bit30_checkbox, "30 [1G]", Position::FollowCursor),
                            tooltip(bit29_checkbox, "29 [512M]", Position::FollowCursor),
                            column![
                                text("28"),
                                tooltip(bit28_checkbox, "28 [256M]", Position::FollowCursor)
                            ]
                            .align_items(Alignment::Center)
                        ]
                        .spacing(1)
                        .align_items(Alignment::End),
                        row![
                            column![
                                text("27"),
                                tooltip(bit27_checkbox, "27 [128M]", Position::FollowCursor)
                            ]
                            .align_items(Alignment::Center),
                            tooltip(bit26_checkbox, "26 [64M]", Position::FollowCursor),
                            tooltip(bit25_checkbox, "25 [32M]", Position::FollowCursor),
                            column![
                                text("24"),
                                tooltip(bit24_checkbox, "24 [16M]", Position::FollowCursor)
                            ]
                            .align_items(Alignment::Center)
                        ]
                        .spacing(1)
                        .align_items(Alignment::End),
                        row![
                            column![
                                text("23"),
                                tooltip(bit23_checkbox, "23 [8M]", Position::FollowCursor)
                            ]
                            .align_items(Alignment::Center),
                            tooltip(bit22_checkbox, "22 [4M]", Position::FollowCursor),
                            tooltip(bit21_checkbox, "21 [2M]", Position::FollowCursor),
                            column![
                                text("20"),
                                tooltip(bit20_checkbox, "20 [1M]", Position::FollowCursor)
                            ]
                            .align_items(Alignment::Center),
                        ]
                        .spacing(1)
                        .align_items(Alignment::End),
                        row![
                            column![
                                text("19"),
                                tooltip(bit19_checkbox, "19 [512K]", Position::FollowCursor)
                            ]
                            .align_items(Alignment::Center),
                            tooltip(bit18_checkbox, "18 [256K]", Position::FollowCursor),
                            tooltip(bit17_checkbox, "17 [128K]", Position::FollowCursor),
                            column![
                                text("16"),
                                tooltip(bit16_checkbox, "16 [64K]", Position::FollowCursor)
                            ]
                            .align_items(Alignment::Center),
                        ]
                        .spacing(1)
                        .align_items(Alignment::End),
                        row![
                            column![
                                text("15"),
                                tooltip(bit15_checkbox, "15 [32K]", Position::FollowCursor)
                            ]
                            .align_items(Alignment::Center),
                            tooltip(bit14_checkbox, "14 [16K]", Position::FollowCursor),
                            tooltip(bit13_checkbox, "13 [8K]", Position::FollowCursor),
                            column![
                                text("12"),
                                tooltip(bit12_checkbox, "12 [4K]", Position::FollowCursor)
                            ]
                            .align_items(Alignment::Center),
                        ]
                        .spacing(1)
                        .align_items(Alignment::End),
                        row![
                            column![
                                text("11"),
                                tooltip(bit11_checkbox, "11 [2K]", Position::FollowCursor)
                            ]
                            .align_items(Alignment::Center),
                            tooltip(bit10_checkbox, "10 [1K]", Position::FollowCursor),
                            tooltip(bit9_checkbox, "9 [512]", Position::FollowCursor),
                            column![
                                text("8"),
                                tooltip(bit8_checkbox, "8 [256]", Position::FollowCursor)
                            ]
                            .align_items(Alignment::Center),
                        ]
                        .spacing(1)
                        .align_items(Alignment::End),
                        row![
                            column![
                                text("7"),
                                tooltip(bit7_checkbox, "7 [128]", Position::FollowCursor)
                            ]
                            .align_items(Alignment::Center),
                            tooltip(bit6_checkbox, "6 [64]", Position::FollowCursor),
                            tooltip(bit5_checkbox, "5 [32]", Position::FollowCursor),
                            column![
                                text("4"),
                                tooltip(bit4_checkbox, "4 [16]", Position::FollowCursor)
                            ]
                            .align_items(Alignment::Center),
                        ]
                        .spacing(1)
                        .align_items(Alignment::End),
                        row![
                            column![
                                text("3"),
                                tooltip(bit3_checkbox, "3 [8]", Position::FollowCursor)
                            ]
                            .align_items(Alignment::Center),
                            tooltip(bit2_checkbox, "2 [4]", Position::FollowCursor),
                            tooltip(bit1_checkbox, "1 [2]", Position::FollowCursor),
                            column![
                                text("0"),
                                tooltip(bit0_checkbox, "0 [1]", Position::FollowCursor)
                            ]
                            .align_items(Alignment::Center),
                        ]
                        .spacing(1)
                        .align_items(Alignment::End),
                    ]
                    .spacing(4)
                    .align_items(Alignment::End),
                    row![
                        column![
                            text("Hexadecimal"),
                            hexadecimal_text_input,
                            hex_pick_list,
                            row![hex_7_button, hex_8_button, hex_9_button, hex_f_button]
                                .spacing(10),
                            row![hex_4_button, hex_5_button, hex_6_button, hex_e_button]
                                .spacing(10),
                            row![hex_1_button, hex_2_button, hex_3_button, hex_d_button]
                                .spacing(10),
                            row![hex_0_button, hex_a_button, hex_b_button, hex_c_button]
                                .spacing(10),
                        ]
                        .spacing(10),
                        vertical_rule(38),
                        column![
                            row![text("Decimal"), signed_toogler].spacing(10),
                            decimal_text_input,
                            dec_pick_list,
                            row![dec_7_button, dec_8_button, dec_9_button].spacing(10),
                            row![dec_4_button, dec_5_button, dec_6_button].spacing(10),
                            row![dec_1_button, dec_2_button, dec_3_button].spacing(10),
                            row![dec_0_button, dec_sign_button, dec_res_button].spacing(10),
                        ]
                        .spacing(10),
                        vertical_rule(38),
                        column![text("Octal")].spacing(10),
                    ]
                    .spacing(20),
                ]
                .spacing(20)
                .padding(20)
                .max_width(1000);

                container(content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    //                    .center_y()
                    .into()
            }
            Pages::Settings => {
                let main_button = button("Back").on_press(Message::Main);
                let choose_theme = [ThemeType::Light, ThemeType::Dark, ThemeType::Custom]
                    .iter()
                    .fold(
                        column![text("Choose a theme:")].spacing(10),
                        |column, theme| {
                            column.push(radio(
                                format!("{:?}", theme),
                                *theme,
                                Some(match self.theme {
                                    Theme::Light => ThemeType::Light,
                                    Theme::Dark => ThemeType::Dark,
                                    Theme::Custom { .. } => ThemeType::Custom,
                                }),
                                Message::ThemeChanged,
                            ))
                        },
                    );
                let content = column![main_button, horizontal_rule(38), choose_theme,]
                    .spacing(20)
                    .padding(20)
                    .max_width(1000);
                container(content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    //.center_y()
                    .into()
            }
        }
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
