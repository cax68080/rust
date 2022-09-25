
fn main() {
    let mut g1 = String::from("過ちを見過ごす人は美しい。");
    g1 = show_message(g1);   //所有権が移動する
    println!("{}",g1);  //ok
}
fn show_message(message: String) -> String{
    println!("{}",message);
    return String::from("終了しました");
}
