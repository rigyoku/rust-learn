// // enum List<T> {
// //     Cons(T, Box<List<T>>),
// //     Nil,
// // }

// // use List::{Cons, Nil};

// // fn main() {
// //     let list= Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
// //     output(list);
// //     println!("Hello, world!");
// // }

// // fn output(list: List<i32>) {
// //     match list {
// //         Cons(head, list) => {
// //             println!("head is {}", head);
// //             output(*list);
// //         },
// //         Nil => {
// //             println!("nil");
// //         }
// //     }
// // }

// // fn main() {
// //     let x = 5;
// //     let y = &x;
// //     assert_eq!(5, x);
// //     assert_eq!(5, *y);
// //     // assert_eq!(5, y);
// // }

// // use std::ops::Deref;

// // struct MyBox<T>(T);

// // impl<T> Deref for MyBox<T> {
// //     type Target = T;

// //     fn deref(&self) -> &Self::Target {
// //         &self.0
// //     }
// // }

// // fn log(str: &str) {
// //     println!("str: {}", str);
// // }

// // fn main() {
// //     let a = "123";
// //     let b = MyBox(a);
// //     assert_eq!(a, *b);
// //     log(&b);
// // }

// // enum List<T> {
// //     Cons(T, Box<List<T>>),
// //     Nil,
// // }

// // use List::{Cons, Nil};

// // fn main() {
// //     let share = Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))));
// //     let a= Cons(1, share);
// //     let b= Cons(2, share);
// //     println!("Hello, world!");
// // }

// // use std::rc::Rc;

// // enum List<T> {
// //     Cons(T, Rc<List<T>>),
// //     Nil,
// // }

// // use List::{Cons, Nil};

// // fn main() {
// //     let share = Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil)))));
// //     let a= Cons(1, Rc::clone(&share));
// //     let b= Cons(2, share.clone());
// //     println!("count: {}", Rc::strong_count(&share));
// //     println!("Hello, world!");
// // }

// // trait Logger {
// //     fn log(&self, msg: &str);
// // }

// // struct Console<'a> {
// //     logs: Vec<&'a str>,
// // }

// // impl<'a> Logger for Console<'a> {
// //     fn log(&self,  msg: &str) {
// //         self.logs.push(msg);
// //     }
// // }

// // fn main() {
// //     let console = Console { logs: Vec::new() };
// //     console.log("Hello, world!");
// // }

// // use std::cell::RefCell;

// // trait Logger<'a> {
// //     fn log(&self, msg: &'a str);
// // }

// // struct Console<'a> {
// //     logs: RefCell<Vec<&'a str>>,
// // }

// // impl<'a> Logger<'a> for Console<'a> {
// //     fn log(&self,  msg: &'a str) {
// //         let mut a = self.logs.borrow_mut();
// //         let mut b = self.logs.borrow_mut();
// //         a.push(msg);
// //     }
// // }

// // fn main() {
// //     let console = Console { logs: RefCell::new(Vec::new()) };
// //     console.log("Hello, world!");
// // }

// use std::{cell::RefCell, ops::Deref, rc::Rc};

// #[derive(Debug)]
// enum List<T> {
//     Cons(T, Rc<RefCell<List<T>>>),
//     Nil,
// }

// use List::{Cons, Nil};

// fn main() {
//     let share = Rc::new(RefCell::new(Cons(2, Rc::new(RefCell::new(Cons(3, Rc::new(RefCell::new(Nil))))))));
//     let a= Cons(1, share.clone());
//     let b= Cons(2, share.clone());
//     if let Cons(head, tail1) = &a {
//         *tail1.borrow_mut() = Cons(999, Rc::new(RefCell::new(Nil)));
//     } else {
//         println!("aa is nil");
//     }
//     if let Cons(head, tail1) = &b {
//         if let Cons(head2, tail2) = tail1.borrow().deref() {
//             if let Cons(head3, tail3) = &*tail2.borrow() {
//             println!("bb x 1 head is {}", head);
//             println!("bb x 2 head is {}", head2);
//             println!("bb x 3 head is {}", head3);
//         } else {
//             println!("bb x 2 nil");
//         }
//         } else {
//             println!("bb x 2 nil");
//         }
//     } else {
//         println!("bb is nil");
//     }
//     println!("count: {}", Rc::strong_count(&share));
//     println!("Hello, world {:#?}", b);
// }

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}


fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack.
    // println!("a next item = {:?}", a.tail());
}