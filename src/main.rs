use zen_han_converter::{zen_to_han, han_to_zen};

fn main() {
    let zen_text = "ＨｅｌｌｏＷｏｒｌｄ！　１２３４５";
    let han_text = "ｺﾝﾆﾁﾊ､ｾｶｲ!";

    println!("全角 -> 半角: {}", zen_to_han(zen_text, true, true, true));
    println!("半角 -> 全角: {}", han_to_zen(han_text, true, true, true));
}
