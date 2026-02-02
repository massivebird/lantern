use clap::{ValueEnum, builder::PossibleValue};

#[derive(Default, Copy, Clone)]
pub enum OutputFmt {
    #[default]
    Line,
    Bullet,
}

// Define CLI controls.
impl ValueEnum for OutputFmt {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Bullet, Self::Line]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            Self::Bullet => Some(PossibleValue::new("bullet")),
            Self::Line => Some(PossibleValue::new("line")),
        }
    }
}
