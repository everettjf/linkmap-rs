# linkmap-rs

Linkmap file parse library for rust.

Usage : 

> ref examples/main.rs


```rust
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
```