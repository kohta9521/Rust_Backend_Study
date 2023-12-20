fn main() {
    for i in 1..51 {
        if i % 3 == 0 {
            println!("A");
        } else if i == 13 || i == 23 || i == 43 {
            println!("A");
        } else if 30 <= i && i <= 39 {
            println!("A");
        } else {
            println!("{}", i);
        }
    }
}