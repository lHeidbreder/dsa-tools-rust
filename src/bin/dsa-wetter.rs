use clap::{Parser, ValueEnum, builder::PossibleValue};
use dsa_tools_rust::Format;
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::Serialize;

#[derive(Parser)]
struct Cli {
    #[arg(short = 'v', long = "verbose", default_value_t = false, 
        help = "Spuckt unnötig viel Holz aus")]
    verbose: bool,
    #[arg(short = 'f', long = "format", default_value_t = Format::TEXT, ignore_case = true, 
        help = "Ausgabeformat: Freitext, md, json oder csv. Standard ist Freitext.", hide_possible_values = true, hide_default_value = true)]
    format: Format,
    #[arg(short = 'o', long = "output", default_value = None,
        help = "Der Speicherort für die Ausgabe. Standard ist stdout.")]
    outfile: Option<std::path::PathBuf>,
    #[arg(short = 'x', long = "seed", default_value = None,
        help = "Setze den Seed manuell.", hide_default_value = true)]
    seed: Option<i64>,
    #[arg(short = 'd', long = "desert", default_value_t = false,
        help = "Die Gruppe befindet sich in der Wüste.")]
    is_desert: bool,
    #[arg(short = 's', long = "season", default_value_t = Season::SUMMER, ignore_case = true,
        help = "Die Jahreszeit. Standard ist Sommer.", hide_default_value = true)]
    season: Season,
    #[arg(short = 'w', long = "windy", default_value_t = false,
        help = "Es ist besonders windig.")]
    is_windy: bool,
    #[arg(short = 'r', long = "region", default_value_t = Region::MITTELREICH, ignore_case = true, 
        help = "Die Region wie angegeben auf S. 157 WdE. Standard ist Zentrales Mittelreich.", hide_possible_values = true, hide_default_value = true)]
    region: Region,
    #[arg(short = 'n', long = "days", default_value_t = 1,
        help = "Die Menge an Tagen, die generiert werden soll. Standard ist 1.", hide_default_value = true)]
    days: u64
}
impl std::fmt::Display for Cli {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let binding = std::path::PathBuf::from("");
        let file = match &self.outfile {
            Some(f) => f,
            None => &binding,
        };
        write!(f, "Output: {}; Format: {}", file.display(), self.format)
    }
}
fn log(args: &Cli, msg: &impl std::fmt::Display) {
    if args.verbose {
        println!("{}", msg)
    }
}

#[derive(Clone, PartialEq)]
enum Season {SUMMER,AUTUMN,WINTER,SPRING}
impl ValueEnum for Season {
    fn value_variants<'a>() -> &'a [Self] {
        &[Season::SUMMER,Season::AUTUMN,Season::WINTER,Season::SPRING]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(PossibleValue::new(format!("{}", self)))
    }
}
impl std::fmt::Display for Season {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Season::SUMMER => write!(f, "Sommer"),
            Season::AUTUMN => write!(f, "Herbst"),
            Season::WINTER => write!(f, "Winter"),
            Season::SPRING => write!(f, "Frühling"),
        }
    }
}
#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
enum Region {
    EWIGES_EIS,
    EHERNES_SCHWERT,
    HOHER_NORDEN,
    TUNDRA,
    THORWAL,
    WEIDEN,
    MITTELREICH,
    ALMADA,
    RASCHTULSWALL,
    HORASREICH_SUED,
    KHOM,
    ECHSENSUEMPFE,
    SUEDMEER
}
impl Region {
    fn temp_base (&self, season: &Season) -> i32 {
        let tuple = match self {
            Region::EWIGES_EIS => (-20,-30,-40),
            Region::EHERNES_SCHWERT => (-10,-20,-30),
            Region::HOHER_NORDEN => (0,-10,-20),
            Region::TUNDRA => (5,0,-5),
            Region::THORWAL => (10,3,-5),
            Region::WEIDEN => (10,5,0),
            Region::MITTELREICH => (15,10,5),
            Region::ALMADA => (20,15,10),
            Region::RASCHTULSWALL => (5,0,-10),
            Region::HORASREICH_SUED => (25,20,15),
            Region::KHOM => (40,35,30),
            Region::ECHSENSUEMPFE => (30,25,20),
            Region::SUEDMEER => (35,30,25),
        };
        match season {
            Season::SUMMER => tuple.0,
            Season::WINTER => tuple.2,
            Season::SPRING | Season::AUTUMN => tuple.1,
        }
    }
}
impl ValueEnum for Region {
    fn value_variants<'a>() -> &'a [Self] {
        &[Region::EWIGES_EIS, Region::EHERNES_SCHWERT, Region::HOHER_NORDEN, 
            Region::TUNDRA, Region::THORWAL, Region::WEIDEN, 
            Region::MITTELREICH, Region::ALMADA, Region::RASCHTULSWALL,
            Region::HORASREICH_SUED, Region::KHOM, Region::ECHSENSUEMPFE,
            Region::SUEDMEER]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(PossibleValue::new(format!("{}", self)))
    }
}
impl std::fmt::Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Region::EWIGES_EIS => write!(f, "Ewiges Eis"),
            Region::EHERNES_SCHWERT => write!(f, "Höhen des Ehernen Schwerts"),
            Region::HOHER_NORDEN => write!(f, "Hoher Norden"),
            Region::TUNDRA => write!(f, "Tundra und Taiga"),
            Region::THORWAL => write!(f, "Bornland, Thorwal"),
            Region::WEIDEN => write!(f, "Streitende Königreiche bis Weiden"),
            Region::MITTELREICH => write!(f, "Zentrales Mittelreich"),
            Region::ALMADA => write!(f, "Nördliches Horasreich, Almada, Aranien"),
            Region::RASCHTULSWALL => write!(f, "Höhen des Raschtulswalls"),
            Region::HORASREICH_SUED => write!(f, "Südliches Horasreich, Reich der ersten Sonne"),
            Region::KHOM => write!(f, "Khom"),
            Region::ECHSENSUEMPFE => write!(f, "Echsensümpfe, Meridiana"),
            Region::SUEDMEER => write!(f, "Altoum, Gewürzinseln, Südmeer"),
        }
    }
}

#[derive(Clone, Copy, Serialize)]
enum Clouds {NONE,FEW,LOTS,ALL}
impl Clouds {
    fn temp_mod (&self) -> i32 {
        match self {
            Clouds::NONE => 10,
            Clouds::FEW => 5,
            Clouds::LOTS => 0,
            Clouds::ALL => -5,
        }
    }
}
impl std::fmt::Display for Clouds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Clouds::NONE => write!(f, "völlig wolkenlos"),
            Clouds::FEW => write!(f, "einzelne Wolken"),
            Clouds::LOTS => write!(f, "bewölkt mit Wolkenlücken"),
            Clouds::ALL => write!(f, "geschlossene Wolkendecke"),
        }
    }
}
#[derive(Clone, Copy, Serialize)]
enum Wind {
    NONE,
    LIGHT,
    SOFT,
    FRESH,
    COOL,
    STRONG,
    STORM
}
impl Wind {
    fn temp_mod (&self) -> i32 {
        match self {
            Wind::NONE => 4,
            Wind::LIGHT => 2,
            Wind::SOFT | Wind::FRESH => 0,
            Wind::COOL => -2,
            Wind::STRONG => -4,
            Wind::STORM => -6,
        }
    }
}
impl std::fmt::Display for Wind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Wind::NONE => write!(f, "windstill"),
            Wind::LIGHT => write!(f, "leichter Wind"),
            Wind::SOFT => write!(f, "sanfte Brise"),
            Wind::FRESH => write!(f, "frische Brise"),
            Wind::COOL => write!(f, "steife Brise"),
            Wind::STRONG => write!(f, "starker Wind"),
            Wind::STORM => write!(f, "Sturm"),
        }
    }
}
#[derive(Clone, Copy, Serialize)]
enum Rain {
    NONE,
    LITTLE,
    LOTS,
    ALL
}
#[derive(Serialize)]
struct Day {
    no: u64,
    clouds: Clouds,
    wind: Wind,
    day_temp: i32,
    night_temp: i32,
    rain: Rain
}
impl Day {
    fn md(&self) -> String {
        format!("- Tag {}: {}, {}, {} - {}", self.no, self.clouds, self.wind, self.day_temp, self.night_temp)
    }
    fn csv(&self) -> String {
        format!("Tag {}, {}, {}, {}, {}", self.no, self.clouds, self.wind, self.day_temp, self.night_temp)
    }
}
impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tag {}\n{} {}\nTemperatur von {} bis {}", self.no, self.clouds, self.wind, self.day_temp, self.night_temp)
    }
}

#[derive(Clone, Copy)]
enum ChangesFlags {
    NONE = 0,
    CLOUDS = 0b0001,
    WIND = 0b0010,
    TEMPERATURE = 0b0100,
    RAIN = 0b1000,
    ALL = 0b1111
}

fn step1(args: &Cli, rng: &mut StdRng) -> Clouds {
    let roll = rng.gen_range(1..=20);
    if args.is_desert {
        match roll {
            1..=16 => Clouds::NONE,
            17..=18 => Clouds::FEW,
            19 => Clouds::LOTS,
            20 => Clouds::ALL,
            _ => panic!()
        };
    }
    match roll {
        1..=4 => Clouds::NONE,
        5..=10 => Clouds::FEW,
        11..=16 => Clouds::LOTS,
        17..=20 => Clouds::ALL,
        _ => panic!()
    }
}
fn step2(args: &Cli, rng: &mut StdRng) -> Wind {
    let roll = if args.is_windy {rng.gen_range(1..=20) + 2} else {rng.gen_range(1..=20)};
    
    if args.season == Season::AUTUMN {
        match roll {
            1..=3 => Wind::NONE,
            4..=5 => Wind::LIGHT,
            6..=7 => Wind::SOFT,
            8..=10 => Wind::FRESH,
            11..=14 => Wind::COOL,
            15..=18 => Wind::STRONG,
            19..=22 => Wind::STORM,
            _ => panic!()
        }
    } else {
        match roll {
            1..=4 => Wind::NONE,
            5..=7 => Wind::LIGHT,
            8..=10 => Wind::SOFT,
            11..=13 => Wind::FRESH,
            14..=16 => Wind::COOL,
            17..=19 => Wind::STRONG,
            20..=22 => Wind::STORM,
            _ => panic!()
        }
    }
}
fn step3(args: &Cli, rng: &mut StdRng, clouds_mod: i32, wind_mod: i32) -> (i32, i32) {
    let roll = rng.gen_range(1..=20)+5;
    (
        args.region.temp_base(&args.season) + wind_mod + clouds_mod,
        args.region.temp_base(&args.season) + wind_mod - clouds_mod - roll
    )
}
fn step4(rng: &mut StdRng, clouds: &Clouds, wind: &Wind) -> Rain {
    let roll = rng.gen_range(1..=20);
    let does_rain: bool = match clouds {
        Clouds::NONE => false,
        Clouds::FEW => roll == 1,
        Clouds::LOTS => roll <= 4,
        Clouds::ALL => roll <= 10,
    };

    if does_rain {
        let roll = rng.gen_range(1..=20);
        return match wind {
            Wind::NONE => match roll {
                1..=12 => Rain::LITTLE,
                13..=19 => Rain::LOTS,
                20 => Rain::ALL,
                _ => panic!()
            },
            Wind::LIGHT => match roll {
                1..=9 => Rain::LITTLE,
                10..=18 => Rain::LOTS,
                19..=20 => Rain::ALL,
                _ => panic!()
            },
            Wind::SOFT => match roll {
                1..=7 => Rain::LITTLE,
                8..=17 => Rain::LOTS,
                18..=20 => Rain::ALL,
                _ => panic!()
            },
            Wind::FRESH => match roll {
                1..=5 => Rain::LITTLE,
                6..=16 => Rain::LOTS,
                17..=20 => Rain::ALL,
                _ => panic!()
            },
            Wind::COOL => match roll {
                1..=3 => Rain::LITTLE,
                4..=15 => Rain::LOTS,
                16..=20 => Rain::ALL,
                _ => panic!()
            },
            Wind::STRONG => match roll {
                1..=2 => Rain::LITTLE,
                3..=13 => Rain::LOTS,
                14..=20 => Rain::ALL,
                _ => panic!()
            },
            Wind::STORM => match roll {
                1 => Rain::LITTLE,
                2..=10 => Rain::LOTS,
                11..=20 => Rain::ALL,
                _ => panic!()
            },
        }
    }
    Rain::NONE
}
fn step6(args: &Cli, rng: &mut StdRng) -> usize {
    let roll = rng.gen_range(1..=20);
    if [Season::SUMMER,Season::WINTER].contains(&args.season) {
        match roll {
            1..=9 => ChangesFlags::NONE as usize,
            10 => ChangesFlags::WIND as usize,
            11 => ChangesFlags::TEMPERATURE as usize,
            12 => ChangesFlags::RAIN as usize,
            13 => ChangesFlags::CLOUDS as usize|ChangesFlags::RAIN as usize,
            14 => ChangesFlags::WIND as usize|ChangesFlags::TEMPERATURE as usize,
            15 => ChangesFlags::WIND as usize|ChangesFlags::TEMPERATURE as usize,
            16 => ChangesFlags::RAIN as usize|ChangesFlags::TEMPERATURE as usize,
            17 => ChangesFlags::ALL as usize^ChangesFlags::CLOUDS as usize,
            18 => ChangesFlags::ALL as usize^ChangesFlags::WIND as usize,
            19 => ChangesFlags::ALL as usize^ChangesFlags::TEMPERATURE as usize,
            20 => ChangesFlags::ALL as usize,
            _ => panic!()
        }
    }
    else {
        match roll {
            1..=4 => ChangesFlags::NONE as usize,
            5 => ChangesFlags::WIND as usize,
            6 => ChangesFlags::TEMPERATURE as usize,
            7 => ChangesFlags::RAIN as usize,
            8..=9 => ChangesFlags::CLOUDS as usize|ChangesFlags::RAIN as usize,
            10..=11 => ChangesFlags::WIND as usize|ChangesFlags::TEMPERATURE as usize,
            12..=13 => ChangesFlags::WIND as usize|ChangesFlags::TEMPERATURE as usize,
            14..=15 => ChangesFlags::RAIN as usize|ChangesFlags::TEMPERATURE as usize,
            16 => ChangesFlags::ALL as usize^ChangesFlags::CLOUDS as usize,
            17 => ChangesFlags::ALL as usize^ChangesFlags::WIND as usize,
            18 => ChangesFlags::ALL as usize^ChangesFlags::TEMPERATURE as usize,
            19..=20 => ChangesFlags::ALL as usize,
            _ => panic!()
        }
    }
}

fn main () {
    let args = Cli::parse();
    let s = match args.seed {
        Some(s) => s as u64,
        None => rand::thread_rng().gen(),
    };
    let mut rng = rand::rngs::StdRng::seed_from_u64(s as u64);
    log(&args, &args);
    log(&args, &format!("Seed {}", s));

    let mut days: Vec<Day> = Vec::new();
    let mut step6_flags = 0b1111; // 0001 - Clouds, 0010 - Wind, 0100 - Temperature, 1000 - Rain
    for i in 0..args.days {
        let res1 = if step6_flags&0001>0 {step1(&args, &mut rng)} else { match days.last() {
            Some(d) => d.clouds,
            None => panic!(),
        } };
        let res2 = if step6_flags&0010>0 {step2(&args, &mut rng)} else { match days.last() {
            Some(d) => d.wind,
            None => panic!(),
        } };
        let res3 = if step6_flags&0100>0 {step3(&args, &mut rng, res1.temp_mod(), res2.temp_mod())} else { match days.last() {
            Some(d) => (d.day_temp, d.night_temp),
            None => panic!(),
        } };
        let res4 = if step6_flags&1000>0 {step4(&mut rng, &res1, &res2)} else { match days.last() {
            Some(d) => d.rain,
            None => panic!(),
        } };

        days.push(Day { no: i+1, clouds: res1, wind: res2, day_temp: res3.0, night_temp: res3.1, rain: res4 });

        step6_flags = step6(&args, &mut rng);
    }

    let mut file: Box<dyn std::io::Write> = match args.outfile {
        Some(f) => match std::fs::File::create(f) {
            Ok(fi) => Box::new(fi),
            Err(_) => Box::new(std::io::stdout()),
        },
        None => Box::new(std::io::stdout()),
    };
    let _ = match args.format {
        Format::TEXT => for d in days {match writeln!(file, "{}", d) {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        };},
        Format::CSV => for d in days {match writeln!(file, "{}", d.csv()) {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        };},
        Format::MD => for d in days {match writeln!(file, "{}", d.md()) {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        };},
        Format::JSON => {match writeln!(file, "{}", match serde_json::to_string(&days) {
            Ok(json) => json,
            Err(e) => panic!("{}", e)
        }) {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        };},
    };
    ()
}