

/// 输出log
/// ```sh
/// echo 123
/// ```
/// 
/// ```rust
/// assert_eq!(_14::log(), 1);
/// ```
/// 
pub fn log() -> i32 {
    let a = 1;
    println!("log...");
    a
}

pub use self::a::b::f;

/// aaa
pub mod a {
    /// aaa.bbb
    pub mod b {
        /// aaa.bbb.fff
        pub fn f() {
            println!("a::b::f");
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_log() {
        assert_eq!(log(), 1);
    }
}

