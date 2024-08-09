// Copyright 2024 KitaSan <info@kitasan.io>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::env;
use std::io::{self, Read};
use zen_han_converter::han_to_zen;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let args: Vec<String> = env::args().collect();
    let (ascii, digit, kana) = parse_args(&args);

    let output = han_to_zen(&input, ascii, digit, kana);
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

