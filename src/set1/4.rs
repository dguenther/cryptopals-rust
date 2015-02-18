// Single-byte XOR cipher
// The hex encoded string:

// 1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736
// ... has been XOR'd against a single character. Find the key, decrypt the message.

// You can do this by hand. But don't: write code to do it for you.

// How? Devise some method for "scoring" a piece of English plaintext.
// Character frequency is a good metric. Evaluate each output and choose the one with the best score.

// i/o isn't tested right now, so let's silence that warning
#![allow(unused_features)]

#![feature(core)]
#![feature(env)]
#![feature(io)]
#![feature(os)]
#![feature(path)]
#![feature(std_misc)]

#[macro_use]
extern crate log;

use std::ascii::AsciiExt;
use std::num::Int;

use std::str;

#[cfg(not(test))]
use std::env;
#[cfg(not(test))]
use std::old_io::BufferedReader;
#[cfg(not(test))]
use std::old_io::File;

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

	let output = match arg.into_string() {
		Ok(s) => detect_xor_in_file(s.as_slice()),
		Err(_) => panic!("Invalid path")
	};
	println!("output string: {:?}", output);
}

#[cfg(not(test))]
fn detect_xor_in_file(path: &str) -> String {
	let path = Path::new(path);
	let mut file = BufferedReader::new(File::open(&path));
	let lines = file.lines().map(|x| x.unwrap()).collect();

	detect_xor_in_lines(lines)

}

fn detect_xor_in_lines(lines: Vec<String>) -> String {
	let mut best_string_score = 0;
	let mut best_string = String::new();

	for line in lines.iter() {
	    let pairs = convert_hex_string_to_decimal_pairs(line);
	    match score_and_xor(pairs) {
	    	(score, string) => {
	    		if score > best_string_score {
	    			best_string_score = score;
	    			best_string = string;
	    		}
	    	}
	    };
	}

	best_string
}

fn score_and_xor(decimal_values: Vec<u8>) -> (usize, String) {
	let mut best_string = String::new();
	let mut best_string_score = 0;

	// starting with 0, test the current string just in case
	for test_val in 0..255 {
		debug!("{:?}", test_val);
		let mut decoded_values = vec!();
		for &x in decimal_values.iter() {
			decoded_values.push(x ^ test_val);
		}

		// turn the byte vector into a string
		match str::from_utf8(decoded_values.as_slice()) {
		    Ok(v) => {
		        let score = score_text(v);
		        if score > best_string_score {
		        	best_string = v.to_string();
		        	best_string_score = score;
		        }
		    }
		    Err(_) => { }
		}
	}
	(best_string_score, best_string)
}

fn convert_hex_string_to_decimal_pairs(string: &String) -> Vec<u8> {
	let string_lower = string.to_ascii_lowercase();
	let bytes = string_lower.as_bytes();

	let mut decimal_values = vec!();
	let mut tick = 1;
	let mut current_byte = 0;

	for &x in bytes.iter() {
		current_byte += convert_hex_char_to_decimal(x) * 16.pow(tick);
		tick = tick ^ 1;
		if tick == 1 {
			decimal_values.push(current_byte);
			current_byte = 0;
		}
	}

	decimal_values
}

fn convert_hex_char_to_decimal(character: u8) -> u8 {
	// if x is in the ascii range for numbers, subtract 48,
	// which is '0' in ascii. Otherwise, it should be a lowercase letter.
	// 'a' is 97, so subtract 87.
	match character < 58 {
		true => character - 48,
		false => character - 87
	}
}

// these are strings so that they can be used in StrExt.replace.
// this seemed easier than converting chars to strings every time.
// characters are taken from relative frequency of letters in the english language:
// https://en.wikipedia.org/wiki/Letter_frequency
static VALUED_CHARS: &'static[&'static str] = &["e", "t", "a", "o", "n"];

// this scoring function is extremely simple/fragile. that said,
// it completes this challenge
fn score_text(text: &str) -> usize {
	let mut score: usize = 0;
	// if we have \u{0} in the output, the text is probably the opposite
	// of what it needs to be, so throw this string out.
	// (the xor-opposite of \u{0} is a space)
	if text.len() != text.replace("\u{0}", "").len() {
		return 0;
	}
	let lowercase = text.to_ascii_lowercase();
	for &test_char in VALUED_CHARS {
		let test_str = lowercase.replace(test_char, "");
		score += text.len() - test_str.len();
		debug!("input: {:?}, test: {:?}, score increase: {:?}", text, test_str, text.len() - test_str.len());
	}
	return score;
}

#[cfg(test)]
mod set1challenge4 {

	#[test]
	fn test() {
		let output = super::detect_xor_in_lines(vec!("1234567890ABCDEF123456".to_string(), "4574626531626563787f76".to_string(), "FEDCBA0987654321FEDCBA".to_string()));
		assert_eq!(output, "Test string");
	}

}
