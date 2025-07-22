fn main() {
    // println!("Hello, world!");
    // let x ;
    // x = 1;
    // println!("{x}");
    // // x = 2;
    // // println!("{x}");
    // const a: i8 = 1;
    // let l = log(x, 'c');
    // println!("{l}");
    // let mut index = 0;
    // let l = loop {
    //     index+=1;
    //     if index == 2 {
    //         break 2*index
    //     }
    // };
    // println!("{l}");

    // let mut l1 = 1;
    // let res: i32 = 'l1: loop {
    //     l1+=1;
    //     println!("l1: {l1}");
    //     let mut l2 = 1;
    //     loop {
    //         l2+=1;
    //         println!("l2: {l2}");
    //         if l2 == 3 {
    //             break 'l1 1;
    //         }
    //     };
    // };
    // println!("res:{res}");

    for item in (0..2).rev() {
        println!("{item}");
    }
}


// fn log(num: i32, c: char) -> i32 {
//     println!("num: {num} char: {c}");
//     num + 7
// }