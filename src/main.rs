mod binary_field_widget;
mod dec_formats;
mod decimal_input_widget;
mod hex_formats;
mod language_type;
mod messages;
mod settings;
mod theme_type;

use binary_field_widget::BinaryFieldWidget;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use dec_formats::DecFormats;
use decimal_input_widget::DecimalInputWidget;
use hex_formats::HexFormats;
use iced::theme::Theme;
use iced::widget::{
    button, column, container, pick_list, row, text, text_input, toggler, vertical_rule,
};
use iced::{window, Alignment, Element, Length, Sandbox, Settings};
use messages::Message;
use num_format::{Locale, ToFormattedString};
use settings::BinaryCalulatorSettings;

pub fn main() -> iced::Result {
    let settings = Settings {
        window: window::Settings {
            size: (800, 500),
            resizable: false,
            ..Default::default()
        },
        ..Default::default()
    };
    BinaryCalculator::run(settings)
}

struct BinaryCalculator {
    input_value: String,
    value: u32,
    signed: bool,
    page: Pages,
    settings: BinaryCalulatorSettings,
}

impl Default for BinaryCalculator {
    fn default() -> Self {
        Self {
            input_value: Default::default(),
            value: Default::default(),
            signed: false,
            page: Pages::default(),
            settings: BinaryCalulatorSettings::new(),
        }
    }
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

impl Sandbox for BinaryCalculator {
    type Message = Message;

    fn new() -> Self {
        BinaryCalculator::default()
    }

    fn title(&self) -> String {
        String::from("Binary Calculator")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(value) => self.input_value = value,
            Message::ShiftLeft => self.value <<= 1,
            Message::ShiftRight => self.value >>= 1,
            Message::Not => self.value = !self.value,
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
            Message::HexCopy(format) => self.copy_hex_to_clipboard(format),
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
                    let char = value.bytes().next().unwrap() as u32;
                    self.value &= 0xFFFFFF00;
                    self.value |= char;
                }
            }
            Message::InputU32Changed(value) => self.value = value,
            Message::SettingsMessage(msg) => self.settings.update(msg),
        }
    }

    fn view(&self) -> Element<Message> {
        let main_button = button(self.settings.main_str()).on_press(Message::Main);
        let settings_button = button(self.settings.setting_str()).on_press(Message::Settings);
        let header_row = row![main_button, settings_button].spacing(10);
        let content: Element<Message> = match self.page {
            Pages::Main => {
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
                let binary_field_widget =
                    BinaryFieldWidget::new(self.value, Message::InputU32Changed);
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
                    .placeholder(self.settings.copy_to_clipboard_str())
                    .width(Length::Units(200));
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
                    .placeholder(self.settings.copy_to_clipboard_str())
                    .width(Length::Units(200));
                let decimal_input_widget =
                    DecimalInputWidget::new(self.value, false, Message::InputU32Changed);
                let hex_input_widget =
                    DecimalInputWidget::new(self.value, true, Message::InputU32Changed);
                let octal_text_input = text_input(
                    "",
                    format!(
                        "{:02o} {:03o} {:03o} {:03o}",
                        (self.value >> 27) & 0o777,
                        (self.value >> 18) & 0o777,
                        (self.value >> 9) & 0o777,
                        self.value & 0o777
                    )
                    .as_str(),
                    Message::DecInputChanged,
                )
                .width(Length::Units(150));
                column![
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
                    binary_field_widget,
                    row![
                        column![
                            text(self.settings.hexadecimal_str()),
                            hexadecimal_text_input,
                            hex_pick_list,
                            hex_input_widget,
                        ]
                        .spacing(10),
                        vertical_rule(38),
                        column![
                            row![text(self.settings.decimal_str()), signed_toogler].spacing(10),
                            decimal_text_input,
                            dec_pick_list,
                            decimal_input_widget,
                        ]
                        .spacing(10),
                        vertical_rule(38),
                        column![text(self.settings.octal_str()), octal_text_input].spacing(10),
                    ]
                    .spacing(20),
                ]
                .spacing(20)
                .max_width(800)
                .into()
            }
            Pages::Settings => self.settings.view().map(Message::SettingsMessage),
        };
        let content2 = column![header_row, content].padding(20).spacing(20);
        container(content2)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            //                    .center_y()
            .into()
    }

    fn theme(&self) -> Theme {
        self.settings.theme().clone()
    }
}

impl BinaryCalculator {
    fn copy_hex_to_clipboard(&self, format: HexFormats) {
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
}
