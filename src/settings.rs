use crate::language_type::LanguageType;
use crate::theme_type::ThemeType;
use fluent_templates::loader::langid;
use fluent_templates::{static_loader, LanguageIdentifier, Loader};
use iced::widget::{column, horizontal_rule, radio, text};
use iced::{theme, Color, Element, Theme};

const GERMAN: LanguageIdentifier = langid!("de");
const ENGLISH: LanguageIdentifier = langid!("en");

static_loader! {
    static LOCALES = {
        locales: "locales",
        fallback_language: "en",
    };
}

pub(crate) struct BinaryCalulatorSettings {
    theme: Theme,
    language: LanguageType,
    setting_str: String,
    copy_to_clipboard_str: String,
    main_str: String,
    decimal_str: String,
    hexadecimal_str: String,
    octal_str: String,
    choose_a_theme_str: String,
    choose_a_language_str: String,
    light_str: String,
    dark_str: String,
    custom_str: String,
}

impl<'a> BinaryCalulatorSettings {
    pub fn new() -> Self {
        Self {
            theme: Theme::Dark,
            language: LanguageType::English,
            setting_str: LOCALES.lookup(&ENGLISH, "settings").unwrap(),
            copy_to_clipboard_str: LOCALES.lookup(&ENGLISH, "copy_to_clipboard").unwrap(),
            main_str: LOCALES.lookup(&ENGLISH, "main").unwrap(),
            decimal_str: LOCALES.lookup(&ENGLISH, "decimal").unwrap(),
            hexadecimal_str: LOCALES.lookup(&ENGLISH, "hexadecimal").unwrap(),
            octal_str: LOCALES.lookup(&ENGLISH, "octal").unwrap(),
            choose_a_theme_str: LOCALES.lookup(&ENGLISH, "choose_a_theme").unwrap(),
            choose_a_language_str: LOCALES.lookup(&ENGLISH, "choose_a_language").unwrap(),
            light_str: LOCALES.lookup(&ENGLISH, "light").unwrap(),
            dark_str: LOCALES.lookup(&ENGLISH, "dark").unwrap(),
            custom_str: LOCALES.lookup(&ENGLISH, "custom").unwrap(),
        }
    }

    pub fn update(&mut self, msg: SettingsMessage) {
        match msg {
            SettingsMessage::ThemeChanged(theme) => {
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
            SettingsMessage::Language(lang) => {
                self.language = lang;
                match lang {
                    LanguageType::English => self.read_language(&ENGLISH),
                    LanguageType::Deutsch => self.read_language(&GERMAN),
                }
            }
        };
    }

    pub fn view(&self) -> Element<'a, SettingsMessage> {
        let choose_theme = ThemeType::ALL.iter().fold(
            column![text(&self.choose_a_theme_str)].spacing(10),
            |column, theme| {
                column.push(radio(
                    match theme {
                        ThemeType::Light => &self.light_str,
                        ThemeType::Dark => &self.dark_str,
                        ThemeType::Custom => &self.custom_str,
                    },
                    *theme,
                    Some(match self.theme {
                        Theme::Light => ThemeType::Light,
                        Theme::Dark => ThemeType::Dark,
                        Theme::Custom { .. } => ThemeType::Custom,
                    }),
                    SettingsMessage::ThemeChanged,
                ))
            },
        );
        let choose_language = LanguageType::ALL.iter().fold(
            column![text(&self.choose_a_language_str)].spacing(10),
            |column, lang| {
                column.push(radio(
                    format!("{:?}", lang),
                    *lang,
                    Some(self.language),
                    SettingsMessage::Language,
                ))
            },
        );
        let content = column![horizontal_rule(38), choose_theme, choose_language].spacing(20);
        content.into()
    }

    fn read_language(&mut self, lang: &LanguageIdentifier) {
        self.setting_str = LOCALES.lookup(lang, "settings").unwrap();
        self.copy_to_clipboard_str = LOCALES.lookup(lang, "copy_to_clipboard").unwrap();
        self.main_str = LOCALES.lookup(lang, "main").unwrap();
        self.decimal_str = LOCALES.lookup(lang, "decimal").unwrap();
        self.hexadecimal_str = LOCALES.lookup(lang, "hexadecimal").unwrap();
        self.octal_str = LOCALES.lookup(lang, "octal").unwrap();
        self.choose_a_theme_str = LOCALES.lookup(lang, "choose_a_theme").unwrap();
        self.choose_a_language_str = LOCALES.lookup(lang, "choose_a_language").unwrap();
        self.light_str = LOCALES.lookup(lang, "light").unwrap();
        self.dark_str = LOCALES.lookup(lang, "dark").unwrap();
        self.custom_str = LOCALES.lookup(lang, "custom").unwrap();
    }

    pub(crate) fn theme(&self) -> &Theme {
        &self.theme
    }

    pub(crate) fn setting_str(&self) -> &str {
        self.setting_str.as_ref()
    }

    pub(crate) fn copy_to_clipboard_str(&self) -> &str {
        self.copy_to_clipboard_str.as_ref()
    }

    pub(crate) fn main_str(&self) -> &str {
        self.main_str.as_ref()
    }

    pub(crate) fn decimal_str(&self) -> &str {
        self.decimal_str.as_ref()
    }

    pub(crate) fn hexadecimal_str(&self) -> &str {
        self.hexadecimal_str.as_ref()
    }

    pub(crate) fn octal_str(&self) -> &str {
        self.octal_str.as_ref()
    }
}

#[derive(Debug, Clone)]
pub(crate) enum SettingsMessage {
    ThemeChanged(ThemeType),
    Language(LanguageType),
}
