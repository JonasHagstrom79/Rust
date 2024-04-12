//---------------------------------------------------
//                  Borrowing
//---------------------------------------------------
/* Borrowing Rules
    - At any time, you can have either one mutable reference or any number of immutable references.
    - Refernces must always be valid.

- Solve out two problems
    - Data race
    - Dangling references
 */

fn main() {
    let mut vec_1 = vec![4, 5, 6]; // vec_1 is a vector of integers
    let ref1 = &vec_1; // ref1 is a mutable reference to vec_1
    let ref2 = &vec_1; // ref2 is a mutable reference to vec_1
    println!("ref1: {:?}, ref2: {:?}", ref1, ref2); // This will print: ref1: [4, 5, 6], ref2: [4, 5, 6]
}
