#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum ThemeType {
    Light,
    Dark,
    Custom,
}

impl ThemeType {
    pub const ALL: [Self; 3] = [Self::Light, Self::Dark, Self::Custom];
}
