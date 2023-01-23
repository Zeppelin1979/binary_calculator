#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DecFormats {
    Plain,
    PointSeperator,
    CommaSeperator,
}

impl DecFormats {
    pub const ALL: [Self; 3] = [Self::Plain, Self::PointSeperator, Self::CommaSeperator];
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
