mod one;
mod two;

use clap::Parser;

#[derive(Parser)] // requires `derive` feature
#[command(name = "aoc")]
#[command(bin_name = "aoc")]
enum AocCli {
    Day(AocDayArgs),
}

#[derive(clap::Args)]
#[command(author, version, about, long_about = None)]
struct AocDayArgs {
    #[arg(long, default_value_t = 1)]
    day: u8,

    #[arg(long, default_value_t = false)]
    second_half: bool,

    #[arg(long)]
    input_path: Option<std::path::PathBuf>,

    #[arg(long, default_value_t = false)]
    debug: bool,
}

fn main() {
    let AocCli::Day(args) = AocCli::parse();
    if args.debug {
        eprintln!("{} {:?}", args.day, args.input_path);
    }
    match args.day {
        1 => {
            let mut input = String::from("./data/one.txt".to_string());
            if let Some(path) = args.input_path {
                input = String::from(path.as_path().display().to_string());
            }
            if args.second_half {
                let sum = one::run_second(&input, args.debug).unwrap();
                println!("one second half: {}", sum);
            } else {
                let sum = one::run(&input, args.debug).unwrap();
                println!("one first half: {}", sum);
            }
        }
        2 => {
            let mut input = String::from("./data/two.txt".to_string());
            if let Some(path) = args.input_path {
                input = String::from(path.as_path().display().to_string());
            }
            if args.second_half {
                let sum = two::run_second(&input, args.debug).unwrap();
                println!("two second half: {}", sum);
            } else {
                let sum = two::run(&input, args.debug).unwrap();
                println!("two first half: {}", sum);
            }
        }
        _ => {
            println!("not implemented")
        }
    }
}
