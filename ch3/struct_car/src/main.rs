// CarSpec構造体を定義する
struct CarSpec{
    model:i32,
    cc:i32,
    color:i32,
}

fn main() {
    // CarSpecオブジェクトを生成する
    let car1 = CarSpec{
        model:3001,
        cc:1500,
        color:0xFF0000,
    };
    let car2 = CarSpec{
        model:3002,
        cc:1200,
        color:0x0000FF,
    };
    // オブジェクトのフィールドを表示する
    println!("car1:{},{}cc,{:06x}",car1.model,car1.cc,car1.color);
    println!("car2:{},{}cc,{:06x}",car2.model,car2.cc,car2.color);
}
