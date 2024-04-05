//--------------------------------------------------
//          Borrowing in Functions
//--------------------------------------------------

/*
- Borrowing Rules
    - At any time, you can have either one mutable reference or any number of immutable references.
    - Refernces must always be valid.
 */

fn main() {
    let vec_9 = vec![1, 2, 3]; // vec_9 is a vector of integers
                               // Better to pass a reference to vec_9 into the function, cheaper than cloning
    let ref9 = &vec_9; // ref9 is a reference to vec_9
    borrows_vec(ref9); // vec_9 is borrowed by the function
}
