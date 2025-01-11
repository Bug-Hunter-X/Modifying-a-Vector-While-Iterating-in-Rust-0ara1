fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // This loop will panic if the vector is empty
    for i in 0..vec.len() {
        println!("{}", vec[i]);
        // Removing an element from the vector while iterating changes the length of the vector.
        if vec[i] == 2 {
            vec.remove(i);
        }
    }
}