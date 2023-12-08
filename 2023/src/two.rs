use std::fs::File;
use std::io::{BufRead, BufReader, Error};

pub fn run(input_path: &String, debug: bool) -> Result<usize, Error> {
    const N_RED: usize = 12;
    const N_GREEN: usize = 13;
    const N_BLUE: usize = 14;
    let input = File::open(input_path)?;
    let buffered = BufReader::new(input);
    let mut sum: usize = 0;
    for line in buffered.lines() {
        let l = line?;
        let parts: Vec<&str> = l.split(":").collect();
        let game: usize = parts[0].replace("Game ", "").trim().parse().unwrap();
        let informations: Vec<&str> = parts[1].split(";").collect();
        if debug {
            eprintln!("game #{}", game)
        }
        let mut possible = true;
        for info in informations {
            let per_color_info: Vec<&str> = info.split(",").collect();
            for i in per_color_info {
                if i.contains("red") {
                    let nreds: usize = i.replace(" red", "").trim().parse().unwrap();
                    if debug {
                        eprintln!("reds {}", nreds);
                    }
                    if nreds > N_RED {
                        possible = false;
                        break;
                    }
                } else if i.contains("green") {
                    let ngreens: usize = i.replace(" green", "").trim().parse().unwrap();
                    if debug {
                        eprintln!("greens {}", ngreens);
                    }
                    if ngreens > N_GREEN {
                        possible = false;
                        break;
                    }
                } else if i.contains("blue") {
                    let nblues: usize = i.replace(" blue", "").trim().parse().unwrap();
                    if debug {
                        eprintln!("blues: {}", nblues);
                    }
                    if nblues > N_BLUE {
                        possible = false;
                        break;
                    }
                }
            }
        }
        if possible {
            sum += game;
            if debug {
                eprintln!("game #{} possible", game)
            }
        } else {
            if debug {
                eprintln!("game #{} not possible", game)
            }
        }
    }
    Ok(sum)
}

pub fn run_second(input_path: &String, debug: bool) -> Result<usize, Error> {
    let input = File::open(input_path)?;
    let buffered = BufReader::new(input);
    let mut sum: usize = 0;
    for line in buffered.lines() {
        let l = line?;
        let parts: Vec<&str> = l.split(":").collect();
        let game: usize = parts[0].replace("Game ", "").trim().parse().unwrap();
        let informations: Vec<&str> = parts[1].split(";").collect();
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for info in informations {
            let per_color_info: Vec<&str> = info.split(",").collect();
            for i in per_color_info {
                if i.contains("red") {
                    let nreds: usize = i.replace(" red", "").trim().parse().unwrap();
                    if debug {
                        eprintln!("reds {}", nreds);
                    }
                    if nreds > max_red {
                        max_red = nreds;
                    }
                } else if i.contains("green") {
                    let ngreens: usize = i.replace(" green", "").trim().parse().unwrap();
                    if debug {
                        eprintln!("greens {}", ngreens);
                    }
                    if ngreens > max_green {
                        max_green = ngreens;
                    }
                } else if i.contains("blue") {
                    let nblues: usize = i.replace(" blue", "").trim().parse().unwrap();
                    if debug {
                        eprintln!("blues: {}", nblues);
                    }
                    if nblues > max_blue {
                        max_blue = nblues;
                    }
                }
            }
        }
        if debug {
            eprintln!(
                "game #{} r: {}, g: {}, b: {}",
                game, max_red, max_green, max_blue
            );
        }
        sum += max_red * max_green * max_blue;
    }
    Ok(sum)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_half_example() {
        let result = run(&("./data/two_example.txt".to_string()), false);
        assert_eq!(result.unwrap(), 8)
    }

    #[test]
    fn second_half_example() {
        let result = run_second(&("./data/two2_example.txt".to_string()), false);
        assert_eq!(result.unwrap(), 2286)
    }
}
