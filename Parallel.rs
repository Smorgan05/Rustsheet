use dashmap::DashMap;
use std::sync::Arc;

// Define a Person struct (just for demonstration purposes)
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: &str, age: u32) -> Self {
        Person {
            name: name.to_owned(),
            age,
        }
    }
}

fn main() {
    // Create a new DashMap instance
    let map: Arc<DashMap<String, Person>> = Arc::new(DashMap::new());

    // Insert some persons into the dictionary
    map.insert("1".to_owned(), Person::new("Alice", 25));
    map.insert("2".to_owned(), Person::new("Bob", 30));
    map.insert("3".to_owned(), Person::new("Charlie", 35));

    // Read a person from the dictionary
    let person = map.get("2").expect("Person not found");
    println!("Name: {}, Age: {}", person.name, person.age);
}

