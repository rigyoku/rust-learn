use std::env;

fn main() {
    let args= env::args();
    eprintln!("Arguments passed to the program: {:?}", args);
    // let a = [19,2,4,5];
    // let [c, d, other @ ..] = a;
    // println!("c: {}, d: {}", c, d);
    // println!("other: {:?}", other);
    // let [c, .., d] = a;
    // println!("c: {}, d: {}", c, d);
    // let a = (19,2,3,4,5);
    // let (c, d, .., other) = a;
    // println!("c: {}, d: {}", c, d);
    // println!("other: {:?}", other);
    // let target = args.get(1).expect("No target file provided");
    // let file_path = args.get(2).expect("No file provided");
    let config: _12::Config = _12::Config::build(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    
    let res = _12::run(config).unwrap_or_else(|err| {
        eprintln!("Application error: {}", err);
        std::process::exit(1);
    });

    print!("{:?}", res);

    if let Ok(v) = env::var("debug") {
        println!("env {}", v);
    } else {
        eprintln!("env debug not set");
    }
    
}
