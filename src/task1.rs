
// Sample task 1 problems

// Fix this code so it compiles
pub fn task1_a() {
    let vector = vec![1, 2, 3, 4, 5, 6];
    print_vector_length(vector);
    print_vector(vector);
}

fn print_vector_length(vector: Vec<i32>) {
    println!("{}", vector.len());
}

fn print_vector(vector: Vec<i32>) {
    println!("{vector:?}");
}




// Fix this code so it compiles
fn task1_b() {
    let original_vector = vec![1, 2, 3, 4, 5, 6];
    let squared_vector = square_vector(original_vector);

    println!("Original vector: {original_vector:?}");
    println!("Squared vector: {squared_vector:?}");
}

fn square_vector(vector: Vec<i32>) -> Vec<i32> {
    vector.into_iter()
        .map(|item| item*item)
        .collect()
}

