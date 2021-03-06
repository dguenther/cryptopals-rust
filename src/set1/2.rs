// Fixed XOR
// Write a function that takes two equal-length buffers and produces their XOR combination.
//
// If your function works properly, then when you feed it the string:
//
// 1c0111001f010100061a024b53535009181c
// ... after hex decoding, and when XOR'd against:
//
// 686974207468652062756c6c277320657965
// ... should produce:
//
// 746865206b696420646f6e277420706c6179

#[macro_use]
extern crate log;
extern crate cryptopalslib;

use std::ascii::AsciiExt;
use std::str;

#[cfg(not(test))]
fn main() {
	println!("Set 1, Challenge 2");
	let string1 = "1c0111001f010100061a024b53535009181c";
	let string2 = "686974207468652062756c6c277320657965";
	println!("string 1: {:?}, string 2: {:?}", string1, string2);
	let output = fixed_hex_xor(string1, string2);
	println!("output string: {:?}", output);
}

fn fixed_hex_xor(first_string: &str, second_string: &str) -> String {
	if first_string.len() != second_string.len() {
		panic!("String lengths should be even.");
	}

	let mut output_vec = vec!();

	let first_lower = first_string.to_ascii_lowercase();
	let second_lower = second_string.to_ascii_lowercase();

	let first_iter = first_lower.as_bytes().iter();
	let second_iter = second_lower.as_bytes().iter();

	for (&x, &y) in first_iter.zip(second_iter) {
		let first_num = cryptopalslib::convert::hex_char_to_decimal(x);
		let second_num = cryptopalslib::convert::hex_char_to_decimal(y);
        output_vec.push(cryptopalslib::convert::decimal_to_hex_char(first_num ^ second_num));
	}

	// turn the byte vector into a string
	match str::from_utf8(&output_vec) {
	    Ok(v) => {
	        debug!("output string: {:?}", v);
	        return v.to_string();
	    }
	    Err(e) => {
	        panic!("error parsing string: {:?}", e);
    	}
	}
}

#[cfg(test)]
mod set1challenge2 {

	#[test]
	fn challenge() {
		let output = super::fixed_hex_xor("1c0111001f010100061a024b53535009181c", "686974207468652062756c6c277320657965");
		assert_eq!(output, "746865206b696420646f6e277420706c6179");
	}

}
