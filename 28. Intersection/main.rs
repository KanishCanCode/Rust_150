use std::collections::HashSet;

fn intersection<T: Eq + std::hash::Hash + Clone>(vec1: &[T], vec2: &[T]) -> Vec<T> {
   
    let set1: HashSet<&T> = vec1.iter().collect();

    vec2.iter()
        .filter(|&x| set1.contains(x))
        .cloned() 
        .collect()
}

fn main() {
    // Example usage
    let v1 = vec![1, 2, 3, 4];
    let v2 = vec![3, 4, 5, 6];
    let result = intersection(&v1, &v2);
    println!("Intersection: {:?}", result); 