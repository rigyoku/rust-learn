// // 普通结构体
// fn main() {
//     println!("===== Hello, world! =====");
//     let mut user1 = User {
//         name: String::from("bob"),
//         age: 1,
//     };
//     println!("user1 name is {} age is {}", user1.name, user1.age);

//     let name: String = String::from("tomy");
//     let mut user2 = User { name, age: -1 };
//     println!("user2 name is {} age is {}", user2.name, user2.age);

//     user2.name = user1.name;
//     println!("user1 age is {}", user1.age);
//     user1.name = String::from("tomy");
//     println!("user1 name is {} age is {}", user1.name, user1.age);
//     println!("user2 name is {} age is {}", user2.name, user2.age);

//     let user3 = User {
//         name: String::from("user3"),
//         ..user2
//     };
//     println!("user3 name is {} age is {}", user3.name, user3.age);

//     let user4 = User {
//         name: user2.name,
//         ..user3
//     };

//     println!("user2 age is {}", user2.age);
//     println!("user3 name is {} age is {}", user3.name, user3.age);
//     println!("user4 name is {} age is {}", user4.name, user4.age);

//     println!("user4 name is {} age is {}", user4.name + "xx", user4.age);
//     // println!("user4 name is {} age is {}", user4.name, user4.age);


// }

// struct User {
//     name: String,
//     age: i32,
// }

// // 元组结构体
// fn main() {
//     let t = (0,1,2);
//     let (t1, t2, t3) = t;
//     println!("{t1} {t2} {t3}");

//     let t1 = Tuple(1);
//     let Tuple (t2) = t1;
//     println!("{t2}");
// }

// struct Tuple (i8);

#[derive(Debug)]
struct Log {
    txt: String
}

impl Log {
    fn output(self) {
        println!("output: {}", self.txt);
    }
    fn haha() -> Self {
        println!("hahaha");
        Log {
            txt: String::from("haha")
        }
    }
}

fn main() {
    let log = Log {
        txt: String::from("value")
    };
    log.output();
    // println!("{}", log.txt);
    let ha = Log::haha();
    println!("{:#?}", ha)
}
