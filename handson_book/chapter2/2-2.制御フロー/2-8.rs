fn main() {
    let num:u8 = 7;
    match num {
        1 => println!("{}月は、正月です。", num),
        2 => println!("{}月は、節分の月です。", num),
        3 => println!("{}月は、ひな祭りの月です。", num),
        4 => println!("{}月は、入学式がある月です。", num),
        5 => println!("{}月といえば、ゴールンウィークです。", num),
        _ => println!("{}月はそれ以外の月です。", num),
    }
}