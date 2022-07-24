//3の倍数と3のつく数字にAと表示する。
fn main(){
    for i in 1 .. 51{
        if i % 3 == 0 {
            println!("A");
        } else if i % 10 == 3 {
            println!("A");
        } else if i / 10 == 3{
            println!("A")
        } else {
            println!("{}",i)
        }
    }
}