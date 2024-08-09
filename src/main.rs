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

fn main() {
    let zen_text = "ＨｅｌｌｏＷｏｒｌｄ！　１２３４５";
    let han_text = "ｺﾝﾆﾁﾊ､ｾｶｲ!";

    println!("全角 -> 半角: {}", zen_to_han(zen_text, true, true, true));
    println!("半角 -> 全角: {}", han_to_zen(han_text, true, true, true));
}
