// fn main() {
//     println!("Hello, world!");
//     let x = test_fn(false).unwrap_or_else(|| 8);
//     println!("x: {}", x);

//     let mut v1 = vec![S{
//         x: 1,
//         y: 2,
//     },S{
//         x: 8,
//         y: 9,
//     },S{
//         x: 3,
//         y: 4,
//     }];
//     let mut p1 = || v1.push(S { x: 4, y: 10 });
//     // println!("v1: {v1:?}");
//     p1();
//     let mut v2: Vec<String> = Vec::new();
//     let str = String::from("hello");
//     println!("v1: {v1:?}");
//     // v1.sort_by_key(|i| i.x);
//     v1.sort_by_key(|i| {
//         v2.push(str.clone());
//         i.x
//     });
//     println!("v1: {v1:?}");
//     println!("v2: {v2:?}");
// }

// #[derive(Debug)]
// struct  S {
//     x: i8,
//     y: i8,
// }

// fn test_fn(x: bool) -> Option<i8>{
//     if x {
//         Some(1)
//     } else {
//         None
//     }
// }

use core::slice; 
fn main() {
    let v1 = vec![2,3,4];
    let i1 = v1.iter();
    for v in i1 {
        println!("for {v:?}");
    }
    // for v in i1 {
    //     println!("for {v:?}");
    // }

    let v2 = vec![4,5,6];
    let mut i2: slice::Iter<i32> = v2.iter();
    println!("{:?}", i2.next().unwrap_or_else(||&9));
    println!("{:?}", i2.next().unwrap_or_else(||&9));
    println!("{:?}", i2.next().unwrap_or_else(||&9));
    println!("{:?}", i2.next().unwrap_or_else(||&9));

    let v3 = vec![4,5,6];
    let mut i3: slice::Iter<i32> = v3.iter(); 

    let i4: Vec<&i32>= i3.map(|x| {
        println!("x is {x}");
        x
    }).collect();


}