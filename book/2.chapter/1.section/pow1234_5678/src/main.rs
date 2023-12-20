// BigIngを使うための宣言
use num_bigint::BigInt;

fn main() {
    let v = BigInt::from(1234);
    println!("v = {}", v.pow(5678));
}
