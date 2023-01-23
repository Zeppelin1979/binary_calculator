#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum LanguageType {
    English,
    Deutsch,
}

impl LanguageType {
    pub const ALL: [Self; 2] = [Self::English, Self::Deutsch];
}
