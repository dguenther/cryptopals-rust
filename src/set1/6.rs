// There's a file here. (6.txt) It's been base64'd after being encrypted with
// repeating-key XOR.

// Decrypt it.

// Here's how:

// 1. Let KEYSIZE be the guessed length of the key;
// try values from 2 to (say) 40.
// 2. Write a function to compute the edit distance/Hamming distance between
// two strings. The Hamming distance is just the number of differing bits.
// The distance between:

// this is a test

// and

// wokka wokka!!!

// is 37. Make sure your code agrees before you proceed.

// 3. For each KEYSIZE, take the first KEYSIZE worth of bytes,
// and the second KEYSIZE worth of bytes, and find the edit distance
// between them. Normalize this result by dividing by KEYSIZE.

// 4. The KEYSIZE with the smallest normalized edit distance is probably
// the key. You could proceed perhaps with the smallest 2-3 KEYSIZE values.
// Or take 4 KEYSIZE blocks instead of 2 and average the distances.

// 5. Now that you probably know the KEYSIZE: break the ciphertext into blocks
// of KEYSIZE length.

// 6. Now transpose the blocks: make a block that is the first byte of every
// block, and a block that is the second byte of every block, and so on.

// 7. Solve each block as if it was single-character XOR. You already have code
// to do this.

// 8. For each block, the single-byte XOR key that produces the best looking
// histogram is the repeating-key XOR key byte for that block. Put them
// together and you have the key.

// This code is going to turn out to be surprisingly useful later on.
// Breaking repeating-key XOR ("Vigenere") statistically is obviously an
// academic exercise, a "Crypto 101" thing. But more people "know how" to break
// it than can actually break it, and a similar technique breaks something much
// more important.

#![feature(collections)]
#![feature(core)]

#![cfg(not(test))]
#![feature(env)]
#![feature(old_io)]
#![feature(old_path)]

#[macro_use]
extern crate log;

use std::ascii::AsciiExt;
use std::collections::BitVec;
use std::cmp;
use std::cmp::Ordering;
use std::iter::range_step;
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

	let (key, output) = break_repeating_key_xor_in_file(arg.as_slice());
	println!("key: {:?}", key);
	println!("output: {:?}", output);
}

#[cfg(not(test))]
fn break_repeating_key_xor_in_file(path: &str) -> (String, String) {
	let path = Path::new(path);
	let mut file = BufferedReader::new(File::open(&path));
	let lines = file.lines().map(|x| x.unwrap()).collect();

	break_repeating_key_xor_in_lines(lines)
}

fn break_repeating_key_xor_in_lines(lines: Vec<String>) -> (String, String) {
	let input = base64_lines_to_hex(lines);
	let nums = convert_hex_string_to_decimal_pairs(&input);
	break_repeating_key_xor(nums)
}

fn base64_lines_to_hex(lines: Vec<String>) -> String {
	let mut output = String::new();
	for line in lines {
		output.push_str(base64_to_hex(line.trim().as_slice()).as_slice());
	}
	output
}

fn break_repeating_key_xor(bytes: Vec<u8>) -> (String, String) {
	let mut results: Vec<(f32, usize)> = Vec::new();
	debug!("{:?}", bytes.len());
	for keysize in 2..cmp::min(40, (bytes.len() / 3)) {
		// take first keysize of bytes
		let first = &bytes[0..keysize];
		// take second keysize of bytes
		let second = &bytes[keysize..keysize*2];

		// take third keysize of bytes
		let third = &bytes[keysize*2..keysize*3];

		let distance = (hamming_distance(first, second) + hamming_distance(second, third) + hamming_distance(first, third)) / 3;
		let normalized = distance as f32 / keysize as f32;
		results.push((normalized, keysize));
	}

	// sort the vector from least to greatest distance.
	results.sort_by(|&(x1, _), &(x2, _)| x1.partial_cmp(&x2).unwrap_or(Ordering::Equal));
	debug!("{:?}", results);

	// loop over the vector of keysizes
	for &(_, first) in results.iter() {
		debug!("{:?}", first);
		let mut output_strings = vec!();
		let mut best_keys = vec!();
		for index in 0..first {
			let filtered_bytes: Vec<_> = bytes.iter().enumerate()
				.filter(
					|&(x, _)| {
						if index > x {
							return false
						} else {
							return (x - index) % first == 0
						}
					})
				.map(|(_, &y)| y)
				.collect();

			let (best_key, string) = score_and_xor(filtered_bytes);

			// if we get an empty string back, we don't have any valid text,
			// so throw out this keysize
			if string == "" {
				best_keys = vec!();
				output_strings = vec!();
				break;
			}

			best_keys.push(best_key);
			output_strings.push(string);
		}

		// if we've got something in the output string list,
		// we made it all the way through, so merge the strings

		if output_strings.len() != 0 {
			let mut output_string = String::new();
			let string_len = output_strings[0].len();
			for index in 0..string_len {
				for str_index in 0..output_strings.len() {

					if index < output_strings[str_index].len() {
						output_string.push(output_strings[str_index].char_at(index));
					}
				}
			}

			let key = match str::from_utf8(best_keys.as_slice()) {
			    Ok(v) => v,
			    Err(_) => { panic!("Test") }
			};
			return (key.to_string(), output_string);
		}
	}
	("".to_string(), "".to_string())
}

fn hamming_distance(input1: &[u8], input2: &[u8]) -> usize {
	let first_iter = input1.iter();
	let second_iter = input2.iter();

	let mut output: usize = 0;

	for (x, y) in first_iter.zip(second_iter) {
		let bv = BitVec::from_bytes(&[x^y]);
		output += bv.iter().filter(|x| *x).count() as usize;
	}
	output
}

fn score_and_xor(decimal_values: Vec<u8>) -> (u8, String) {
	let mut best_string = String::new();
	let mut best_string_score = 0;
	let mut best_string_value = 0;

	// starting with 0, test the current string just in case
	for test_val in 0..255 {
		debug!("{:?}", test_val);

		let decoded_values: Vec<_> = decimal_values.iter().map(|x| x ^ test_val).collect();

		// turn the byte vector into a string
		match str::from_utf8(decoded_values.as_slice()) {
		    Ok(v) => {
		        let score = score_text(v);
		        if score > best_string_score {
		        	best_string = v.to_string();
		        	best_string_score = score;
		        	best_string_value = test_val;
		        }
		    }
		    Err(_) => { }
		}
	}

	(best_string_value, best_string)
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

static FAILED_CHARS: &'static[&'static str] = &["\u{0}", "\u{1}", "\u{2}", "\u{3}", "\u{4}", "\u{5}", "\u{6}", "\u{7}", "\u{8}", "\u{9}", "\u{12}", "\u{c}", "\u{7f}", "\u{1d}"];

// this scoring function is extremely simple/fragile. that said,
// it completes this challenge
fn score_text(text: &str) -> usize {
	let mut score: usize = 0;
	// if we have any failed characters in the text,
	// it's probably not text we want, so return 0

	for &test_char in FAILED_CHARS {
		if text.contains(test_char) {
			return 0;
		}
	}


	let lowercase = text.to_ascii_lowercase();
	for &test_char in VALUED_CHARS {
		let test_str = lowercase.replace(test_char, "");
		score += text.len() - test_str.len();
		debug!("input: {:?}, test: {:?}, score increase: {:?}", text, test_str, text.len() - test_str.len());
	}
	return score;
}

// BASE 64 TO HEX

fn base64_to_hex(input: &str) -> String {
	let bytes = input.as_bytes();
	let mut nums = vec!();
	for index in range_step(0, bytes.len(), 4) {
		let slice = &bytes[index..index+4];

		let byte0 = base64_ascii_to_index(slice[0]);
		let byte1 = base64_ascii_to_index(slice[1]);
		let byte2 = base64_ascii_to_index(slice[2]);
		let byte3 = base64_ascii_to_index(slice[3]);

		match (byte0, byte1, byte2, byte3) {
			(Some(b0), Some(b1), Some(b2), Some(b3)) => {
				nums.push(b0 >> 2);
				nums.push(((b0 & 3) << 2) + (b1 >> 4));
				nums.push(b1 & 15);
				nums.push((b2 & 60) >> 2);
				nums.push(((b2 & 3) << 2) + (b3 >> 4));
				nums.push(b3 & 15);
			},
			(Some(b0), Some(b1), Some(b2), None) => {
				nums.push(b0 >> 2);
				nums.push(((b0 & 3) << 2) + (b1 >> 4));
				nums.push(b1 & 15);
				nums.push((b2 & 60) >> 2);
			},
			(Some(b0), Some(b1), None, None) => {
				nums.push(b0 >> 2);
				nums.push(((b0 & 3) << 2) + (b1 >> 4));
			},
			_ => panic!("Only the last two bytes may be unused")
		};

	}

	let hex: Vec<u8> = nums.iter().map(|&x| convert_num_to_hex_char(x)).collect();

	match str::from_utf8(hex.as_slice()) {
	    Ok(v) => {
	        debug!("output string: {:?}", v);
	        return v.to_string();
	    }
	    Err(e) => {
	        panic!("error parsing string: {:?}", e);
    	}
	}

}

fn base64_ascii_to_index(ascii: u8) -> Option<u8> {
	match ascii {
		65...90 => Some(ascii - 65), // uppercase
		97...122 => Some(ascii - 71), // lowercase
		48...57 => Some(ascii + 4), // numbers
		43 => Some(ascii + 19), // +
		47 => Some(ascii + 16), // /
		// equals means empty byte. need some way of
		// differentiating from 'A'.
		61 => None, // =

		_ => panic!("Not in base64 range")
	}
}

fn convert_num_to_hex_char(num: u8) -> u8 {
	match num {
		0...9 => num + 48,
		10...15 => num + 87,
		_ => panic!("Not a valid hex char")
	}
}

#[cfg(test)]
mod set1challenge6 {

	#[test]
	fn hamming_distance() {
		let output = super::hamming_distance("this is a test".as_bytes(), "wokka wokka!!!".as_bytes());
		assert_eq!(output, 37);
	}

	#[test]
	fn base64_one_equals() {
		let output = super::base64_to_hex("oSNFZ4k=");
		assert_eq!(output, "a123456789");
	}

	#[test]
	fn base64_two_equals() {
		let output = super::base64_to_hex("EjRWeJq83g==");
		assert_eq!(output, "123456789abcde");
	}

	#[test]
	fn base64_man() {
		let output = super::base64_to_hex("TWFu");
		assert_eq!(output, "4d616e");
	}

	#[test]
	fn decode() {
		// taken from https://picoctf.com/crypto_mats/index.html
		let input = vec!("mIdwJYSyjmxxt7uZfnGVv4F6OIS/mDU4ifqffTTHvIp2Jceug3Qly/qeeyWOtstzMI6oh2xxlb+IcD+TtpI5cY6uy2IwlPqbZz6Fu4l5KMeYmXwlhrOFMiLHuI5mJcexjmUlx6mOdiOCrsU1BY+zmDU4lPqJcDKGr5hwcYi8y2E5gvqYcDKVv4hscZSvmWc+krSPfD+A+op5Pceug3BxhrmffCeOroJwIse5imcjjr+PNT6J+oNwI4L6j2AjjrSMNQaIqIdxcbC7mTUFkLXLYjCU+oRzcZGzn3Q9x7OGZT6Vrop7MoL6n3pxiK+ZNT+GroJ6P4a2y2Y0hK+ZfCWe+op7Ncevh2E4irufcHGRs4hhPpWjxQ==".to_string());
		let (key, output) = super::break_repeating_key_xor_in_lines(input);
		assert_eq!(output, "Bletchey Park rejoices in the fact that, until fairly recently, it was probably Britain's best kept secret. This is because of the secrecy surrounding all the activities carried on here during World War Two was of vital importance to our national security and ultimate victory.");
	}
}


