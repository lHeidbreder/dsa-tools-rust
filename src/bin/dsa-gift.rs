use clap::Parser;
use dsa_tools_rust::*;

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

/// Zufallsgenerator für Gift beliebiger Stufe
fn main() {
    let args = Cli::parse();
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
        start = DiceOverTime{dice: 1, flat: 0, time: Timeunit::SR};
        damage = DiceOverTime{dice: 0, flat: 1, time: Timeunit::STD};
        duration = DiceOverTime{dice: 0, flat: dice, time: Timeunit::SR};
    }
    else if 6 <= args.level && args.level <= 9 {
        start = DiceOverTime{dice: 0, flat: 1, time: Timeunit::SR}
        damage = DiceOverTime{dice: 1, flat: 0, time: Timeunit::SR}
        duration = DiceOverTime{dice: 0, flat: dice, time: Timeunit::SR}
    }
    else if 10 <= args.level && args.level <= 15 {
        start = DiceOverTime{}
    }
    else {
        panic!("Hier solltest du nicht ankommen")
    }

    log(&args, &start);
    log(&args, &damage);
    log(&args, &duration);
}
