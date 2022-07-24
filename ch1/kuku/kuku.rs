//Rustで九九の表を作成する
fn main(){
    for y in 1 .. 10{
        for x in 1 .. 10{
            if x < 9{
                print!("{:3},",x * y);
            } else {
                print!("{:3}",x * y);
            }
            
        }
        println!("");
    }
}