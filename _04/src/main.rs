// fn main() {
//     let s = "const";
//     // s.push_str("append");
//     println!("const: {s}");

//     let mut s = String::from("let");
//     s.push_str(" / append");
//     println!("let: {s}");

//     let s = "-";
//     let s2 = s;
//     println!("s: {s} s2: {s2}");

//     let s = String::from("-");
//     let s2 = s.clone();
//     println!("s: {s} s2: {s2}");

//     let s = String::from("-");
//     func(s);
//     // println!("s: {s}");

//     let mut s = String::from("-");
//     func2(&mut s);
//     func2(&mut s);
//     println!("func2: {s}");

//     let mut s = String::from("-");
//     let s2= &s;
//     let s3= &s;
//     // let s4= &mut s;
//     // let s5= &s;
//     // let s5= &mut s;
//     // func2(s4);
//     func1(s2);
    
//     // println!("s: {s} s2: {s2} s3: {s3} s4: {s4}");

//     let s = String::from("12 3");
//     let index = f3(&s);
//     println!("index - {index}");

//     let mut str = String::from("7890");
//     // str.clear();
//     let s = &str[..2];
//     println!("s: {s}");

//     let mut str = String::from("7890");
//     let res: &str = f4(&str);
//     // str.clear();
//     println!("str: {str}");
//     println!("res: {res}");

//     let mut s1 = "s1";
//     let s2 = String::from("s2");
//     let s3 = &s2;
//     let r1 = f5(s1);
//     let r2 = f5(&s2);
//     let r3 = f5(&mut s1);
//     let r4 = f5(&s1[..]);

//     let arr: [i32; 4] = [1,2,3,4];
//     let a1 = &arr;
//     let a2: &[i32] = &arr[..2];


//     println!("Hello, world!");
// }


// fn func(s: String) {
//     println!("s: {s}");
// }


// fn func1(s: &String) {
//     println!("s: {s}");
// }

// fn func2(s: &mut String) {
//     s.push_str("-append");
//     println!("s: {s}");
// }

// fn f3(target: &String) -> usize {
//     for (index, &item) in target.as_bytes().iter().enumerate() {
//         println!("item: {item} index: {index}");
//         if item == b' ' {
//             return index;
//         }
//     }
//     target.len()
// }



// fn f4(target: &String) -> &str {
//     for (index, &item) in target.as_bytes().iter().enumerate() {
//         println!("item: {item} index: {index}");
//         if item == b' ' {
//             return &target[..index];
//         }
//     }
//     &target[..]
// }


// fn f5(target: &str) -> &str {
//     &target[..]
// }

fn main() {
    println!("main ==>");
    let a: String = String::from("value");
    let res = slice(&a);
    println!("{a} {res}");
}

fn slice(a: &str) -> String {
    let mut str = a.to_string();
    str.push_str("_pushed");
    str
}