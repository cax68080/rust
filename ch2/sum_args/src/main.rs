fn main() {
    //コマンドライン引数を取得する
    let args = std::env::args();
    let mut total = 0.0;
    //コマンドライン引数を順に加算する
    for (i,s) in args.enumerate(){
        //0番目はコマンド自身なので飛ばす
        if i == 0{continue;}
        //コマンドライン引数を数値に変換する
        let num: f64 = match s.parse(){
            Ok(v) => v,
            Err(_) => 0.0,
        };
        total += num;
    }
    //結果を表示する
    println!("{}",total);
}
