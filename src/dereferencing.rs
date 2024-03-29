//---------------------------------------------------
//            Dereferencing
//---------------------------------------------------

fn main() {
    let mut some_data = 42; // some_data is an integer
    let ref_1 = &some_data; // ref1 is a reference to some_data
    let deref_copy = *ref_1; // deref_copy is a copy of some_data
    println!("some_data is: {some_data}, dref_copy is: {deref_copy}"); // This will print: 42
}
