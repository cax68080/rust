use std::{env,fs};

fn main(){
    //コマンドライン引数を得る
    let args = env::args();
    let mut total:f64 = 0.0;
    //すべてノンコマンドライン引数を処理
    for (i,fname) in args.enumerate(){
        if i == 0{continue;}
        //テキストファイルを読む
        let text = fs::read_to_string(fname).unwrap();
        //1行ごとに区切る
        let lines = text.split('\n');
        //くりかえし加算
        for line in lines{
            //数値に変換
            //println!("{}",line);
            let n:f64 = match line.parse(){
                Ok(v) => v,
                Err(_) => 0.0,
            };
            println!("{}",n);
            total += n;
        }
    }
    //結果を表示
    println!("{}",total);
}
