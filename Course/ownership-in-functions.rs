//---------------------------------------------------
//                  Ownership in Functions
//---------------------------------------------------

fn main() {
    let vec_1 = vec![1, 2, 3, 4, 5]; // vec_1 is a vector of integers
    takes_ownership(vec_1); // Ownership of vec_1 is transferred to the function

    let vec_2 = gives_ownership(); // Ownership of the returned vector is transferred to vec_2

    let mut vec_3 = vec![1, 2, 3, 4, 5]; // vec_3 is a vector of integers
    takes_and_gives_ownership(vec_3); // Ownership of vec_3 is transferred to the function
}
