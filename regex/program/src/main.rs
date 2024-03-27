#![no_main]
sp1_zkvm::entrypoint!(main);
use core::panic;

use regex::Regex;

pub fn main() {
    // Read two inputs from the prover: a regex pattern and a target string.
    let pattern = sp1_zkvm::io::read::<String>();
    let target_string = sp1_zkvm::io::read::<String>();

    // try to compile the regex pattern, if is fails write false as output and return
    let regex = match Regex::new(pattern.as_str()) {
        Ok(regex) => regex,
        Err(_) => {
           panic!("Invalid regex pattern");
        }
    };
    
    let result = regex.is_match(&target_string);

    sp1_zkvm::io::write(&result);

}
