use std::collections::HashSet;

// Generic function that removes duplicates from a vector
fn remove_duplicates<T: Eq + std::hash::Hash + Clone>(vec: Vec<T>) -> Vec<T> {
    let mut seen = HashSet::new();
    let mut unique = Vec::new();

    for item in vec {
        if seen.insert(item.clone()) {
            unique.push(item);
        }
    }

    unique
}

fn main() {
    let numbers = vec![1, 2, 3, 2, 4, 3, 5, 1];
    let result = remove_duplicates(numbers);
    println!("{:?}", result); // Output: [1, 2, 3, 4, 5]
}
