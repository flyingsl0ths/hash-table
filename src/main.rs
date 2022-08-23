mod table;

fn main() {
    let mut ht = table::Table::new();

    ht.insert("hello".to_string(), "world".to_string());
    ht.insert("what's".to_string(), "up!!".to_string());
    ht.insert("what's".to_string(), "up people!!".to_string());
    ht.insert("hey".to_string(), "people!!".to_string());

    ht.delete("hey".to_string());

    println!("{}", ht);

    if let Some(val) = ht.get("hello".to_string()) {
        println!("Found: \"{}\"", val);
    }

    ht.clear();
}
