enum Type<T> {
    Name(T),
}

struct People<T> {
    something: T,
}

impl<T> People<T> {
    fn speak<U>(self, append: &U) {
        println!("speak something {:?} append {:?}", self.something, append);
    }
}

fn main() {
    println!("Hello, world!");
}


