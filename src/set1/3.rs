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

use std::ascii::AsciiExt;

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
		current_byte += cryptopalslib::convert::hex_char_to_decimal(x) * 16u8.pow(tick);
		tick = tick ^ 1;
		if tick == 1 {
			decimal_values.push(current_byte);
			current_byte = 0;
		}
	}

	let (_, _, best_string) = cryptopalslib::xor::score_and_xor(decimal_values);

	best_string
}

#[cfg(test)]
mod set1challenge3 {

	#[test]
	fn challenge() {
		let output = super::decode_single_byte_xor("4574626531626563787f76");
		assert_eq!(output, "Test string");
	}

}
