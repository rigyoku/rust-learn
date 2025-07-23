// use std::fmt::Debug;

// #[derive(Debug)]
// enum Type<T> {
//     Name(T),
// }

// struct People<T> {
//     something: T,
// }

// trait Log {
//     fn log(&self) {
//         println!("Logging something");
//     }
// }

// impl<T: Debug> People<T> {
//     fn speak<U: Debug>(&self, append: &U) {
//         println!("speak something {:?} append {:?}", self.something, append);
//     }
// }

// impl <T> Log for People<T> {}

// fn main() {
//     println!("Hello, world!");
//     let p = People {
//         something: Type::Name("hello"),
//     };
//     p.speak(&"world");
//     p.log();
// }

// fn main() {
//     let a;
//     {
//         let b = 10;
//         a = &b;
//     }
//     // println!("a: {}", a); // 这会导致悬垂引用, 因为 b 已经被销毁了
// }

fn compare<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

struct A<'a> {
    name: &'a str,
}

impl<'a> A<'a> {
    fn log() {
        println!("Logging A");
    }
}

fn main() {
    let result = compare("123", "45");
    println!("Comparison result: {}", result);
    let a = A { name: "hello" };
    println!("A name: {}", a.name);
    // {
    //    let s: &'static String = &String::from("Hello, world!");
    // }
    // println!("s name: {}", s);
}
