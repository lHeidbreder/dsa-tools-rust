use clap::{ValueEnum, builder::PossibleValue};
use serde::Serialize;

#[derive(Serialize)]
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

#[derive(Serialize)]
pub enum Format {
    TEXT, MD, CSV, JSON
}
impl Clone for Format {
    fn clone(&self) -> Self {
        match self {
            Format::MD => Format::MD,
            Format::CSV => Format::CSV,
            Format::JSON => Format::JSON,
            _ => Format::TEXT
        }
    }
}
impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Format::MD => write!(f, "MD"),
            Format::CSV => write!(f, "CSV"),
            Format::TEXT => write!(f, "TEXT"),
            Format::JSON => write!(f, "JSON")
        }
    }
}
impl ValueEnum for Format {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::TEXT, Self::CSV, Self::MD, Self::JSON]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Format::TEXT => PossibleValue::new("text"),
            Format::CSV => PossibleValue::new("csv"),
            Format::MD => PossibleValue::new("md"),
            Format::JSON => PossibleValue::new("json"),
        })
    }
}

#[derive(Default, Serialize)]
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

#[derive(Serialize)]
pub enum Characteristic {
    MU, KL, IN, CH, FF, GE, KO, KK
}
impl std::fmt::Display for Characteristic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Characteristic::MU => write!(f, "MU"),
            Characteristic::KL => write!(f, "KL"),
            Characteristic::IN => write!(f, "IN"),
            Characteristic::CH => write!(f, "CH"),
            Characteristic::FF => write!(f, "FF"),
            Characteristic::GE => write!(f, "GE"),
            Characteristic::KO => write!(f, "KO"),
            Characteristic::KK => write!(f, "KK"),
        }
    }
}