pub mod c2 {
    pub fn f2() {
        println!("child2 f2");
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_f2() {
            f2();
        }
    }
}