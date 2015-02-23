// Convert hex to base64
// The string:
//
// 49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d
// Should produce:
//
// SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t
// So go ahead and make that happen. You'll need to use this code for the rest of the exercises.
//
// Cryptopals Rule
// Always operate on raw bytes, never on encoded strings. Only use hex and base64 for pretty-printing.

#![feature(core)]
#![feature(collections)]

#[macro_use]
extern crate log;

use std::ascii::AsciiExt;
use std::num::Int;
use std::str;

#[cfg(not(test))]
fn main() {
	println!("Set 1, Challenge 1");
	let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
	println!("input string: {:?}", input);
	let output = hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
	println!("output string: {:?}", output);
}

fn hex_to_base64(hex_string: &str) -> String {
	if hex_string.len() % 2 != 0 {
		panic!("Hex string length should be even.");
	}

	let hex_lower = hex_string.to_ascii_lowercase();

	let bytes = hex_lower.as_bytes();

	let mut shift: i8 = 20;
	let mut string_group = 0;
	let mut output_vec = vec!();

	for &x in bytes.iter() {

		// if x is in the ascii range for numbers, subtract 48,
		// which is '0' in ascii. Otherwise, it should be a lowercase letter.
		// 'a' is 97, so subtract 87.
		let byte = match x < 58 {
			true => x - 48,
			false => x - 87
		};

		let test = (byte as usize) << shift;
		debug!("byte: {:?}, test: {:?}, shift: {:?}", byte, test, shift);
		string_group += test;

		shift -= 4;

		if shift < 0 {
			output_vec.append(&mut convert_string_group(string_group, 3));
			shift = 20;
			string_group = 0;
		}

	}

	if string_group != 0 {
		// assuming an even number of hex chars, shift will either be 12 or 4.
		// if 12, we have 1 significant byte, and if 4 we have 2.
		let significant_bytes: usize = (2 - (shift / 8)) as usize;
		output_vec.append(&mut convert_string_group(string_group, significant_bytes));
	}

	// turn the byte vector into a string
	match str::from_utf8(output_vec.as_slice()) {
	    Ok(v) => {
	        debug!("output string: {:?}", v);
	        return v.to_string();
	    }
	    Err(e) => {
	        panic!("error parsing string: {:?}", e);
    	}
	}
}

fn convert_string_group(group: usize, significant_bytes: usize) -> Vec<u8> {
	let mut converted_vec = vec!();
	let octets = vec!(
		((group & (63 * 64.pow(3))) >> 18) as u8,
		((group & (63 * 64.pow(2))) >> 12) as u8,
		((group & (63 * 64)) >> 6) as u8,
		(group & 63) as u8,
	);
	debug!("group: {:?}", group);
	debug!("octets: {:?}", octets);
	for &octet in octets.iter() {
		converted_vec.push(
			match octet {
				0...25 => octet + 65, // uppercase
				26...51 => octet + 71, // lowercase
				52...61 => octet - 4, // numbers
				62 => 43, // +
				63 => 47, // /
				_ => panic!("Not a valid octet")
			}
		);
	}

	// if we have less than 3 significant bytes, truncate insignificant chars
	// and replace them with '=' characters
	if significant_bytes < 3 {
		converted_vec.truncate(significant_bytes + 1);
		converted_vec.resize(4, 61);
	}

	converted_vec
}

#[cfg(test)]
mod set1challenge1 {

	#[test]
	fn challenge() {
		let output = super::hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
		assert_eq!(output, "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
	}

	#[test]
	fn ff() {
		let output = super::hex_to_base64("ff");
		assert_eq!(output, "/w==");
	}

	#[test]
	fn ffff() {
		let output = super::hex_to_base64("ffff");
		assert_eq!(output, "//8=");
	}

	#[test]
	fn uppercase() {
		let output = super::hex_to_base64("4D616E");
		assert_eq!(output, "TWFu");
	}
}
