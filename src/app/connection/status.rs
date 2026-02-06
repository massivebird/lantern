use super::Address;
use ratatui::style::Color;

type Code = Result<u16, String>;
type Timestamp = chrono::DateTime<chrono::Local>;

#[derive(Clone, Debug)]
pub struct Status {
    code: Code,
    time: Timestamp,
}

impl Status {
    pub fn new(code: Code) -> Self {
        Self {
            code,
            time: chrono::Local::now(),
        }
    }

    pub const fn code(&self) -> &Code {
        &self.code
    }

    pub const fn timestamp(&self) -> Timestamp {
        self.time
    }

    pub const fn generate_color(&self, addr: &Address) -> Color {
        let Ok(code) = self.code() else {
            return Color::Red;
        };

        match addr {
            Address::Remote { .. } | Address::Json { .. } => match code {
                200 => Color::Green,
                400.. => Color::Red,
                _ => Color::Yellow,
            },

            Address::Local { .. } => Color::Green,
        }
    }
}

impl From<Code> for Status {
    fn from(value: Code) -> Self {
        Self::new(value)
    }
}
