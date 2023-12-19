fn main() {
    let l1: [i32, 3] = [1, 2, 3];
    println!("{:?}", l1);

    // let i: i32 = li[0];

    // let [x: i32, y: i32, _] = l1;

    let l3: &[i32] = &l1[0..2];
    println!("{:?}", l3);
}