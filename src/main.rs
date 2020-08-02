use clap::{Arg, App};
use rand::prelude::*;
use rand_xorshift::XorShiftRng;
use std::default::Default;

const KORRUPTION_METHODS: [&'static str; 10] = ["zeros", "ones", "flip", "swap", "reverse", "repeat", "shorten", "addnoise", "rcycle", "lcycle"];
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

#[derive(Debug)]
enum KorruptionMethod {
    Zeros,
    Ones,
    Flip,
    Swap,
    Reverse,
    Repeat,
    Shorten,
    AddNoise,
    RCycle,
    LCycle,
}

#[derive(Debug)]
struct PartLengthGenerator {
    is_fixed: bool,
    prng: XorShiftRng,
    min: u64,
    max: u64,
}

impl PartLengthGenerator {
    pub fn new(min: u64, max: u64, seed: u64) -> Self {
        if min > max {
            return PartLengthGenerator {
                is_fixed: min == max,
                prng: XorShiftRng::seed_from_u64(seed),
                min: max,
                max: min,
            };
        }
        PartLengthGenerator {
            is_fixed: min == max,
            prng: XorShiftRng::seed_from_u64(seed),
            min,
            max,
        }
    }

    pub fn get_len(&mut self) -> u64 {
        if self.is_fixed {
            return self.min;
        }
        self.prng.next_u64() % (self.max - self.min + 1) + self.min
    }
}

impl Default for PartLengthGenerator {
    fn default() -> Self {
        PartLengthGenerator {
            is_fixed: true,
            prng: XorShiftRng::seed_from_u64(0),
            min: 0,
            max: 0,
        }
    }
}

#[derive(Debug)]
struct KorruptionMethodConfig {
    pub method: KorruptionMethod,
    pub rounds: u64,
    pub part_len: PartLengthGenerator,
}

impl KorruptionMethodConfig {
    pub fn new(method: KorruptionMethod, rounds: u64, part_len: PartLengthGenerator) -> Self {
        KorruptionMethodConfig {
            method,
            rounds,
            part_len
        }
    }

    // pub fn process(&mut self, bin_data: Vec<bool>) -> Vec<bool> {
    //
    // }
}

#[derive(Debug)]
struct Korruption {
    pub methods: Vec<KorruptionMethodConfig>,
    pub seed: u64,
}

fn main() {
    // let matches = App::new("Korrupt")
    //     .version("Test Version")
    //     .author("Author: PhirosWolf")
    //     .about("About: A little corruption tool")
    //     .arg(Arg::with_name("corruption method")
    //         .short("m")
    //         .long("method")
    //         .help("Sets the corruption method to use")
    //         .takes_value(true)
    //         .possible_values(&KORRUPTION_METHODS)
    //         .required(true)
    //     )
    //     .arg(Arg::with_name("min part length")
    //         .long("minpartlen")
    //         .help("Minimum part length")
    //         .takes_value(true)
    //         .required_unless("fixed part length")
    //         .requires("max part length")
    //         .empty_values(false)
    //     )
    //     .arg(Arg::with_name("max part length")
    //         .long("maxpartlen")
    //         .help("Maximum part length")
    //         .takes_value(true)
    //         .required_unless("fixed part length")
    //         .requires("min part length")
    //         .empty_values(false)
    //     )
    //     .arg(Arg::with_name("fixed part length")
    //         .long("fixedpartlen")
    //         .help("Fixed part length")
    //         .takes_value(true)
    //         .required_unless_all(&["min part length", "max part length"])
    //         .empty_values(false)
    //     )
    //     .arg(Arg::with_name("output")
    //         .short("o")
    //         .long("output")
    //         .help("Output file path")
    //         .takes_value(true)
    //         .default_value("/tmp/korrupt/@@filename@@")
    //         .empty_values(false)
    //         .value_name("PATH")
    //         .max_values(1)
    //     )
    //     .arg(Arg::with_name("input")
    //         .help("Input file path")
    //         .required(true)
    //         .index(1)
    //     )
    //     .arg(Arg::with_name("rounds")
    //         .short("r")
    //         .long("rounds")
    //         .help("Number of rounds. If you give only one number, it will be used for every method given. Otherwise, they will be used in order for the methods given.")
    //         .takes_value(true)
    //         .empty_values(false)
    //         .required(true)
    //     )
    //     .arg(Arg::with_name("seed")
    //         .short("s")
    //         .long("seed")
    //         .help("Random seed")
    //         .takes_value(true)
    //         .empty_values(false)
    //         .max_values(1)
    //     )
    //     .get_matches();

        println!("{:?}", to_binary(10u8));
        println!("{:?}", to_binary(255u8));
        println!("{:?}", to_binary(99u8));
        // println!("{:?}", from_binary(&to_binary(10u8)[..]));
        // println!("{:?}", from_binary(&to_binary(255u8)[..]));
        println!("{:?}", arr_to_binary(vec![10, 255, 99]));
}

fn to_binary(input: u8) -> Vec<bool> {
    let mut rslt: Vec<bool> = vec![];
    for i in (0..8).rev() {
        let mask = (2u8).pow(i);
        rslt.push((input & mask) == mask);
    }
    rslt
}

fn from_binary(mut input: &[bool]) -> u8 {
    let mut rslt = 0u8;
    for (i, item) in (0..8).rev().zip(input[0..8].iter()) {
        if *item {
            rslt += (2u8).pow(i);
        }
    }
    rslt
}

fn arr_to_binary(input: Vec<u8>) -> Vec<bool> {
    input.iter().map(|el| to_binary(*el)).fold(vec![], |mut acc, mut x| {
        acc.append(&mut x);
        return acc;
    })
}

// fn arr_from_binary(input: Vec<bool>) -> Vec<u8> {
//
// }
