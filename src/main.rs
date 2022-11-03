//! This project is not very useful, but I still found it fun to write.

use num_bigint::BigUint;
use num_format::{Locale, ToFormattedString};
use clap::Parser;

/// A struct used for parsing the CLI arguments
#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[clap(short, long, default_value = "4", value_name = "DECIMAL PRECISION")]
    digits: u32,

    #[clap(short, long, default_value = "3", value_name = "TARGET NUMBER")]
    number: u32,
}

/// Entry of the program.
fn main() {
    // load the arguments
    let args = Args::parse();

    // load from arguments the precision requested
    let decimal_precision_modulo = BigUint::from(10u64).pow(args.digits);

    // load from arguments the number we want to compute the power tower of
    let n = BigUint::from(args.number);

    // compute the tetration
    let mut tetration_n = n.clone();
    tetration_n = n.clone().modpow(&tetration_n, &decimal_precision_modulo);
    tetration_n = n.clone().modpow(&tetration_n, &decimal_precision_modulo);
    tetration_n = n.clone().modpow(&tetration_n, &decimal_precision_modulo);
    
    // show the results
    println!("The last {precision} digits of {n}^{n}^{n}^{n} are :", precision = args.digits, n = args.number);
    println!("..{formatted_result}", formatted_result = tetration_n.to_formatted_string(&Locale::en))
}
