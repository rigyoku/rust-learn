// enum People {
//     Liy(i8),
//     Nana{

//     }
// }

// fn main() {
//     println!("Hello, world!");
//     let mut v1: Vec<i8> = Vec::new();
//     v1.push(2);
//     v1.push(8);
//     println!("v {} {}", v1[0], if let Option::Some(i) = v1.get(1) {
//         i
//     } else {&0});
    
//     *&mut v1[1] = 12;

//     println!("v {} {}", v1[0], if let Option::Some(i) = v1.get(1) {
//         i
//     } else {&0});

//     let v = v1.pop();
//     match v {
//         Some(i) => println!("{i}"),
//         None => println!("none")
//     }
//     let l = v1.len();
//     println!("{l}");
    
// }


// fn main() {
//     let a = "1";
//     let mut s = String::from(a);
//     s.push_str(a);
//     println!("{a}");
//     println!("{s}");
//     s.push('c');
//     println!("{s}");
//     let b = a.to_string() + &s;
//     println!("{b}");
//     let c = b + "123";
//     println!("{c}");
//     let d = format!("{s} == {c}");
//     println!("{s}");
//     println!("{c}");
//     println!("{d}");
//     let b = d.contains("=");
//     println!("{b}");
//     let b = d.contains("g");
//     println!("{b}");
//     let b = d.replace("==", "x");
//     println!("{b}");
//     println!("{d}");
// }

#[derive(Debug)]
enum Type {
    T1,
    T2,
}

use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, crate::Type::T1);
    map.insert(2, crate::Type::T2);
    let item = map.get(&1);
    if let Some(i) = item {
        println!("{:?}", i);
    } else {
        println!("None");
    }
    for (k, v) in &map {
        println!("k {k} v {:?}", v);
    }
    let map = test();
    println!("{}", map.get(&1).unwrap_or(& "".to_string()));
}

fn test() -> HashMap<i32, String> {
    let mut map = HashMap::new();
    let v = String::from("value");
    let v2 = v.to_string();
    map.insert(1, v);
    println!("{}", map.get(&1).unwrap_or(& "".to_string()));
    // map.insert(1, v2 + "hahaha");
    // map.entry(1).or_insert(v2 + "hahaha");
    * map.entry(1).or_insert(v2 + "hahaha") = "a?".to_string();
    println!("{}", map.get(&1).unwrap_or(& "".to_string()));

    map
}