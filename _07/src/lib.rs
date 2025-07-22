pub mod parent {
    use crate::other;

    pub mod child1 {
        fn test() {

        }
    }
    mod child2;
    pub fn log() {
        other::other_fn();
        // child1::log();
        child2::log();
    }
}

pub mod other;

pub struct A {
    pub name: String,
    age: u8,
}

impl A {
    pub fn create_a(name: String) -> A {
        A {
            name,
            age: 1,
        }
    }
}