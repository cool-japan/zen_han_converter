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

use zen_han_converter::{zen_to_han, han_to_zen};

#[test]
fn test_zen_to_han_ascii() {
    assert_eq!(zen_to_han("ＡＢＣ", true, false, false), "ABC");
    assert_eq!(zen_to_han("ＡＢＣ", false, false, false), "ＡＢＣ");
}

#[test]
fn test_zen_to_han_digit() {
    assert_eq!(zen_to_han("１２３", false, true, false), "123");
    assert_eq!(zen_to_han("１２３", false, false, false), "１２３");
}

#[test]
fn test_zen_to_han_kana() {
    assert_eq!(zen_to_han("アイウ", false, false, true), "ｱｲｳ");
    assert_eq!(zen_to_han("アイウ", false, false, false), "アイウ");
}

#[test]
fn test_zen_to_han_mixed() {
    assert_eq!(zen_to_han("ＡＢＣ１２３アイウ", true, true, true), "ABC123ｱｲｳ");
}

#[test]
fn test_han_to_zen_ascii() {
    assert_eq!(han_to_zen("ABC", true, false, false), "ＡＢＣ");
    assert_eq!(han_to_zen("ABC", false, false, false), "ABC");
}

#[test]
fn test_han_to_zen_digit() {
    assert_eq!(han_to_zen("123", false, true, false), "１２３");
    assert_eq!(han_to_zen("123", false, false, false), "123");
}

#[test]
fn test_han_to_zen_kana() {
    assert_eq!(han_to_zen("ｱｲｳ", false, false, true), "アイウ");
    assert_eq!(han_to_zen("ｱｲｳ", false, false, false), "ｱｲｳ");
}

#[test]
fn test_han_to_zen_mixed() {
    assert_eq!(han_to_zen("ABC123ｱｲｳ", true, true, true), "ＡＢＣ１２３アイウ");
}

#[test]
fn test_han_to_zen_dakuten() {
    assert_eq!(han_to_zen("ｶﾞｷﾞｸﾞｹﾞｺﾞ", false, false, true), "ガギグゲゴ");
}

#[test]
fn test_han_to_zen_handakuten() {
    assert_eq!(han_to_zen("ﾊﾟﾋﾟﾌﾟﾍﾟﾎﾟﾋﾟｬﾋﾟｭﾋﾟｮ", false, false, true), "パピプペポピャピュピョ");
}
