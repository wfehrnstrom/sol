use clap::{Arg, App, ArgGroup, SubCommand};

pub struct Lat{
    deg: u8,
    dir: Option<bool>
}

impl Lat {
    pub fn new(deg: u8, dir: Option<bool>) -> Self {
        Lat {deg, dir}
    }
}

fn dir_to_bool(dir: &str) -> Option<bool> {
    match dir.to_lowercase().as_ref() {
        "n" => Some(true),
        "s" => Some(false),
         _ => None
    }
}

impl From<String> for Lat {
    fn from(s: String) -> Self {
        let el: Vec<&str> = s.split(|c| c == '.' || c == ',' || c == '\'' || c == ' ').collect();
        match el.as_slice() {
            [deg, dir] => Lat::new(deg.parse::<u8>().unwrap(), dir_to_bool(dir)),
            [deg] => Lat::new(deg.parse::<u8>().unwrap(), None),
            _ => Lat::new(0, None)
        }
    }
}

fn print_orientation(d: Option<bool>){
    match d {
        Some(dir) => {
            let dirstring = if dir {"south"} else {"north"};
            println!("Orientation: {}", dirstring)
        },
        None => {
            println!("Direction not given. In the Northern Hemisphere, point your solar panel
            true south. In the Southern Hemisphere, point your solar panel true north.");
        }
    }
}

fn naive_calculate(l: Lat){
    let tilt: f32 = if l.deg < 25 {
        f32::from(l.deg)*0.87
    }
    else if l.deg >= 25 && l.deg <= 50 {
        f32::from(l.deg)*0.76+3.1
    }
    else {
        // No-Op Currently
        f32::from(l.deg)
    };
    print_orientation(l.dir);
    println!("Inclination: {}°\n", tilt);
}

fn season_calculate(l: Lat){

}

const FIXED: &str = "fixed";
const ADJUSTABLE: &str = "adjustable";

fn main() {
    let matches = App::new("Sol")
                    .version("1.0.0")
                    .author("Will Fehrnstrom, wfehrnstrom@gmail.com")
                    .about("Utility for determining optimal positioning of solar panels")
                    .subcommand(SubCommand::with_name("placement")
                            .about("Determine what the optimal placement for solar panels is.")
                            .version("1.0.0")
                            .author("Will Fehrnstrom, wfehrnstrom@gmail.com")
                            .arg(Arg::with_name(FIXED)
                                .short("f")
                                .long(FIXED)
                                .value_name("LAT")
                                .help("Calculate optimal positioning for completely fixed solar.\n\
                                LAT takes the regex form '([0-9]{1-2}[' .,]){1-3} [NSns]'")
                                .takes_value(true)
                            )
                            .arg(Arg::with_name(ADJUSTABLE)
                                .short("a")
                                .long(ADJUSTABLE)
                                .value_name("LAT")
                                .help("Calculate optimal positioning for seasonally adjusted solar.\n\
                                LAT takes the regex form '([0-9]{1-2}[' .,]){1-3} [NSns]'")
                            )
                            .group(ArgGroup::with_name("placement_types")
                                .args(&[FIXED, ADJUSTABLE])
                                .required(true)
                            )
                    )
                    .get_matches();
    if let Some(matches) = matches.subcommand_matches("placement"){
        if matches.is_present(FIXED){
            let val = matches.value_of(FIXED).unwrap();
            let lat = Lat::from(String::from(val));
            naive_calculate(lat);
        }
        else if matches.is_present(ADJUSTABLE){
            let val = matches.value_of(ADJUSTABLE).unwrap();
            let lat = Lat::from(String::from(val));

        }
        else{
            println!("No other options implemented for placement");
        }
    }
    else{
        println!("No other options implemented");
    }
}
