// Single-byte XOR cipher
// The hex encoded string:

// 1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736
// ... has been XOR'd against a single character. Find the key, decrypt the message.

// You can do this by hand. But don't: write code to do it for you.

// How? Devise some method for "scoring" a piece of English plaintext.
// Character frequency is a good metric. Evaluate each output and choose the one with the best score.

#[macro_use]
extern crate log;
extern crate cryptopalslib;

#[cfg(not(test))]
use std::env;
#[cfg(not(test))]
use std::io::BufReader;
#[cfg(not(test))]
use std::fs::File;
#[cfg(not(test))]
use std::path::Path;
#[cfg(not(test))]
use std::io::prelude::*;

#[cfg(not(test))]
fn main() {
	if env::args().count() < 2 {
		panic!("Must pass a file to decode")
	}

	let arg = match env::args().nth(1) {
		Some(s) => s,
		None => panic!("No input argument given")
	};
	println!("input: {:?}", arg);
	let output = detect_xor_in_file(&arg);
	println!("output string: {:?}", output);
}

#[cfg(not(test))]
fn detect_xor_in_file(path: &str) -> String {
	let path = Path::new(path);
	let f = File::open(&path).unwrap();
	let file = BufReader::new(f);
	let lines = file.lines().map(|x| x.unwrap()).collect();

	detect_xor_in_lines(lines)
}

fn detect_xor_in_lines(lines: Vec<String>) -> String {
	let mut best_string_score = 0;
	let mut best_string = String::new();

	for line in lines.iter() {
	    let pairs = cryptopalslib::convert::hex_string_to_decimal_pairs(line);
	    let (score, _, string) = cryptopalslib::xor::score_and_xor(pairs);
	    if score > best_string_score {
			best_string_score = score;
			best_string = string;
	    }
	}

	best_string
}

#[cfg(test)]
mod set1challenge4 {

	#[test]
	fn test() {
		let output = super::detect_xor_in_lines(vec!("1234567890ABCDEF123456".to_string(), "4574626531626563787f76".to_string(), "FEDCBA0987654321FEDCBA".to_string()));
		assert_eq!(output, "Test string");
	}

}
