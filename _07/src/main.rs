// fn main() {
//     for index in 1..12 {
//         println!("{index}");
//     }
//     println!("Hello, world!");
// }

use _07::A;

fn main() {
    // _07::parent::child1::log();
    _07::parent::log();
    let a = A::create_a(String::from("value"));
    println!("{}", a.name);
}