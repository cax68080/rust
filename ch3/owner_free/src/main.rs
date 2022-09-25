fn main() {
    //ブロック
    {
        let s1 = String::from("聞かないで返事をする人は愚か");
        let s2 = 1000;
        println!("{}",s1);
        println!("{}",s2);
    }
    //ブロックを抜けるとs1の値は破棄される
    //println!("{}",s2);
}
