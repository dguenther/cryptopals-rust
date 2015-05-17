
use std::str;

/// Xors a string with values 0-255, scoring each xored string using
/// the score::score_text function and returning the best result.
///
/// Returns the best string's score, the value xored to get the best
/// string, and the best string itself.
///
/// # Examples
/// ```
/// let nums = vec!(77, 97, 110);
/// let (score, value, best_string) = cryptopalslib::xor::score_and_xor(nums);
/// println!("{} {} {}", score, value, best_string);
/// ```
pub fn score_and_xor(decimal_values: Vec<u8>) -> (usize, u8, String) {
    let mut best_string = String::new();
    let mut best_string_score = 0;
    let mut best_string_value = 0;

    // starting with 0, test the current string just in case
    for test_val in 0..255 {
        debug!("{:?}", test_val);

        let decoded_values: Vec<_> = decimal_values.iter().map(|x| x ^ test_val).collect();

        // turn the byte vector into a string
        match str::from_utf8(&decoded_values) {
            Ok(v) => {
                let score = ::score::score_text(v);
                if score > best_string_score {
                    best_string = v.to_string();
                    best_string_score = score;
                    best_string_value = test_val;
                }
            }
            Err(_) => { }
        }
    }

    (best_string_score, best_string_value, best_string)
}
