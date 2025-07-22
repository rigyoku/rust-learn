use std::cmp::Ordering;

use std::io;
use rand::Rng;

fn main() {
    println!("开始游戏");
    let secret = rand::thread_rng().gen_range(1..=100);
    // println!("随机数字是: {secret}");
    loop {
        println!("请输入数字");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("error");
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let prefix: &str = "输入的是";
        println!("{}: {input}", prefix);
        match input.cmp(&secret) {
            Ordering::Less => println!("猜小了"),
            Ordering::Greater => println!("猜大了"),
            Ordering::Equal => {
                println!("猜对了");
                break;
            }
        }
    }
}
