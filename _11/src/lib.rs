pub mod lib {
    use std::{time::SystemTime, vec};

    pub fn f1() {
        println!("Function f1 in Lib module");
    }
    fn f2(input: &String) -> &String {
        input
    }
    #[test]
    fn t1() {
        let s: String = String::from("Hello, test!");
        println!("Testing f2 with: {}", s);
        assert_eq!(f2(&s), &String::from("Hello, test!"));

        assert_eq!(vec![1,2,3], vec![1,2,3]);

        assert_ne!(vec![1,2,3], vec![1,3]);
    }
    #[test]
    #[ignore]
    fn t2() {
        panic!("error...")
    }
    #[test]
    fn t3() {
        let time = SystemTime::now();
        assert!(1<2,"Time: {:?}", time);
    }
    #[test]
    #[should_panic(expected = "ah")]
    fn f4() {
        panic!("haha")
    }
    #[test]
    #[ignore]
    fn f5<'a>() -> Result<(), &'a str> {
        Result::Ok(Result::<(), &str>::Err("error")?.to_owned())
    }
}