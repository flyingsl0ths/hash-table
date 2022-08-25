# Hash Table

A rust version of https://github.com/jamesroutley/write-a-hash-table

## Usage

```rust
fn main() {
    let mut ht = table::Table::new();

    ht.insert("hello", "world");
    ht.insert("what's", "up!!");
    ht.insert("what's", "up people!!");
    ht.insert("hey", "people!!");

    ht.delete("hey");

    println!("{}", ht);

    if let Some(val) = ht.get("hello") {
        println!("Found: \"{}\"", val);
    }

    ht.clear();
}
```
