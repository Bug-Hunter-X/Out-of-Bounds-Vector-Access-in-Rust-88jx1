fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);

    // Check if the index is within bounds before accessing
    if vec.len() > 2 {
        let value = vec[2];
        println!("Value: {}", value);
    } else {
        println!("Index out of bounds");
    }
    //Alternatively, using get to handle the error
    match vec.get(2) {
        Some(value) => println!("Value: {}", value),
        None => println!("Index out of bounds")
    }
} 