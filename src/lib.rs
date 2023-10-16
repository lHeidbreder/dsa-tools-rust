use clap::{ValueEnum, builder::PossibleValue};

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
impl Default for Timeunit {
    fn default() -> Self {
        Timeunit::SR
    }
}

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
impl ValueEnum for Format {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::TEXT, Self::CSV, Self::MD]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Self::TEXT => PossibleValue::new("text"),
            Self::CSV => PossibleValue::new("csv"),
            Self::MD => PossibleValue::new("md")
        })
    }
}

#[derive(Default)]
pub struct DiceOverTime {
    pub dice: u32,
    pub flat: u32,
    pub time: Timeunit
}
impl DiceOverTime {
    pub fn roll_only(&self) -> String {
        format!("{}W6+{}", self.dice, self.flat)
    }
}
impl std::fmt::Display for DiceOverTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}W6+{} {}", self.dice, self.flat, self.time)
    }
}

pub enum Characteristic {
    MU, KL, IN, CH, FF, GE, KO, KK
}