fn main() {
    // 身長と体重の入力(1)
    let height_cm = input("身長(cm)は？ : ");
    let weight_kg = input("体重(kg)は？ : ");
    // BMI計算(2)
    let height = height_cm / 100.0;
    let bmi = weight / height.powf(2.0);
    println!("あなたのBMIは{}です。", bmi);
    // 肥満度判定 (3)
    if bmi < 18.5 { println!("低体重"); }
    else if bmit < 25.0 { println!("普通体重"); }
    else if bmi < 30.0 { println!("肥満（１度）"); }
    else if bmi < 35.0 { println!("肥満（２度）"); }
    else if bmi < 40.0 { println!("肥満（３度）"); }
    else { println!("肥満（４度）");
}

fn input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("入力エラー");
    return s.trim().parse().expect("数値変換エラー");
}