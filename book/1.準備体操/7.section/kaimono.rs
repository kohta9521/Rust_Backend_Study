fn main() {
    let pc_price = 98000.0;
    let a = 1200.0;
    let a_rate = 0.8;
    let b = 0.0;
    let b_rate = 0.9;

    println!("A社={}円", pc_price * a_rate + a);
    println!("B社={}円", pc_price * b_rate + b);
}