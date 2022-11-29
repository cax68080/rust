// clapで定義されているclap::Appとclap::Argの
// 2つの型をスコープに入れる
use clap::{App,Arg};
fn main() {
    // clap::Appでコマンド名やバージョンなどを設定
    let arg_matches = App::new("trip-Analyzer")
        .version("1.0")
        .about("Analyze yellow cab trip records")
        // INFILEという名前のコマンドライン引数を登録
        .arg(Arg::with_name("INFILE")
            .help("Sets the input CSV file")
            .index(1)
            .required(true)
        )
        // get_matches()メソッドを呼ぶとユーザーが与えた
        // コマンドライン引数がパースされる
        .get_matches();
    let infile = arg_matches.value_of("INFILE").unwrap();
    // INFILEの文字列を表示。"{:?}"はデバッグ用文字列を表示
    println!("INFILE: {}",infile);
}
