// ファイル: tests/test.rs

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
    assert_eq!(han_to_zen("ﾊﾟﾋﾟﾌﾟﾍﾟﾎﾟ", false, false, true), "パピプペポ");
}
