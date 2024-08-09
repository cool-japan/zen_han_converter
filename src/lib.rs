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
    'ａ', 'ｂ', 'ｃ', 'ｄ', 'ｅ', 'ｆ', 'ｇ', 'ｈ', 'ｉ', 'ｊ', 'ｋ', 'ｌ', 'ｍ', 'ｎ', 'ｏ',
    'ｐ', 'ｑ', 'ｒ', 'ｓ', 'ｔ', 'ｕ', 'ｖ', 'ｗ', 'ｘ', 'ｙ', 'ｚ', 'Ａ', 'Ｂ', 'Ｃ', 'Ｄ',
    'Ｅ', 'Ｆ', 'Ｇ', 'Ｈ', 'Ｉ', 'Ｊ', 'Ｋ', 'Ｌ', 'Ｍ', 'Ｎ', 'Ｏ', 'Ｐ', 'Ｑ', 'Ｒ', 'Ｓ',
    'Ｔ', 'Ｕ', 'Ｖ', 'Ｗ', 'Ｘ', 'Ｙ', 'Ｚ', '！', '"', '＃', '＄', '％', '＆', '’', '（',
    '）', '＊', '＋', '，', '－', '．', '／', '：', '；', '＜', '＝', '＞', '？', '＠', '［',
    '￥', '］', '＾', '＿', '`', '｛', '｜', '｝', '～', '　', '＼',
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
    'ア', 'イ', 'ウ', 'エ', 'オ', 'カ', 'キ', 'ク', 'ケ', 'コ', 'サ', 'シ', 'ス', 'セ', 'ソ',
    'タ', 'チ', 'ツ', 'テ', 'ト', 'ナ', 'ニ', 'ヌ', 'ネ', 'ノ', 'ハ', 'ヒ', 'フ', 'ヘ', 'ホ',
    'マ', 'ミ', 'ム', 'メ', 'モ', 'ヤ', 'ユ', 'ヨ', 'ラ', 'リ', 'ル', 'レ', 'ロ', 'ワ', 'ヲ',
    'ン', 'ァ', 'ィ', 'ゥ', 'ェ', 'ォ', 'ッ', 'ャ', 'ュ', 'ョ', '。', '、', '・', '゛', '゜',
    '「', '」', 'ー',
];

const KANA_HANKAKU_CHARS: &[char] = &[
    'ｱ', 'ｲ', 'ｳ', 'ｴ', 'ｵ', 'ｶ', 'ｷ', 'ｸ', 'ｹ', 'ｺ', 'ｻ', 'ｼ', 'ｽ', 'ｾ', 'ｿ', 'ﾀ', 'ﾁ',
    'ﾂ', 'ﾃ', 'ﾄ', 'ﾅ', 'ﾆ', 'ﾇ', 'ﾈ', 'ﾉ', 'ﾊ', 'ﾋ', 'ﾌ', 'ﾍ', 'ﾎ', 'ﾏ', 'ﾐ', 'ﾑ', 'ﾒ',
    'ﾓ', 'ﾔ', 'ﾕ', 'ﾖ', 'ﾗ', 'ﾘ', 'ﾙ', 'ﾚ', 'ﾛ', 'ﾜ', 'ｦ', 'ﾝ', 'ｧ', 'ｨ', 'ｩ', 'ｪ', 'ｫ',
    'ｯ', 'ｬ', 'ｭ', 'ｮ', '｡', '､', '･', 'ﾞ', 'ﾟ', '｢', '｣', 'ｰ',
];

const DIGIT_ZENKAKU_CHARS: &[char] = &['０', '１', '２', '３', '４', '５', '６', '７', '８', '９'];

const DIGIT_HANKAKU_CHARS: &[char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

const KANA_TEN_MAP: &[(char, char)] = &[
    ('ガ', 'ｶ'), ('ギ', 'ｷ'), ('グ', 'ｸ'), ('ゲ', 'ｹ'), ('ゴ', 'ｺ'),
    ('ザ', 'ｻ'), ('ジ', 'ｼ'), ('ズ', 'ｽ'), ('ゼ', 'ｾ'), ('ゾ', 'ｿ'),
    ('ダ', 'ﾀ'), ('ヂ', 'ﾁ'), ('ヅ', 'ﾂ'), ('デ', 'ﾃ'), ('ド', 'ﾄ'),
    ('バ', 'ﾊ'), ('ビ', 'ﾋ'), ('ブ', 'ﾌ'), ('ベ', 'ﾍ'), ('ボ', 'ﾎ'),
    ('ヴ', 'ｳ'),
];

const KANA_MARU_MAP: &[(char, char)] = &[
    ('パ', 'ﾊ'), ('ピ', 'ﾋ'), ('プ', 'ﾌ'), ('ペ', 'ﾍ'), ('ポ', 'ﾎ'),
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
                (c, Some('ﾞ')) if KANA_TEN_MAP.iter().any(|&(_, h)| h == c) => {
                    chars.next(); // Consume the dakuten
                    result.push(KANA_TEN_MAP.iter().find(|&&(_, h)| h == c).unwrap().0);
                },
                (c, Some('ﾟ')) if KANA_MARU_MAP.iter().any(|&(_, h)| h == c) => {
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
