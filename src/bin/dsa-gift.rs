use std::fs::File;
use serde::Serialize;
use clap::Parser;
use dsa_tools_rust::*;
use rand::Rng;

#[derive(Parser)]
struct Cli {
    #[arg(short = 'v', long = "verbose", default_value_t = false)]
    verbose: bool,
    #[arg(short = 'o', long = "output", default_value = None)]
    outfile: Option<std::path::PathBuf>,
    #[arg(short = 'f', long = "format", default_value_t = Format::TEXT, ignore_case = true)]
    format: Format,
    #[arg(short = 'x', long = "seed", default_value_t = 0)]
    seed: i64,
    #[arg(short = 'l', long = "level", default_value_t = 1)]
    level: u32,
}
impl std::fmt::Display for Cli {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let binding = std::path::PathBuf::from("");
        let file = match &self.outfile {
            Some(f) => f,
            None => &binding,
        };
        write!(f, "Seed: {}; Level: {}, Output: {}, Format: {}", self.seed, self.level, file.display(), self.format)
    }
}

fn log(args: &Cli, msg: &impl std::fmt::Display) {
    if args.verbose {
        println!("{}", msg)
    }
}

#[derive(Default, Serialize)]
struct Symptom {
    amount: u32,
    name: Box<str>,
    characteristic: Option<Characteristic>,
    disadvantage: Option<String>,
    unconsciousness: bool
}
impl std::fmt::Display for Symptom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;
        match &self.characteristic {
            Some(c) => write!(f, " ({} -{}W6)", c, self.amount)?,
            None => (),
        };
        match &self.disadvantage {
            Some(d) => write!(f, " ({} +{}W6)", d, self.amount)?,
            None => (),
        };
        if self.unconsciousness {write!(f, " (bewusstlos)")?;}
        Ok(())
    }
}
fn roll_symptom (rng: &mut rand::rngs::ThreadRng) -> Symptom {
    let roll:u32 = rng.gen_range(1..21);

    match roll {
        1..=4 => Symptom{name: "Erbrechen".into(), characteristic: Some(Characteristic::CH), ..Default::default()},
        5..=6 => Symptom{name: "Durchfall / Koliken".into(), characteristic: Some(Characteristic::MU), ..Default::default()},
        7..=8 => Symptom{name: "Schweißausbrüche / Atemnot".into(), characteristic: Some(Characteristic::KO), ..Default::default()},
        9..=10 => Symptom{name: "Schwäche".into(), characteristic: Some(Characteristic::KK), ..Default::default()},
        11..=12 => Symptom{name: "Kopfschmerz / Schwindel".into(), characteristic: Some(Characteristic::KL), ..Default::default()},
        13..=14 => Symptom{name: "Lähmungen".into(), characteristic: Some(Characteristic::GE), ..Default::default()},
        15..=16 => Symptom{name: "Taubheiten".into(), characteristic: Some(Characteristic::FF), ..Default::default()},
        17 => Symptom{name: "Schwellungen".into(), characteristic: Some(Characteristic::GE), ..Default::default()},
        18 => Symptom{name: "Erregung".into(), disadvantage: Some("Jähzorn".to_string()), ..Default::default()},
        19 => Symptom{name: "Blutungen".into(), disadvantage: Some("Aberglaube".to_string()), ..Default::default()},
        _ => Symptom { name: "Bewusstlosigkeit".into(), unconsciousness: true, ..Default::default()}
    }
}

#[derive(Serialize)]
struct SymptomList {
    symptoms: Vec<Symptom>
}
impl SymptomList {
    fn push(&mut self, x: Symptom) {
        for s in &mut self.symptoms {
            if s.name == x.name {s.amount += 1;}
            return
        }
        self.symptoms.push(x)
    }
}
impl std::fmt::Display for SymptomList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for s in 0..self.symptoms.len() {
            write!(f, "{}", self.symptoms[s])?;
            if s < (self.symptoms.len()-1) {write!(f, ", ")?;}
        }
        write!(f, "]")
    }
}

#[derive(Serialize)]
struct Poison {
    level: u32,
    start: DiceOverTime,
    damage: DiceOverTime,
    duration: DiceOverTime,
    symptoms: SymptomList
}
impl Poison {
    fn md(&self) -> String {
        format!("- Stufe {}\n- Beginn nach {}\n- Dauer {}\n- Schaden {} pro {}\n- {}", self.level, self.start, self.duration, self.damage.roll_only(), self.damage.time, self.symptoms)
    }
    fn csv(&self) -> String{
        format!("\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"", self.level, self.start, self.damage, self.duration, self.symptoms)
    }
    fn json(&self) -> String {
        match serde_json::to_string(self) {
            Ok(json) => json,
            Err(e) => panic!("{}", e)
        }
    }
}
impl std::fmt::Display for Poison {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Stufe {}\nBeginn nach {}\nDauer {}\nSchaden {} pro {}\n{}", self.level, self.start, self.duration, self.damage.roll_only(), self.damage.time, self.symptoms)
    }
}

/// Zufallsgenerator für Gift beliebiger Stufe
fn main() {
    let args = Cli::parse();
    let mut rng = rand::thread_rng();
    log(&args, &args);

    if args.level < 1 || args.level > 20 {
        panic!("Stufe war: {}\nMuss zwischen 1 und 20 liegen.", args.level);
    }

    let dice = (args.level as f64/ 4.0).ceil() as u32;
    log(&args, &format!("Schadenswürfel: {}", dice));

    let start: DiceOverTime;
    let damage: DiceOverTime;
    let duration: DiceOverTime;
    match args.level {
        1..=5 => {
            start = DiceOverTime{dice: 1, ..Default::default()};
            damage = DiceOverTime{flat: 1, time: Timeunit::STD, ..Default::default()};
            duration = DiceOverTime{flat: dice, ..Default::default()};
        },
        6..=9 => {
            start = DiceOverTime{flat: 1, ..Default::default()};
            damage = DiceOverTime{dice: 1, ..Default::default()};
            duration = DiceOverTime{flat: dice, ..Default::default()};
        },
        10..=15 => {
            start = DiceOverTime{dice: 1, flat: 4, time: Timeunit::KR};
            damage = DiceOverTime{dice: 2, ..Default::default()};
            duration = DiceOverTime{flat: dice/2, ..Default::default()};
        },
        16..=20 => {
            start = DiceOverTime{dice: 1, time: Timeunit::KR, ..Default::default()};
            damage = DiceOverTime{dice: 1, time: Timeunit::KR, ..Default::default()};
            duration = DiceOverTime{flat: dice, time: Timeunit::KR, ..Default::default()};
        },
        _ => panic!()
    }
    
    log(&args, &start);
    log(&args, &damage);
    log(&args, &duration);

    let mut symptoms:SymptomList = SymptomList { symptoms: Vec::new() };
    for _ in 0..(args.level as f64/ 2.0).ceil() as u32 {
        symptoms.push(roll_symptom(&mut rng))
    }

    let p = Poison{level: args.level, start: start, damage: damage, duration: duration, symptoms: symptoms };
    let mut file: Box<dyn std::io::Write> = match args.outfile {
        Some(f) => match File::create(f) {
            Ok(fi) => Box::new(fi),
            Err(_) => Box::new(std::io::stdout()),
        },
        None => Box::new(std::io::stdout()),
    };
    let _ = match args.format {
        Format::TEXT => write!(file, "{}", p),
        Format::CSV => write!(file, "{}", p.csv()),
        Format::MD => write!(file, "{}", p.md()),
        Format::JSON => write!(file, "{}", p.json())
    };
}
