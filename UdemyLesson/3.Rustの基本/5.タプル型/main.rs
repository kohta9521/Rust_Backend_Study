fn main() {
    let t1: (i32, bool, f64) = (1, true, 2.0);

    println!("{:?}", t1 );

    // タプルから値を取り出す
    let i = t1.0;
    println!("{}", i);

    // タプルから一気に取り出す
    let (x: f64, y: bool, _) = t1;
    println("{}, {}", x, y);

    // let u: () = ();
}