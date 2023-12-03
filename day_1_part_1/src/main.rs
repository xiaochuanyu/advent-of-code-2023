use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn process_input_file<R: Read>(mut reader: BufReader<R>) -> u16 {
    let mut line = String::new();
    let mut sum = 0;
    loop {
        let len = reader.read_line(&mut line).expect("error reading line");
        if len == 0 {
            break;
        }
        let v: Vec<char> = line.chars().collect();
        let leftmost = v
            .iter()
            .find(|e| e.is_digit(10))
            .expect("leftmost digit not found");
        let rightmost = v
            .iter()
            .rfind(|e| e.is_digit(10))
            .expect("rightmost digit not found");
        let combined = format!("{}{}", leftmost, rightmost);
        let num: u16 = combined.parse().unwrap();
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
    fn works() {
        let input = vec!["1a0", "2b0"].join("\n");
        let result = process_input_file(BufReader::new(input.as_bytes()));

        assert_eq!(30, result);
    }
}
