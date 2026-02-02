use clap::{ValueEnum, builder::PossibleValue};
use strum::EnumIter;
use strum::IntoEnumIterator;

#[derive(Default, Copy, Clone, EnumIter, PartialEq, Eq)]
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

impl OutputFmt {
    /// Returns the "next" output format, in cyclic fashion.
    ///
    /// Is using a cycling `strum` iterator overkill for this? Probably.
    /// Whatever. It's scalable, yo.
    pub fn next(self) -> Self {
        let mut iter = Self::iter().cycle();

        iter.find(|f| *f == self);

        iter.next().unwrap()
    }
}
