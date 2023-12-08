use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn find_number(line: String, debug: bool) -> i32 {
    let mut first = -1;
    let mut last = -1;
    for c in line.chars() {
        if c.is_digit(10) {
            if first == -1 {
                first = c as i32 - 0x30;
            }
            last = c as i32 - 0x30;
        }
    }
    first * 10 + last
}

pub fn run(input_path: &String, debug: bool) -> Result<i32, Error> {
    let input = File::open(input_path)?;
    let buffered = BufReader::new(input);
    let mut sum = 0;
    for line in buffered.lines() {
        let num = find_number(line?, debug);
        sum += num;
    }

    Ok(sum)
}

fn parse_numbers(line: String, debug: bool) -> String {
    let mut line_out = line.clone();
    if debug {
        eprintln!("{}", line_out);
    }
    loop {
        let mut idx = usize::MAX;
        let mut to_replace = "";
        let mut with = "";
        if let Some(one) = line_out.find("one") {
            if one < idx {
                idx = one;
                to_replace = "one";
                with = "o1e";
            }
        }
        if let Some(two) = line_out.find("two") {
            if two < idx {
                idx = two;
                to_replace = "two";
                with = "t2o";
            }
        }
        if let Some(three) = line_out.find("three") {
            if three < idx {
                idx = three;
                to_replace = "three";
                with = "t3e";
            }
        }
        if let Some(four) = line_out.find("four") {
            if four < idx {
                idx = four;
                to_replace = "four";
                with = "f4r";
            }
        }
        if let Some(five) = line_out.find("five") {
            if five < idx {
                idx = five;
                to_replace = "five";
                with = "f5e";
            }
        }
        if let Some(six) = line_out.find("six") {
            if six < idx {
                idx = six;
                to_replace = "six";
                with = "s6x";
            }
        }
        if let Some(seven) = line_out.find("seven") {
            if seven < idx {
                idx = seven;
                to_replace = "seven";
                with = "s7n";
            }
        }
        if let Some(eight) = line_out.find("eight") {
            if eight < idx {
                idx = eight;
                to_replace = "eight";
                with = "e8t";
            }
        }
        if let Some(nine) = line_out.find("nine") {
            if nine < idx {
                to_replace = "nine";
                with = "n9e";
            }
        }

        if to_replace != "" {
            line_out = line_out.replacen(to_replace, with, 1);
            if debug {
                eprintln!("replaced {} with {}", to_replace, with);
                eprintln!("{}", line_out);
            }
        } else {
            break;
        }
    }

    line_out
}

pub fn run_second(input_path: &String, debug: bool) -> Result<i32, Error> {
    let input = File::open(input_path)?;
    let buffered = BufReader::new(input);
    let mut sum = 0;
    for line in buffered.lines() {
        let treated_line = parse_numbers(line?, debug);
        let num = find_number(treated_line, debug);
        if debug {
            eprintln!("{}", num);
        }
        sum += num;
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn first_half_example() {
        let result = run(&("./data/one_example.txt".to_string()), false);
        assert_eq!(result.unwrap(), 142);
    }

    #[test]
    fn second_half_example() {
        let result = run_second(&("./data/one2_example.txt".to_string()), false);
        assert_eq!(result.unwrap(), 281);
    }
}
