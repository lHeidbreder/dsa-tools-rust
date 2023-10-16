pub enum Timeunit {
    KR, SR, STD
}
impl std::fmt::Display for Timeunit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Timeunit::KR => write!(f, "KR"),
            Timeunit::SR => write!(f, "SR"),
            Timeunit::STD => write!(f, "Stunde(n)")
        }
    }
}

#[derive(clap::ValueEnum)]
pub enum Format {
    TEXT, MD, CSV
}
impl Clone for Format {
    fn clone(&self) -> Self {
        match self {
            Format::MD => Format::MD,
            Format::CSV => Format::CSV,
            _ => Format::TEXT
        }
    }
}
impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Format::MD => write!(f, "MD"),
            Format::CSV => write!(f, "CSV"),
            Format::TEXT => write!(f, "TEXT")
        }
    }
}

pub struct DiceOverTime {
    pub dice: u32,
    pub flat: u32,
    pub time: Timeunit
}
impl std::fmt::Display for DiceOverTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}W6+{} {}", self.dice, self.flat, self.time)
    }
}
