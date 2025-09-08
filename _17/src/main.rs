// use trpl::Either;

// fn main() {
//     trpl::run(async {
//         // let title: String = load_title("https://app.slack.com/client/E06D01RSM29").await;
//         // println!("Title: {}", title);
//         let title: String = match trpl::race(load_title("https://app.slack.com/client/E06D01RSM29"), load_title("https://app.slack.com/client/E06D01RSM29")).await {
//             Either::Left(left) => left,
//             Either::Right(right) => right,
//         };
//         println!("Title: {}", title);
//     });
//     println!("Hello, world!");
// }

// async fn load_title(url: &str) -> String {
//     let res = trpl::get(&url).await.text().await;
//     trpl::Html::parse(&res).select_first("title").map(|i|i.inner_html()).unwrap()
// }

// extern crate trpl;

// use std::time::Duration;

// fn main() {
//     trpl::run(async {
//         let fut1 = async {
//             for i in 1..10 {
//                 println!("hi number {i} from the first task!");
//                 trpl::sleep(Duration::from_millis(500)).await;
//             }
//         };
//         // for i in 1..10 {
//         //     println!("hi number {i} from the first task!");
//         //     trpl::sleep(Duration::from_millis(500)).await;
//         // }

//         let fut2 = async {
//             // for i in 1..5 {
//             //     println!("hi number {i} from the second task!");
//             //     trpl::sleep(Duration::from_millis(500)).await;
//             // }
//         };

//         for i in 1..5 {
//             println!("hi number {i} from the second task!");
//             trpl::sleep(Duration::from_millis(500)).await;
//         }

//         fut1.await;

//         // trpl::join(fut1, fut2).await;
//     });
// }

// extern crate trpl; // required for mdbook test

// use std::time::Duration;

// fn main() {
//     trpl::run(async {
//         let (tx, mut rx) = trpl::channel();

//         let vals = vec![
//             String::from("hi"),
//             String::from("from"),
//             String::from("the"),
//             String::from("future"),
//         ];

//         for val in vals {
//             tx.send(val).unwrap();
//             trpl::sleep(Duration::from_millis(500)).await;
//         }

//         while let Some(value) = rx.recv().await {
//             println!("received '{value}'");
//         }

//         // while let Some(i) = rx.recv().await {
//         //     println!("received '{i}'");
//         // }
//     });
// }

// extern crate trpl; // required for mdbook test

// use std::time::Duration;

// fn main() {
//     trpl::run(async {
//         let (tx, mut rx) = trpl::channel();

//         let txh= async move {
//             let vals = vec![
//                 String::from("hi"),
//                 String::from("from"),
//                 String::from("the"),
//                 String::from("future"),
//             ];

//             for val in vals {
//                 tx.send(val).unwrap();
//                 trpl::sleep(Duration::from_millis(500)).await;
//             }
//         };

//         let rxh =async {
//             while let Some(value) = rx.recv().await {
//                 println!("received '{value}'");
//             }
//             // for i in rx.recv().await {
//             //     println!("received '{i}'");
//             // }
//         };

//         trpl::join(txh, rxh).await;

//         // while let Some(i) = rx.recv().await {
//         //     println!("received '{i}'");
//         // }
//     });
// }

use std::pin::pin;
use std::{ time::Duration};

use trpl::{ReceiverStream, Stream};
use trpl::StreamExt;

fn main() {
    println!("Hello, world!");

    trpl::run(async {
        let mut stream = pin!(test().throttle(Duration::from_millis(500)));
        while let Some(i) = stream.next().await {
            println!("GOT = {i}");
        }
    });

}

fn test() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();
    trpl::spawn_task(async move {
        let arr = 1..10;
        for i in arr {
            println!("send {i}");
            tx.send(format!("hi number {i} from the first task!")).unwrap();
            trpl::sleep(Duration::from_millis(100)).await;
        }
    });
    ReceiverStream::new(rx)
}