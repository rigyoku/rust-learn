use std::{sync::{mpsc, Arc, Mutex}, thread};

// fn main() {
//     println!("Hello, world!");
//     let handle = thread::spawn(|| {
//         for i in 1..5 {
//             println!("spawn {}", i);
//         }
//     });
//     handle.join().unwrap();
//     // thread::sleep(std::time::Duration::from_millis(1));
//     for i in 1..5 {
//         println!("main {}", i);
//         // thread::sleep(std::time::Duration::from_millis(1));
//     }
//     // handle.join().unwrap();
// }

// fn main() {
//     let v = vec![1, 2, 3];
//     let handler = thread::spawn(move || {
//         println!("spawn: {:?}", v);
//     });
//     // println!("main: {:?}", v);
//     handler.join().unwrap();
// }

// fn main() {
//     let (tx, rx) = mpsc::channel();
//     thread::spawn(move || {
//         thread::sleep(std::time::Duration::from_millis(5000));
//         let v = vec![1, 2, 3];
//         tx.send(v).unwrap();
//         println!("spawn: sent {:?}", v);
//     });
//     let received = rx.recv().unwrap();
//     println!("main: {:?}", received);
// }

fn main() {
    let (tx, rx) = mpsc::channel();
    let s2 = tx.clone();
    thread::spawn(move || {
        let v = vec![1, 2, 3];
        for vi in v {
            tx.send(vi).unwrap();
            thread::sleep(std::time::Duration::from_secs(2));
        }
    });
    thread::spawn(move || {
        let v = vec![5, 6, 7];
        for vi in v {
            s2.send(vi).unwrap();
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });
    for rec in rx {
        println!("main: {:?}", rec);
    }
}


// fn main() {
//     let m = Mutex::new(5);
//     {
//         let mut mm = m.lock().unwrap();
//         *mm = 10;
//     }
//     println!("m = {:?}", m);
// }


// fn main() {
//     let m = Arc::new(Mutex::new(5));
//     let mut handlers = vec![];
//     for i in 0..10 {
//         let m = m.clone();
//         let handler = thread::spawn(move || {
//             let mut mm = m.lock().unwrap();
//             *mm += i;
//             println!("mm = {:?}", mm);
//         });
//         handlers.push(handler);
//     }
//     for handler in handlers {
//         handler.join().unwrap();
//     }
//     println!("m = {:?}", m);
// }