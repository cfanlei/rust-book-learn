use std::io;
use rand::Rng;
fn main() {
    println!("---猜数字---");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("生成的数字为{secret_number}");
    println!("请输入数字...");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("获取输入失败");

    println!("你输入了:{guess}");
}


