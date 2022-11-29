// スライスの各要素を加算する関数
fn sum_slice(items: &[i64]) -> i64{
    let mut total = 0;
    for i in items{
        total += i;
    }
    total   // 合計値を返す
}
fn main() {
    // 配列を初期化する。
    let a = [1,2,3,4,5,6,7,8,9,10];
    println!("a={}",sum_slice(&a[3..7]));
    // ベクターを初期化する
    let b = vec![1,2,3,4,5,6,7,8,9,10];
    println!("b={}",sum_slice(&b[2..4]))

}
