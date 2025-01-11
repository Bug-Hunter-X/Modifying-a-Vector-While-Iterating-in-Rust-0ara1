fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // Safe way to remove elements during iteration
    let mut i = 0;
    while i < vec.len() {
        println!("{}", vec[i]);
        if vec[i] == 2 {
            vec.remove(i);
        } else {
            i += 1;
        }
    }
    println!("vec after removing element: {:?}",vec);
} 
