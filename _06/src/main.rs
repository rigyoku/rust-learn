// // 定义
// #[derive(Debug)]
// enum Home {
//     Liy (i8),
//     Nana {
//         speak: String,
//         money: i32,
//     }
// }

// impl Home {
//     fn output(&self) {
//         println!("output");
//     }
// }

// fn log(home: &Home) {
//     home.output();
// }

// fn main() {
//     println!("Hello, world!");

//     let liy = Home::Liy(8);
//     let nana = Home::Nana { speak: String::from("haha"), money: 100 };
//     log(&liy);
//     log(&nana);
//     liy.outputWithType();
//     nana.outputWithType();
//     logOpt(Some(String::from("opt...")));
//     logOpt(None);
//     nana.l1();
//     nana.l2();
// }

// // match
// impl Home {
//     fn outputWithType(&self) {
//         match self {
//             Self::Liy(age) => println!("liy age is {age}"),
//             Self::Nana { speak, money } => println!("Nana speak {speak} has {money}")
//         }
//     }
// }

// fn logOpt(opt: Option<String>) {
//     match opt {
//         Some(s) => println!("opt with value: {s}"),
//         None => println!("None..."),
//     }
// }

// impl Home {
//     fn l1(&self) {
//         match self {
//             Self::Liy(age) => println!("liy age is {age}"),
//             other =>   println!("other... {:#?}", other),
//         }
//     }
//     fn l2(&self) {
//         match self {
//             Self::Liy(age) => println!("liy age is {age}"),
//             _ =>   println!("_..."),
//         }
//     }
// }

// if let
enum Status {
    ok(u8),
    ng
}

fn main() {
    let a = if false {1} else {2};
    println!("{a}");

    let b = Status::ok(200);
    let c = Status::ng;
    if let Status::ok(code) = b {
        println!("status is ok. code is {code}");
    } else {
        println!("ng");
    }
    if let Status::ng = c {
        println!("ng");
    } else {
        println!("ok");
    }

    let Status::ok(code) = c else {
        return ();
    };
    println!("code is {code}");
}
