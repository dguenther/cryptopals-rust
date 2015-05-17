
use std::ascii::AsciiExt;
use std::collections::BitVec;

/// Computes the hamming distance between two integer arrays.
///
/// # Examples
///
/// ```
/// let byte1 = "this is a test".as_bytes();
/// let byte2 = "wokka wokka!!!".as_bytes();
/// let output = cryptopalslib::score::hamming_distance(byte1, byte2);
/// assert_eq!(output, 37);
/// ```
pub fn hamming_distance(input1: &[u8], input2: &[u8]) -> usize {
    let first_iter = input1.iter();
    let second_iter = input2.iter();

    let mut output: usize = 0;

    for (x, y) in first_iter.zip(second_iter) {
        let bv = BitVec::from_bytes(&[x^y]);
        output += bv.iter().filter(|x| *x).count() as usize;
    }
    output
}

// these are strings so that they can be used in StrExt.replace.
// this seemed easier than converting chars to strings every time.
// characters are taken from relative frequency of letters in the english language:
// https://en.wikipedia.org/wiki/Letter_frequency
static VALUED_CHARS: &'static[&'static str] = &["e", "t", "a", "o", "n"];

static FAILED_CHARS: &'static[&'static str] = &[
    "\u{0}", "\u{1}", "\u{2}", "\u{3}", "\u{4}", "\u{5}", "\u{6}",
    "\u{7}", "\u{8}", "\u{9}", "\u{12}", "\u{c}", "\u{7f}", "\u{1d}"
];

/// Scores a string based on how likely it is the string is English.
///
/// # Examples
///
/// ```
/// let text = "This is a test";
/// let score = cryptopalslib::score::score_text(text);
/// println!("{}", score);
/// ```
pub fn score_text(text: &str) -> usize {
    // this scoring function is extremely simple/fragile. that said,
    // it completes the set1 challenges
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


#[cfg(test)]
mod test {

    #[test]
    fn hamming_distance() {
        let output = super::hamming_distance("this is a test".as_bytes(), "wokka wokka!!!".as_bytes());
        assert_eq!(output, 37);
    }

}