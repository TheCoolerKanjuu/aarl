mod enums;

use std::env;
use std::path::Path;
use notify::{Watcher, RecursiveMode, Result};

fn main() -> Result<()> {

    let args: Vec<_> = env::args().collect();
    if args.len() <= 1{
        panic!("No path provided.");
    }
    let mut watcher = notify::recommended_watcher(|res| {
        match res {
            Ok(event) => println!("event: {:?}", event),
            Err(e) => println!("error: {:?}", e),
        }
    })?;
    watcher.watch(Path::new(&args[1]), RecursiveMode::Recursive)?;
    loop{}
}