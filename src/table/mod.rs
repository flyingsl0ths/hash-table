use self::prime::next_prime;
use std::fmt::Display;

mod prime;

const INITIAL_BASE_SIZE: usize = 53usize;

type Item = Option<(String, String)>;

pub struct Table {
    base_size: usize,
    size: usize,
    count: usize,
    items: Vec<Item>,
}

impl Table {
    pub fn new() -> Self {
        Self::new_with_size(INITIAL_BASE_SIZE)
    }

    fn new_with_size(size: usize) -> Self {
        let actual_size = next_prime(size);

        Table {
            base_size: size,
            size: actual_size,
            count: 0,
            items: vec![None; actual_size],
        }
    }

    pub fn insert(&mut self, key: String, value: String) {
        let load = self.count * 100 / self.size;

        if load > 70 {
            self.resize_up();
        }

        let key = &key;
        let mut index = Self::get_hash(key, self.size, 0usize);
        let mut item = self.items.get_mut(index);
        let mut i = 1;

        while let Some(Some((item_key, item_value))) = item {
            if item_key == key {
                *item_value = value;
                return;
            }
            index = Self::get_hash(key, self.size, i);
            item = self.items.get_mut(index);
            i += 1;
        }

        self.items
            .insert(index, Some((key.to_string(), value.to_string())));

        self.count += 1;
    }

    pub fn get(&self, key: String) -> Option<&String> {
        let key = &key;
        let mut index = Self::get_hash(key, self.size, 0usize);
        let mut item = self.items.get(index);
        let mut value: Option<&String> = None;
        let mut i = 1;

        while let Some(entry) = item {
            match entry {
                Some((item_key, item_value)) if item_key == key => {
                    value = Some(item_value);
                    break;
                }
                _ => (),
            }
            index = Self::get_hash(key, self.size, i);
            item = self.items.get(index);
            i += 1;
        }

        value
    }

    pub fn delete(&mut self, key: String) {
        let load = self.count * 100 / self.size;

        if load < 10 {
            self.resize_down();
        }

        let key = &key;
        let mut index = Self::get_hash(key, self.size, 0);
        let mut item = self.items.get_mut(index);
        let mut i = 1;

        while let Some(entry) = item {
            match entry {
                Some((item_key, _)) if item_key == key => {
                    *entry = None;
                    return;
                }
                _ => (),
            }
            index = Self::get_hash(key, self.size, i);
            item = self.items.get_mut(index);
            i += 1;
        }

        self.count -= 1;
    }

    pub fn clear(&mut self) {
        self.count = 0usize;
        self.items.clear();
    }

    fn resize_up(&mut self) {
        let new_size = self.base_size * 2;
        self.resize(new_size);
    }

    fn resize_down(&mut self) {
        let new_size = self.base_size / 2;
        self.resize(new_size);
    }

    fn resize(&mut self, base_size: usize) {
        if self.base_size < INITIAL_BASE_SIZE {
            return;
        }

        let mut other = Self::new_with_size(base_size);

        self.items.iter().filter(|e| e.is_some()).for_each(|e| {
            let (k, v) = e.as_ref().unwrap();
            other.insert(k.clone(), v.clone());
        });

        self.count = other.count;

        self.base_size = other.base_size;

        self.size = other.size;

        self.items = other.items;
    }

    fn get_hash(input: &str, bucket_count: usize, attempt: usize) -> usize {
        let hash_a = Self::hash_input(input, prime::HASH_PRIME, bucket_count);
        let hash_b = Self::hash_input(input, prime::HASH_PRIME2, bucket_count);
        (hash_a + (attempt * (hash_b + 1))) % bucket_count
    }

    fn hash_input(input: &str, prime: u32, bucket_count: usize) -> usize {
        let mut hash = 0usize;
        let input_length = input.len();

        let mut i = 0usize;
        for c in input.chars() {
            if let Some(c) = c.to_digit(10) {
                hash += (prime.pow((input_length - (i + 1)) as u32) as usize) * (c as usize);
            }

            i += 1;
        }

        hash = hash % bucket_count;

        hash
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}",
            self.items
                .iter()
                .filter(|e| e.is_some())
                .map(|e| {
                    let (k, v) = e.as_ref().unwrap();
                    format!("({}, {})", k, v)
                })
                .collect::<Vec<String>>()
        )
    }
}
