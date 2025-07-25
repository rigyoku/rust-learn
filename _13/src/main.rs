fn main() {
    println!("Hello, world!");
    let x = test_fn(false).unwrap_or_else(|| 8);
    println!("x: {}", x);

    let mut v1 = vec![S{
        x: 1,
        y: 2,
    },S{
        x: 8,
        y: 9,
    },S{
        x: 3,
        y: 4,
    }];
    let mut p1 = || v1.push(S { x: 4, y: 10 });
    // println!("v1: {v1:?}");
    p1();
    let mut v2: Vec<String> = Vec::new();
    let str = String::from("hello");
    println!("v1: {v1:?}");
    // v1.sort_by_key(|i| i.x);
    v1.sort_by_key(|i| {
        v2.push(str.clone());
        i.x
    });
    println!("v1: {v1:?}");
    println!("v2: {v2:?}");
}

#[derive(Debug)]
struct  S {
    x: i8,
    y: i8,
}

fn test_fn(x: bool) -> Option<i8>{
    if x {
        Some(1)
    } else {
        None
    }
}