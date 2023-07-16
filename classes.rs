use std::collections::HashMap;

struct Person {
    name: String,
    age: u32,
}

fn main() {
    let mut persons = HashMap::new();

    // Add persons to the dictionary
    persons.insert("Alice", Person { name: "Alice".to_string(), age: 25 });
    persons.insert("Bob", Person { name: "Bob".to_string(), age: 30 });
    persons.insert("Charlie", Person { name: "Charlie".to_string(), age: 35 });

    // Lookup by key
    let bob = persons.get("Bob");
    match bob {
        Some(person) => {
            println!("Name: {}", person.name);
            println!("Age: {}", person.age);
        },
        None => {
            println!("Person not found");
        }
    }
}
