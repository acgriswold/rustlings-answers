// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand for a hint.

// // I AM NOT DONE

fn main() {
    let a: Vec<i32> = (0..100).collect();
    // let a = Vec::from_iter(0..100);
    // let a = vec![0; 100];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
