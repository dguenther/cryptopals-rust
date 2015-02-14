// Single-byte XOR cipher
// The hex encoded string:

// 1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736
// ... has been XOR'd against a single character. Find the key, decrypt the message.

// You can do this by hand. But don't: write code to do it for you.

// How? Devise some method for "scoring" a piece of English plaintext.
// Character frequency is a good metric. Evaluate each output and choose the one with the best score.


#![feature(core)]
#![feature(std_misc)]

#[macro_use]
extern crate log;

use std::ascii::AsciiExt;
use std::num::Int;
use std::str;

#[cfg(not(test))]
fn main() {
	println!("Set 1, Challenge 3");
	let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
	println!("input: {:?}", input);
	let output = decode_single_byte_xor(input);
	println!("output string: {:?}", output);
}

fn decode_single_byte_xor(input: &str) -> String {
	if input.len() % 2 != 0 {
		panic!("Hex string length should be even.");
	}

	// make vector of hex-decoded values
	// for each value from 0 to 255
	//    for each char in vector, xor it and add it to a temp vector
	//    turn vector into string
	//    score string
	//    if string score better than best_string
	//       update best_string, best_string_score

	let mut decimal_values = vec!();

	let input_lower = input.to_ascii_lowercase();
	let bytes = input_lower.as_bytes();

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

	best_string
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
	let lowercase = text.to_ascii_lowercase();
	for &test_char in VALUED_CHARS {
		let test_str = lowercase.replace(test_char, "");
		score += text.len() - test_str.len();
		debug!("input: {:?}, test: {:?}, score increase: {:?}", text, test_str, text.len() - test_str.len());
	}
	return score;
}

#[cfg(test)]
mod set1challenge3 {

	#[test]
	fn challenge() {
		let output = super::decode_single_byte_xor("4574626531626563787f76");
		assert_eq!(output, "Test string");
	}

}
