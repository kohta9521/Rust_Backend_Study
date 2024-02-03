fn main() {
    println!("Hello, world!");
    println!("Hello, {}", "students");
    // １行のコメント
    /*
        複数行のコメント
    */

    // 変数宣言
    let x: i32 = 10;
    println!("{}", x);
    // x = 20;

    // 定数
    // const A: i32 = 1;


    let f: f64 = 1 as f64 + 2.0;
    println!("{}", f);

    let t1: (i32, bool, f64) = (1, true, 1.0);
    println!("{:?}", t1);

    let i = t1.0;
    println!("{}", i);
}
