fn append_vec<T>(vec: &mut Vec<T>,item: T ){
    vec.push(item);
}


fn main() {
let mut numbers = Vec::new();
append_vec(&mut numbers, 42);
append_vec(&mut numbers,14);
println!("Vectors :{:?}",numbers);
}
