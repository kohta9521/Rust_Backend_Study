pub fn run() {
    // stack overflow
    // let _a1: [u8; 1000000000] = [1; 1000000000];

    // vector
    let mut v1 = vec![1, 2, 3, 4];
    let mut v2 = vec![5, 6, 7, 8];
    let mut v3 = vec![9, 10];
    println!("Stack address of v1 is: {:p}", &v1);
    println!("Stack address of v2 is: {:p}", &v2);
}