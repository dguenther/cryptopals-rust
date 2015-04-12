// Implement repeating-key XOR
// Here is the opening stanza of an important work of the English language:

// Burning 'em, if you ain't quick and nimble
// I go crazy when I hear a cymbal
// Encrypt it, under the key "ICE", using repeating-key XOR.

// In repeating-key XOR, you'll sequentially apply each byte of the key;
// the first byte of plaintext will be XOR'd against I, the next C,
// the next E, then I again for the 4th byte, and so on.

// It should come out to:

// 0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272
// a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f

// Encrypt a bunch of stuff using your repeating-key XOR function.
// Encrypt your mail. Encrypt your password file. Your .sig file.
// Get a feel for it. I promise, we aren't wasting your time with this.

#[macro_use]
extern crate log;

use std::str;

#[cfg(not(test))]
fn main() {
	let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
	let key = "ICE";
	println!("input: {:?}, key: {:?}", input, key);
	let output = encrypt_repeating_byte_xor(input, key);
	println!("output: {:?}", output);
}

fn encrypt_repeating_byte_xor(input: &str, key: &str) -> String {
	let mut output: Vec<u8> = vec!();
	let input_iter = input.bytes();
	let mut key_iter = key.bytes().cycle();
	for byte in input_iter {
		output.push(byte ^ key_iter.next().unwrap());
	}
	convert_decimals_to_hex_string( output )
}

fn convert_decimals_to_hex_string(decimals: Vec<u8>) -> String {
	let mut converted_string: Vec<u8> = vec!();
	for num in decimals {
		if num < 16 {
			converted_string.push(convert_decimal_to_hex_char(0));
			converted_string.push(convert_decimal_to_hex_char(num));
		} else {
			converted_string.push(convert_decimal_to_hex_char(num / 16));
			converted_string.push(convert_decimal_to_hex_char(num % 16));
		}
	}

	// turn the byte vector into a string
	match str::from_utf8(&converted_string) {
	    Ok(v) => {
	        return v.to_string();
	    }
	    Err(_) => { panic!("Error converting numbers to string") }
	}
}

fn convert_decimal_to_hex_char(decimal: u8) -> u8 {
	match decimal {
		0...9 => decimal + 48,
		10...15 => decimal + 87,
		_ => panic!("Not a valid hex char")
	}
}

#[cfg(test)]
mod set1challenge5 {

	#[test]
	fn challenge() {
		let output = super::encrypt_repeating_byte_xor("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal", "ICE");
		assert_eq!(output, "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f".to_string());
	}

	#[test]
	fn wizard() {
		let output = super::encrypt_repeating_byte_xor("I like the wizard of oz", "TEST");
		assert_eq!(output, "1d653f3d3f2073203c2073233d3f322630653c32742a29".to_string());
	}

}
