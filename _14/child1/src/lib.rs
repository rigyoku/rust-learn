pub mod c1 {
    pub fn f1() {
        println!("child1 f1");
        child2::c2::f2();
        println!("rand: {}", rand::random::<u8>());
    }
}