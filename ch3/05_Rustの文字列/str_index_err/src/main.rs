// 問題のあるプログラム
fn main() {
    let s = "こんにちは";
    // 先頭の１文字を取り出して表示
    let ch = s.chars().nth(0).unwrap();
    println!("{}",ch);  //こ
    // (０から数えて)２文字目を取り出して表示
    let ch = s.chars().nth(2).unwrap();
    println!("{}",ch);  //に
}
