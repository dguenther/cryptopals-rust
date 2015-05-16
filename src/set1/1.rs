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

#[macro_use]
extern crate log;
extern crate cryptopalslib;

#[cfg(not(test))]
fn main() {
	println!("Set 1, Challenge 1");
	let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
	println!("input string: {:?}", input);
	let output = cryptopalslib::convert::hex_to_base64(input);
	println!("output string: {:?}", output);
}

#[cfg(test)]
mod set1challenge1 {

	extern crate cryptopalslib;

	#[test]
	fn challenge() {
		let output = cryptopalslib::convert::hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
		assert_eq!(output, "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
	}

}
