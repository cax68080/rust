//エラーになる
fn main() {
    let s = "気前よく与えて豊かになる人がいる。".to_string();
    echo(s);
    println!("{}",s);
}
//println!を模して作った関数
fn echo(s: String){
    println!("{}",s);
}
