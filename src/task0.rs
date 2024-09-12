

// Given an integer n, the next number in the hailstone sequence can be calculated using the function:
//      If n is even: divide by 2
//      If n is odd: multiply by 3 then add 1


// Write a function that takes in a number n, and return the next number in the hailstone sequence
fn next_hailstone_number(n: u32) -> u32 {
    if n%2 == 0 {
        return n/2
    } else {
        return n*3 + 1
    }
}


// Using your previous function:
// Write a function that takes in an integer n and returns a vector containing the hailstone sequence
// beginning with n and ending with 1.
fn get_hailstone_sequence(mut n: u32) -> Vec<u32> {
    let mut v = vec![n];

    while n > 1 {
        n = next_hailstone_number(n);
        v.push(n);
    }

    v
}