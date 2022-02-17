use anyhow::{anyhow, Result};
//
// written by everettjf
// email : everettjf@live.com
// created at 2022-01-02
//

use std::env;
extern crate linkmap;

fn main() -> Result<(), anyhow::Error> {
    let path = env::args().skip(1).next();
    let path = match path {
        Some(path) => path,
        None => {
            return Err(anyhow!("no file path specified"));
        }
    };
    let linkmap = linkmap::parse_linkmap(&path, true)?;

    println!("path : {}", linkmap.path);
    println!("arch : {}", linkmap.arch);
    println!("object files:");
    for file in &linkmap.object_files {
        println!("{:?}", file);
    }
    println!("sections:");
    for section in &linkmap.sections {
        println!("{:?}", section);
    }
    println!("symbols:");
    for symbol in &linkmap.symbols {
        println!("{:?}", symbol);
    }
    println!("dead_stripped_symbols:");
    for symbol in &linkmap.dead_stripped_symbols {
        println!("{:?}", symbol);
    }

    Ok(())
}
