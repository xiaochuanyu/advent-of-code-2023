use std::cmp::min;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const DIGIT_WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
fn matches_digit_word(vec: &[char]) -> Option<usize> {
    let result = DIGIT_WORDS
        .iter()
        .enumerate()
        .find(|(_, word)| {
            vec.len() >= word.len() && word.chars().zip(vec.iter()).all(|(a, b)| a == *b)
        })
        .map(|(i, _)| i + 1);
    result
}
fn process_input_file<R: Read>(mut reader: BufReader<R>) -> u32 {
    let mut line = String::new();
    let mut sum = 0;

    let max_digit_word_len = DIGIT_WORDS.iter().map(|w| w.len()).max().unwrap();
    loop {
        let len = reader.read_line(&mut line).expect("error reading line");
        if len == 0 {
            break;
        }
        let v: Vec<char> = line.chars().collect();
        let mut left_dig: Option<u32> = None;
        for (i, c) in v.iter().enumerate() {
            if let Some(dig) = c.to_digit(10) {
                left_dig = Some(dig);
                break;
            } else if let Some(dig) =
                matches_digit_word(&v[i..min(i + max_digit_word_len, v.len())])
            {
                left_dig = Some(dig as u32);
                break;
            }
        }
        let mut right_dig: Option<u32> = None;
        for (i, c) in v.iter().enumerate().rev() {
            if let Some(dig) = c.to_digit(10) {
                right_dig = Some(dig);
                break;
            } else if let Some(dig) =
                matches_digit_word(&v[i..min(i + max_digit_word_len, v.len())])
            {
                right_dig = Some(dig as u32);
                break;
            }
        }
        let num = left_dig.unwrap() * 10 + right_dig.unwrap();
        sum += num;

        line.clear();
    }
    return sum;
}

fn main() {
    let f = File::open("input.txt").expect("error reading input");
    let reader = BufReader::new(f);
    let sum = process_input_file(reader);
    println!("{}", sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn with_mix() {
        let input = vec!["1ootwooo", "ootwommmfour3oo"].join("\n");
        let result = process_input_file(BufReader::new(input.as_bytes()));

        assert_eq!(12 + 23, result);
    }

    #[test]
    fn with_words() {
        let input = vec!["onetwo", "twothree"].join("\n");
        let result = process_input_file(BufReader::new(input.as_bytes()));

        assert_eq!(12 + 23, result);
    }

    #[test]
    fn just_digits() {
        let input = vec!["1a0", "2b0"].join("\n");
        let result = process_input_file(BufReader::new(input.as_bytes()));

        assert_eq!(30, result);
    }
}
