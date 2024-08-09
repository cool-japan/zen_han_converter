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

use std::collections::HashMap;
use lazy_static::lazy_static;

// Constants
const ASCII_ZENKAKU_CHARS: &[char] = &[
    '\u{FF41}', '\u{FF42}', '\u{FF43}', '\u{FF44}', '\u{FF45}', '\u{FF46}', '\u{FF47}', '\u{FF48}', '\u{FF49}', '\u{FF4A}', '\u{FF4B}', '\u{FF4C}', '\u{FF4D}', '\u{FF4E}', '\u{FF4F}',
    '\u{FF50}', '\u{FF51}', '\u{FF52}', '\u{FF53}', '\u{FF54}', '\u{FF55}', '\u{FF56}', '\u{FF57}', '\u{FF58}', '\u{FF59}', '\u{FF5A}', '\u{FF21}', '\u{FF22}', '\u{FF23}', '\u{FF24}',
    '\u{FF25}', '\u{FF26}', '\u{FF27}', '\u{FF28}', '\u{FF29}', '\u{FF2A}', '\u{FF2B}', '\u{FF2C}', '\u{FF2D}', '\u{FF2E}', '\u{FF2F}', '\u{FF30}', '\u{FF31}', '\u{FF32}', '\u{FF33}',
    '\u{FF34}', '\u{FF35}', '\u{FF36}', '\u{FF37}', '\u{FF38}', '\u{FF39}', '\u{FF3A}', '\u{FF01}', '\u{FF02}', '\u{FF03}', '\u{FF04}', '\u{FF05}', '\u{FF06}', '\u{FF07}', '\u{FF08}',
    '\u{FF09}', '\u{FF0A}', '\u{FF0B}', '\u{FF0C}', '\u{FF0D}', '\u{FF0E}', '\u{FF0F}', '\u{FF1A}', '\u{FF1B}', '\u{FF1C}', '\u{FF1D}', '\u{FF1E}', '\u{FF1F}', '\u{FF20}', '\u{FF3B}',
    '\u{FFE5}', '\u{FF3D}', '\u{FF3E}', '\u{FF3F}', '\u{FF40}', '\u{FF5B}', '\u{FF5C}', '\u{FF5D}', '\u{FF5E}', '\u{3000}', '\u{FF3C}',
];

const ASCII_HANKAKU_CHARS: &[char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
    'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
    'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y',
    'Z', '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':',
    ';', '<', '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~', ' ',
    '\\',
];

const KANA_ZENKAKU_CHARS: &[char] = &[
    '\u{30A2}', '\u{30A4}', '\u{30A6}', '\u{30A8}', '\u{30AA}', '\u{30AB}', '\u{30AD}', '\u{30AF}', '\u{30B1}', '\u{30B3}', '\u{30B5}', '\u{30B7}', '\u{30B9}', '\u{30BB}', '\u{30BD}',
    '\u{30BF}', '\u{30C1}', '\u{30C4}', '\u{30C6}', '\u{30C8}', '\u{30CA}', '\u{30CB}', '\u{30CC}', '\u{30CD}', '\u{30CE}', '\u{30CF}', '\u{30D2}', '\u{30D5}', '\u{30D8}', '\u{30DB}',
    '\u{30DE}', '\u{30DF}', '\u{30E0}', '\u{30E1}', '\u{30E2}', '\u{30E4}', '\u{30E6}', '\u{30E8}', '\u{30E9}', '\u{30EA}', '\u{30EB}', '\u{30EC}', '\u{30ED}', '\u{30EF}', '\u{30F2}',
    '\u{30F3}', '\u{30A1}', '\u{30A3}', '\u{30A5}', '\u{30A7}', '\u{30A9}', '\u{30C3}', '\u{30E3}', '\u{30E5}', '\u{30E7}', '\u{3002}', '\u{3001}', '\u{30FB}', '\u{309B}', '\u{309C}',
    '\u{300C}', '\u{300D}', '\u{30FC}',
];

const KANA_HANKAKU_CHARS: &[char] = &[
    '\u{FF71}', '\u{FF72}', '\u{FF73}', '\u{FF74}', '\u{FF75}', '\u{FF76}', '\u{FF77}', '\u{FF78}', '\u{FF79}', '\u{FF7A}', '\u{FF7B}', '\u{FF7C}', '\u{FF7D}', '\u{FF7E}', '\u{FF7F}',
    '\u{FF80}', '\u{FF81}', '\u{FF82}', '\u{FF83}', '\u{FF84}', '\u{FF85}', '\u{FF86}', '\u{FF87}', '\u{FF88}', '\u{FF89}', '\u{FF8A}', '\u{FF8B}', '\u{FF8C}', '\u{FF8D}', '\u{FF8E}',
    '\u{FF8F}', '\u{FF90}', '\u{FF91}', '\u{FF92}', '\u{FF93}', '\u{FF94}', '\u{FF95}', '\u{FF96}', '\u{FF97}', '\u{FF98}', '\u{FF99}', '\u{FF9A}', '\u{FF9B}', '\u{FF9C}', '\u{FF66}',
    '\u{FF9D}', '\u{FF67}', '\u{FF68}', '\u{FF69}', '\u{FF6A}', '\u{FF6B}', '\u{FF6F}', '\u{FF6C}', '\u{FF6D}', '\u{FF6E}', '\u{FF61}', '\u{FF64}', '\u{FF65}', '\u{FF9E}', '\u{FF9F}',
    '\u{FF62}', '\u{FF63}', '\u{FF70}',
];

const DIGIT_ZENKAKU_CHARS: &[char] = &['\u{FF10}', '\u{FF11}', '\u{FF12}', '\u{FF13}', '\u{FF14}', '\u{FF15}', '\u{FF16}', '\u{FF17}', '\u{FF18}', '\u{FF19}'];

const DIGIT_HANKAKU_CHARS: &[char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

const KANA_TEN_MAP: &[(char, char)] = &[
    ('\u{30AC}', '\u{FF76}'), ('\u{30AE}', '\u{FF77}'), ('\u{30B0}', '\u{FF78}'), ('\u{30B2}', '\u{FF79}'), ('\u{30B4}', '\u{FF7A}'),
    ('\u{30B6}', '\u{FF7B}'), ('\u{30B8}', '\u{FF7C}'), ('\u{30BA}', '\u{FF7D}'), ('\u{30BC}', '\u{FF7E}'), ('\u{30BE}', '\u{FF7F}'),
    ('\u{30C0}', '\u{FF80}'), ('\u{30C2}', '\u{FF81}'), ('\u{30C5}', '\u{FF82}'), ('\u{30C7}', '\u{FF83}'), ('\u{30C9}', '\u{FF84}'),
    ('\u{30D0}', '\u{FF8A}'), ('\u{30D3}', '\u{FF8B}'), ('\u{30D6}', '\u{FF8C}'), ('\u{30D9}', '\u{FF8D}'), ('\u{30DC}', '\u{FF8E}'),
    ('\u{30F4}', '\u{FF73}'),
];

const KANA_MARU_MAP: &[(char, char)] = &[
    ('\u{30D1}', '\u{FF8A}'), ('\u{30D4}', '\u{FF8B}'), ('\u{30D7}', '\u{FF8C}'), ('\u{30DA}', '\u{FF8D}'), ('\u{30DD}', '\u{FF8E}'),
];

lazy_static! {
    static ref ASCII_ZH_TABLE: HashMap<char, char> = {
        let mut m = HashMap::new();
        ASCII_ZENKAKU_CHARS.iter().zip(ASCII_HANKAKU_CHARS.iter()).for_each(|(&z, &h)| {
            m.insert(z, h);
        });
        m
    };

    static ref ASCII_HZ_TABLE: HashMap<char, char> = {
        let mut m = HashMap::new();
        ASCII_HANKAKU_CHARS.iter().zip(ASCII_ZENKAKU_CHARS.iter()).for_each(|(&h, &z)| {
            m.insert(h, z);
        });
        m
    };

    static ref KANA_ZH_TABLE: HashMap<char, char> = {
        let mut m = HashMap::new();
        KANA_ZENKAKU_CHARS.iter().zip(KANA_HANKAKU_CHARS.iter()).for_each(|(&z, &h)| {
            m.insert(z, h);
        });
        m
    };

    static ref KANA_HZ_TABLE: HashMap<char, char> = {
        let mut m = HashMap::new();
        KANA_HANKAKU_CHARS.iter().zip(KANA_ZENKAKU_CHARS.iter()).for_each(|(&h, &z)| {
            m.insert(h, z);
        });
        m
    };

    static ref DIGIT_ZH_TABLE: HashMap<char, char> = {
        let mut m = HashMap::new();
        DIGIT_ZENKAKU_CHARS.iter().zip(DIGIT_HANKAKU_CHARS.iter()).for_each(|(&z, &h)| {
            m.insert(z, h);
        });
        m
    };

    static ref DIGIT_HZ_TABLE: HashMap<char, char> = {
        let mut m = HashMap::new();
        DIGIT_HANKAKU_CHARS.iter().zip(DIGIT_ZENKAKU_CHARS.iter()).for_each(|(&h, &z)| {
            m.insert(h, z);
        });
        m
    };

    static ref KANA_TEN_ZH_TABLE: HashMap<char, char> = {
        let mut m = HashMap::new();
        KANA_TEN_MAP.iter().for_each(|&(z, h)| {
            m.insert(z, h);
        });
        m
    };

    static ref KANA_TEN_HZ_TABLE: HashMap<char, char> = {
        let mut m = HashMap::new();
        KANA_TEN_MAP.iter().for_each(|&(z, h)| {
            m.insert(h, z);
        });
        m
    };

    static ref KANA_MARU_ZH_TABLE: HashMap<char, char> = {
        let mut m = HashMap::new();
        KANA_MARU_MAP.iter().for_each(|&(z, h)| {
            m.insert(z, h);
        });
        m
    };

    static ref KANA_MARU_HZ_TABLE: HashMap<char, char> = {
        let mut m = HashMap::new();
        KANA_MARU_MAP.iter().for_each(|&(z, h)| {
            m.insert(h, z);
        });
        m
    };
}

// 変換関数
pub fn zen_to_han(text: &str, ascii: bool, digit: bool, kana: bool) -> String {
    let mut result = String::with_capacity(text.len() * 2);
    for c in text.chars() {
        if ascii && ASCII_ZH_TABLE.contains_key(&c) {
            result.push(*ASCII_ZH_TABLE.get(&c).unwrap());
        } else if digit && DIGIT_ZH_TABLE.contains_key(&c) {
            result.push(*DIGIT_ZH_TABLE.get(&c).unwrap());
        } else if kana && KANA_ZH_TABLE.contains_key(&c) {
            result.push(*KANA_ZH_TABLE.get(&c).unwrap());
        } else if kana && KANA_TEN_ZH_TABLE.contains_key(&c) {
            result.push(*KANA_TEN_ZH_TABLE.get(&c).unwrap());
            result.push('ﾞ');
        } else if kana && KANA_MARU_ZH_TABLE.contains_key(&c) {
            result.push(*KANA_MARU_ZH_TABLE.get(&c).unwrap());
            result.push('ﾟ');
        } else {
            result.push(c);
        }
    }
    result
}

pub fn han_to_zen(text: &str, ascii: bool, digit: bool, kana: bool) -> String {
    let mut result = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if ascii && ASCII_HZ_TABLE.contains_key(&c) {
            result.push(*ASCII_HZ_TABLE.get(&c).unwrap());
        } else if digit && DIGIT_HZ_TABLE.contains_key(&c) {
            result.push(*DIGIT_HZ_TABLE.get(&c).unwrap());
        } else if kana {
            let next = chars.peek().cloned();
            match (c, next) {
                (c, Some('\u{FF9E}')) if KANA_TEN_MAP.iter().any(|&(_, h)| h == c) => {
                    chars.next(); // Consume the dakuten
                    result.push(KANA_TEN_MAP.iter().find(|&&(_, h)| h == c).unwrap().0);
                },
                (c, Some('\u{FF9F}')) if KANA_MARU_MAP.iter().any(|&(_, h)| h == c) => {
                    chars.next(); // Consume the handakuten
                    result.push(KANA_MARU_MAP.iter().find(|&&(_, h)| h == c).unwrap().0);
                },
                (c, _) if KANA_HZ_TABLE.contains_key(&c) => {
                    result.push(KANA_HZ_TABLE[&c]);
                },
                _ => result.push(c),
            }
        } else {
            result.push(c);
        }
    }
    result
}
