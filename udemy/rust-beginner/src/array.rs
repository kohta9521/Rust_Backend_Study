fn main() {
    let l1: [i32; 3] = [1, 2, 3];
    let l2: [i32; 1000] = [0; 1000];

    println!("{:?}", l1);
    let i: i32 = l1[0];
    println!("{}", i);

    let [ x, y, z ] = li;
    let l3 = &l1[0..2];
}