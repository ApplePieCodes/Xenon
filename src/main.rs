use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 1 {
        println!("Xenon Toolchain Version {}", env!("CARGO_PKG_VERSION"));
        println!("{}", env!("CARGO_PKG_DESCRIPTION"));
        println!("Created by {}", env!("CARGO_PKG_AUTHORS"));
        println!("Source Code at {}", env!("CARGO_PKG_REPOSITORY"));
    }
}