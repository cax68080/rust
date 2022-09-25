//借用と参照
fn main() {
    let g1 = String::from("過ちを見過ごす人は美しい。");
    show_message(&g1);   //参照を渡す
    println!("{}",&g1);  //所有権は移動しない
}
fn show_message(message: &String){
    println!("{}",message);
}
