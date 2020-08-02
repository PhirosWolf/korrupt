use clap::{Arg, App};

const CORRUPTION_METHODS: [&'static str; 10] = ["zeros", "ones", "flip", "swap", "reverse", "repeat", "shorten", "addnoise", "rcycle", "lcycle"];
/*
Quick explanation of these methods

Zeros: Replaces parts with 0s
Ones: Replaces parts with 1s
Flip: Flips random bits
Swap: Swaps two random parts
Reverse: Reverses parts (e.g. 10001010 becomes 01010001)
Repeat: Takes a part and repeats it (overwriting the parts following the origin)
Shorten: Removes random parts
AddNoise: Adds random noise
RCycle: Shifts bits to the right but adds the now out of bounds bits to the other side.
    e.g. xxxxx|1111010100|xxxxx, intensity = 4
         xxxxx|yyyy111101|0100x becomes
         xxxxx|0100111101|xxxxx (obviously, the previous step is just for the sake of the explanation, not the real behaviour of the code)
LCycle: Same as RCycle, but in the opposite direction
*/

fn main() {
    let matches = App::new("Korrupt")
        .version("test")
        .author("PhirosWolf")
        .about("A little corruption tool")
        .arg(Arg::with_name("corruption method")
            .short("m")
            .long("method")
            .help("Sets the corruption method to use")
            .takes_value(true)
            .possible_values(&CORRUPTION_METHODS)
            .required(true)
        )
        .arg(Arg::with_name("min part length")
            .long("minpartlen")
            .help("Minimum part length")
            .takes_value(true)
            .max_values(1)
            .required_unless("fixed part length")
            .requires("max part length")
            .empty_values(false)
        )
        .arg(Arg::with_name("max part length")
            .long("maxpartlen")
            .help("Maximum part length")
            .takes_value(true)
            .max_values(1)
            .required_unless("fixed part length")
            .requires("min part length")
            .empty_values(false)
        )
        .arg(Arg::with_name("fixed part length")
            .long("fixedpartlen")
            .help("Fixed part length")
            .takes_value(true)
            .max_values(1)
            .required_unless_all(&["min part length", "max part length"])
            .empty_values(false)
        )
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .help("Output file path")
            .takes_value(true)
            .default_value("/tmp/korrupt/@@filename@@")
            .empty_values(false)
            .value_name("PATH")
        )
        .arg(Arg::with_name("input")
            .help("Input file path")
            .required(true)
            .index(1)
        )
        .arg(Arg::with_name("rounds")
            .short("r")
            .long("rounds")
            .help("Number of rounds. If you give only one number, it will be used for every method given. Otherwise, they will be used in order for the methods given.")
            .takes_value(true)
            .empty_values(false)
            .required(true)
        )
        .get_matches();

    println!("{:#?}", matches);
}
