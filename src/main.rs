use clap::{Arg, App};
use rand::prelude::*;
use rand_xorshift::XorShiftRng;
use std::default::Default;
use std::num::NonZeroU64;
use std::convert::From;
use std::cmp;

const KORRUPTION_METHODS: [&'static str; 10] = ["zeros", "ones", "flip", "swap", "reverse", "repeat", "shorten", "addnoise", "interference", "rrotate", "lrotate"];
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
v-- Right rotation
RRotate: Shifts bits to the right but adds the now out of bounds bits to the other side.
    e.g. xxxxx|1111010100|xxxxx, intensity = 4
         xxxxx|yyyy111101|0100x becomes
         xxxxx|0100111101|xxxxx (obviously, the previous step is just for the sake of the explanation, not the real behaviour of the code)
v-- Left rotation
LRotate: Same as RRotate, but in the opposite direction
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
    Interference,
    RRotate,
    LRotate,
}

impl From<String> for Option<KorruptionMethod> {
    pub fn from(raw: String) -> Self {
        match raw.to_lowercase() {
            "zeros" => Some(KorruptionMethod::Zeros),
            "ones" => Some(KorruptionMethod::Ones),
            "flip" => Some(KorruptionMethod::Flip),
            "swap" => Some(KorruptionMethod::Swap),
            "reverse" => Some(KorruptionMethod::Reverse),
            "repeat" => Some(KorruptionMethod::Repeat),
            "shorten" => Some(KorruptionMethod::Shorten),
            "addnoise" => Some(KorruptionMethod::AddNoise),
            "interference" => Some(KorruptionMethod::Interference),
            "rrotate" => Some(KorruptionMethod::RRotate),
            "lrotate" => Some(KorruptionMethod::LRotate),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct PartLengthGenerator {
    is_fixed: bool,
    prng: XorShiftRng,
    min: u64,
    max: u64,
}

impl PartLengthGenerator {
    pub fn new(min: u64, max: u64, prng: XorShiftRng) -> Self {
        if min > max {
            return PartLengthGenerator {
                is_fixed: false,
                prng,
                min: max,
                max: min,
            };
        }
        PartLengthGenerator {
            is_fixed: min == max,
            prng,
            min,
            max,
        }
    }

    pub fn get_len(&mut self) -> u64 {
        if self.is_fixed {
            return self.min;
        }
        // Maybe adapt this thing with cmp::min and cmp::max ?
        self.prng.next_u64() % (self.max - self.min + 1) + self.min
    }

    pub fn get_origin(&mut self, max: u64) -> u64 {
        // Same for this ?
        self.prng.next_u64() % (max.saturating_add(1))
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
    pub part_gen: PartLengthGenerator,
}

impl KorruptionMethodConfig {
    pub fn new(method: KorruptionMethod, rounds: u64, part_len: PartLengthGenerator) -> Self {
        KorruptionMethodConfig {
            method,
            rounds,
            part_len
        }
    }

    pub fn process(&mut self, bin_data: &mut [bool], range_origin: u64, range_len: u64) {
        for _ in 0..self.rounds {
            self.process_once(bin_data, range_origin, range_len);
        }
    }

    pub fn process_once(&mut self, bin_data: &mut [bool], range_origin: u64, range_len: u64) {
        match self.method {
            KorruptionMethod::Zeros => {
                let part_len = cmp::min(self.part_gen.get_len(), bin_data.len());
                let part_origin = self.part_gen.get_origin(bin_data.len().saturating_sub(1));
                bin_data
                    .iter_mut()
                    .skip(range_origin.saturating_add(part_origin))
                    .take(part_len)
                    .for_each(|item| {
                        item = false;
                    });
            },
            KorruptionMethod::Ones => {
                let part_len = cmp::min(self.part_gen.get_len(), bin_data.len());
                let part_origin = self.part_gen.get_origin(bin_data.len().saturating_sub(1));
                bin_data
                    .iter_mut()
                    .skip(range_origin.saturating_add(part_origin))
                    .take(part_len)
                    .for_each(|item| {
                        item = true;
                    });
            },
            KorruptionMethod::Flip => {
                let part_len = cmp::min(self.part_gen.get_len(), bin_data.len().saturating_sub(1));
                let part_origin = self.part_gen.get_origin(bin_data.len().saturating_sub(1));
                bin_data
                    .iter_mut()
                    .skip(range_origin.saturating_add(part_origin))
                    .take(part_len)
                    .for_each(|item| {
                        item = !item;
                    });
            },
            KorruptionMethod::Swap => {
                // let part_one_len = cmp::min(self.part_gen.get_len(), bin_data.len().saturating_sub(1));
                // let part_one_origin = self.part_gen.get_origin(bin_data.len().saturating_sub(1));
                // let part_two_len = cmp::min(self.part_gen.get_len(), bin_data.len().saturating_sub(1));
                // let part_two_origin = self.part_gen.get_origin(bin_data.len().saturating_sub(1));
                // bin_data
                //     .iter_mut()
                //     .zip(
                //         bin_data
                //             .iter_mut()
                //     )
                //     .skip(range_origin.saturating_add(part_one_origin))
                //     .take(part_one_len)
                //     .for_each(|(item, item_to_swap_with)| {
                //         item =
                //     })
            },
            KorruptionMethod::Reverse => {
                // bin_data.iter_mut().skip(10).take(3).rev().
            },
            KorruptionMethod::Repeat => {

            },
            KorruptionMethod::Shorten => {

            },
            KorruptionMethod::AddNoise => {

            },
            KorruptionMethod::Interference => {

            },
            KorruptionMethod::RRotate => {

            },
            KorruptionMethod::LRotate => {

            },
        }
    }
}

#[derive(Debug)]
struct Range {
    pub from: u64,
    pub len: u64,
}

impl Range {
    pub fn new(from: u64, len: NonZeroU64) -> Self {
        Range {
            from,
            len: len.get(),
        }
    }
}

#[derive(Debug)]
struct Korruption {
    pub methods: Vec<KorruptionMethodConfig>,
    pub ranges: Vec<Range>,
    pub seed: u64,
}

impl Korruption {
    pub fn new() -> Self {
        Korruption {
            methods: vec![],
            ranges: vec![],
            seed: 0,
        }
    }

    pub fn method(&mut self, m: KorruptionMethodConfig) -> &mut Self {
        self.methods.push(m);
        self
    }

    pub fn methods(&mut self, mut m: Vec<KorruptionMethodConfig>) -> &mut Self {
        self.methods.append(&mut m);
        self
    }

    pub fn range(&mut self, range: Range) -> &mut Self {
        self.ranges.push(range);
        self
    }

    pub fn ranges(&mut self, mut ranges: Vec<Range>) -> &mut Self {
        self.ranges.append(&mut ranges);
        self
    }

    pub fn seed(&mut self, seed: u64) -> &mut Self {
        self.seed = seed;
        self
    }
}

fn main() {
    /*
    ./korrupt -m zeros,ones,shorten --minpartlen 2,3,4 --maxpartlen 10,11,12 --fixedpartlen _,3,_
    */
    let matches = App::new("Korrupt")
        .version("Test Version")
        .author("Author: PhirosWolf")
        .about("About: A little corruption tool")
        .arg(Arg::with_name("corruption method")
            .short("m")
            .long("method")
            .help("Sets the corruption method to use")
            .takes_value(true)
            .possible_values(&KORRUPTION_METHODS)
            .required(true)
        )
        .arg(Arg::with_name("min part length")
            .long("minpartlen")
            .help("Minimum part length")
            .takes_value(true)
            .required_unless("fixed part length")
            .requires("max part length")
            .empty_values(false)
        )
        .arg(Arg::with_name("max part length")
            .long("maxpartlen")
            .help("Maximum part length")
            .takes_value(true)
            .required_unless("fixed part length")
            .requires("min part length")
            .empty_values(false)
        )
        .arg(Arg::with_name("fixed part length")
            .long("fixedpartlen")
            .help("Fixed part length")
            .takes_value(true)
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
            .max_values(1)
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
        .arg(Arg::with_name("seed")
            .short("s")
            .long("seed")
            .help("Random seed")
            .takes_value(true)
            .empty_values(false)
            .max_values(1)
            .required(true)
        )
        .arg(Arg::with_name("range origin")
            .long("rangeorigin")
            .help("Range origin. The range is where korrupt is allowed to corrupt.")
            .takes_value(true)
            .empty_values(false)
            .required(true)
        )
        .arg(Arg::with_name("range length")
            .long("rangelength")
            .help("Range length")
            .takes_value(true)
            .empty_values(false)
            .required(true)
        )
        .get_matches();

    let mut korruption = Korruption::new();
    korruption.seed(matches.value_of("seed").unwrap_or("0").parse::<u64>().unwrap_or(0));
    matches
        .values_of("range origin")
        .expect("Expected range origins")
        .zip(matches
            .values_of("range length")
            .expect("Expected range lengths")
        )
        .map(|(origin, length)| (
            origin
                .parse::<u64>()
                .expect("Range origin must be a number"),
            NonZeroU64::new(
                length
                    .parse::<u64>()
                    .expect("Range length must a number")
            ).expect("Range length must not be zero")
        ))
        .for_each(|(origin, length)|
            korruption.range(Range::new(origin, length))
        );

    let seed = matches.value_of("seed").expect("Expected seed").parse::<u64>().expect("Seed must be a number");
    let main_rng = XorShiftRng::seed_from_u64(seed);

    // issue with this method: the length of `methods` is not properly taken into account
    matches
        .values_of("fixed part length")
        .map(|values| values.map(|value| value.parse::<u64>().expect("Fixed part length must be a number")))
        .unwrap_or(vec![usize::MAX].to_iter())
        // Min part length
        .zip(matches
            .values_of("min part length")
            .map(|values| values.map(|value| value.parse::<u64>().expect("Min part length must be a number")))
            .unwrap_or(vec![usize::MIN].to_iter().cycle())
        )
        // Max part length
        .zip(matches
            .values_of("max part length")
            .map(|values| values.map(|value| value.parse::<u64>().expect("Max part length must be a number")))
            .unwrap_or(vec![usize::MAX].to_iter().cycle())
        )
        // Methods
        .zip(matches
            .values_of("methods")
            .expect("Expected methods")
            .map(|values| values.map(|value| value.into::<Option<KorruptionMethod>>().expect("Method must be one of the valid methods")))
        )
        // Rounds
        .zip(matches
            .values_of("rounds")
            .map(|values| values.map(|value| value.parse::<u64>().expect("Round must be a number")))
            .unwrap_or(vec![1].to_iter().cycle())
        )
        // Pack them all together and append them all
        .for_each(|(fixed, min, max, method, rounds)| {
            main_rng.next_u64();
            korruption.method(
                KorruptionMethodConfig::new(
                    method,
                    rounds,
                    if fixed == usize::MAX {
                        PartLengthGenerator::new(fixed, fixed, XorShiftRng::from_rng(main_rng))
                    } else {
                        PartLengthGenerator::new(min, max, XorShiftRng::from_rng(main_rng))
                    }
                )
            )
        });

}

fn to_binary(input: u8) -> Vec<bool> {
    let mut rslt: Vec<bool> = Vec::with_capacity(8);
    for i in (0..8).rev() {
        let mask = 1 << i;
        rslt.push((input & mask) == mask);
    }
    rslt
}

fn from_binary(mut input: &[bool]) -> u8 {
    let mut rslt = 0u8;
    for item in input[0..8].iter() {
        rslt = (rslt << 1) | (*item as u8);
    }
    rslt
}

fn arr_to_binary(input: Vec<u8>) -> Vec<bool> {
    input.into_iter().map(|el| to_binary(el).into_iter()).flatten().collect()
}

fn arr_from_binary(input: Vec<bool>) -> Vec<u8> {
    input.chunks(8).map(|el| from_binary(&el)).collect()
}
