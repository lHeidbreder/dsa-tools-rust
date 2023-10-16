use std::{fs::File, fmt::write};

use clap::Parser;
use dsa_tools_rust::*;
use rand::Rng;

#[derive(Parser)]
struct Cli {
    #[arg(short = 'v', long = "verbose", default_value_t = false)]
    verbose: bool,
    #[arg(short = 'o', long = "output", default_value = "/dev/null")]
    outfile: std::path::PathBuf,
    #[arg(short = 'f', long = "format", default_value_t = Format::TEXT, ignore_case = true)]
    format: Format,
    #[arg(short = 'x', long = "seed", default_value_t = 0)]
    seed: i64,
    #[arg(short = 'l', long = "level", default_value_t = 1)]
    level: u32,
}
impl std::fmt::Display for Cli {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Seed: {}; Level: {}, Output: {}, Format: {}", self.seed, self.level, self.outfile.display(), self.format)
    }
}

fn log(args: &Cli, msg: &impl std::fmt::Display) {
    if args.verbose {
        println!("{}", msg)
    }
}

#[derive(Default)]
struct Symptom {
    name: Box<str>,
    characteristic: Option<Characteristic>,
    disadvantage: Option<String>,
    unconsciousness: bool
}
impl std::fmt::Display for Symptom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
fn display_symptoms(symptoms: &Vec<Symptom>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "[");
    for s in 0..symptoms.len() {
        write!(f, "{}", symptoms[s]);
        if s < (symptoms.len()-1) {write!(f, ", ");}
    }
    write!(f, "]")
}
fn roll_symptom (rng: &mut rand::rngs::ThreadRng) -> Symptom {
    let roll:u32 = rng.gen_range(1..21);

    if roll <= 4 {
        return Symptom{name: "Erbrechen".into(), characteristic: Some(Characteristic::CH), ..Default::default()};
    }
    if roll <= 6 {
        return Symptom{name: "Durchfall / Koliken".into(), characteristic: Some(Characteristic::MU), ..Default::default()};
    }
    if roll <= 8 {
        return Symptom{name: "Schweißausbrüche / Atemnot".into(), characteristic: Some(Characteristic::KO), ..Default::default()};
    }
    if roll <= 10 {
        return Symptom{name: "Schwäche".into(), characteristic: Some(Characteristic::KK), ..Default::default()};
    }
    if roll <= 12 {
        return Symptom{name: "Kopfschmerz / Schwindel".into(), characteristic: Some(Characteristic::KL), ..Default::default()};
    }
    if roll <= 14 {
        return Symptom{name: "Lähmungen".into(), characteristic: Some(Characteristic::GE), ..Default::default()};
    }
    if roll <= 16 {
        return Symptom{name: "Taubheiten".into(), characteristic: Some(Characteristic::FF), ..Default::default()};
    }
    if roll <= 17 {
        return Symptom{name: "Schwellungen".into(), characteristic: Some(Characteristic::GE), ..Default::default()};
    }
    if roll <= 18 {
        return Symptom{name: "Erregung".into(), disadvantage: Some("Jähzorn".to_string()), ..Default::default()};
    }
    if roll <= 19 {
        return Symptom{name: "Blutungen".into(), disadvantage: Some("Aberglaube".to_string()), ..Default::default()};
    }
    return Symptom { name: "Bewusstlosigkeit".into(), unconsciousness: true, ..Default::default()}
}

struct Poison {
    level: u32,
    start: DiceOverTime,
    damage: DiceOverTime,
    duration: DiceOverTime,
    symptoms: Vec<Symptom>
}
impl Poison {
    fn md(&self) -> String {
        todo!("Symptome auflisten");
        format!("- Stufe {}\n- Beginn nach {}\n- Dauer {}\n- Schaden {} pro {}", self.level, self.start, self.duration, self.damage.roll_only(), self.damage.time)
    }
    fn csv(&self) -> String{
        format!("\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"", self.level, self.start, self.damage, self.duration, display_symptoms(self.symptoms))
    }
}
impl std::fmt::Display for Poison {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!("Symptome auflisten");
        write!(f, "Stufe {}\nBeginn nach {}\nDauer {}\nSchaden {} pro {}", self.level, self.start, self.duration, self.damage.roll_only(), self.damage.time)
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
    if 1 <= args.level && args.level <= 5 {
        start = DiceOverTime{dice: 1, ..Default::default()};
        damage = DiceOverTime{flat: 1, time: Timeunit::STD, ..Default::default()};
        duration = DiceOverTime{flat: dice, ..Default::default()};
    }
    else if 6 <= args.level && args.level <= 9 {
        start = DiceOverTime{flat: 1, ..Default::default()};
        damage = DiceOverTime{dice: 1, ..Default::default()};
        duration = DiceOverTime{flat: dice, ..Default::default()};
    }
    else if 10 <= args.level && args.level <= 15 {
        start = DiceOverTime{dice: 1, flat: 4, time: Timeunit::KR};
        damage = DiceOverTime{dice: 2, ..Default::default()};
        duration = DiceOverTime{flat: dice/2, ..Default::default()};
    }
    else if 16 <= args.level && args.level <= 20 {
        start = DiceOverTime{dice: 1, time: Timeunit::KR, ..Default::default()};
        damage = DiceOverTime{dice: 1, time: Timeunit::KR, ..Default::default()};
        duration = DiceOverTime{flat: dice, time: Timeunit::KR, ..Default::default()};
    }
    else {
        panic!("Hier solltest du nicht ankommen")
    }

    log(&args, &start);
    log(&args, &damage);
    log(&args, &duration);

    let mut symptoms:Vec<Symptom> = Vec::new();
    for _ in 0..(args.level as f64/ 2.0).ceil() as u32 {
        symptoms.push(roll_symptom(&mut rng))
    }

    let p = Poison{level: args.level, start: start, damage: damage, duration: duration, symptoms: symptoms};
    let mut file: Box<dyn std::io::Write> = match File::create(args.outfile) {
        Ok(f) => Box::new(f),
        Err(e) => Box::new(std::io::stdout())
    };
    match args.format {
        Format::TEXT => write!(file, "{}", p),
        Format::CSV => write!(file, "{}", p.csv()),
        Format::MD => write!(file, "{}", p.md()),
    };
}
