use std::env;
use std::io::{self, Read};
use zen_han_converter::zen_to_han;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let args: Vec<String> = env::args().collect();
    let (ascii, digit, kana) = parse_args(&args);

    let output = zen_to_han(&input, ascii, digit, kana);
    print!("{}", output);

    Ok(())
}

fn parse_args(args: &[String]) -> (bool, bool, bool) {
    let mut ascii = true;
    let mut digit = true;
    let mut kana = true;

    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "-a" => ascii = false,
            "-d" => digit = false,
            "-k" => kana = false,
            _ => {}
        }
    }

    (ascii, digit, kana)
}
