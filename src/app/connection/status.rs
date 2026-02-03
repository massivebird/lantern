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
}

impl From<Code> for Status {
    fn from(value: Code) -> Self {
        Self::new(value)
    }
}
