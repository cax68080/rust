//コマンドライン引数を表示するだけのプログラム
use std::env;

fn main(){
    let args = env::args();
    for arg in args{
        println!("{}",arg);
    }
}
