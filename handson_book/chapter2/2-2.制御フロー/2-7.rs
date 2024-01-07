fn main() {
    let num = 123;
    if num % 5 == 0 {
        println!("{}は、5で割れます。", num);
    } else if num % 4 == 0 {
        println!("{}は、4で割れます。", num);
    } else if num % 3 == 0 {
        println!("{}は、3で割れます。", num);
    } else if num % 2 == 0 {
        println!("{}は、2で割れます。", num);
    } else {
        println!("{}は、割り切れません。", num);
    }
}