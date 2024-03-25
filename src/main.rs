fn main() {
    // Definition of a variable
    let x = 10; // x is immutable
    println!("x is {}", x); // This will print: x is 10

    // Mutability
    let mut y = 10; // y is mutable
    y = 20; // This will change the value of y
    println!("y is {}", y); // This will print: y is 20

    // Scope
    {
        let z = 30; // z is only available in this scope
        println!("z is {}", z); // This will print: z is 30
    }

    // Shadowing
    let t = 10; // t is 10
    let t = t + 20; // t is 30
    println!("x is {}", x); // This will print: x is 10

    // Constants
    const MAX_POINTS: u32 = 100_000; // _ can be used to improve readability
    println!("MAX_POINTS is {}", MAX_POINTS); // This will print: MAX_POINTS is 100000

    //unsigned integer
    let a: u8 = 255; // or 0 to 255
    println!("a is {}", a); // This will print: a is 255

    //signed integer
    let b: i8 = -128; // or -127 to 127
    println!("b is {}", b); // This will print: b is -128

    //floating point
    let c: f32 = 3.14; // or f64

    // platform dependent
    let d: isize = 10; // or i32 or i64
    let e: usize = 10; // or u32 or u64
    println!("d is {}", d); // This will print: d is 10
    println!("e is {}", e); // This will print: e is 10

    //character
    let f: char = 'a'; // or 'A' or '1'
    println!("f is {}", f); // This will print: f is a

    //boolean
    let g: bool = true; // or false
    println!("g is {}", g); // This will print: g is true

    //type aliasing
    type Age = u32; // Age is an alias for u32
    let age: Age = 20; // age is of type Age
    println!("age is {}", age); // This will print: age is 20

    // Type Conversion
    let h: i32 = 10; // h is of type i32
    let i: f64 = h as f64; // This will convert h to f64
    println!("i is {}", i); // This will print: i is 10.0

    // &str and String
    let fixed_string: &str = "Fixed length string"; // &str is a fixed length string
    let flexible_string: String = String::from("this string will grow"); // String is a growable string, removed mut as it's not mutated
    println!("j is {}", fixed_string); // This will print: j is Fixed length string
    println!("k is {}", flexible_string); // Corrected variable name to flexible_string
                                          // Arrays
    let mut array_1 = [1, 2, 3, 4, 5]; // array_1 is an array of integers
    let mut array_2 = [0; 5]; // array_2 is an array of 5 zeros
    let mut array_3 = [1, 2, 3, 4, 5]; // array_3 is an array of integers
    let mut array_4 = [0; 5]; // array_4 is an array of 5 zeros

    // vectors
    let mut vector_1 = vec![1, 2, 3, 4, 5]; // vector_1 is a vector of integers
    let mut vector_2 = vec![0; 5]; // vector_2 is a vector of 5 zeros

    // Tuples
    let tuple_1 = (1, 2, 3, 4, 5); // tuple_1 is a tuple of integers
    let tuple_2 = (1, "hello", 3.14, true); // tuple_2 is a tuple of different types
    let (x, y, z, a, b) = tuple_1; // destructuring a tuple
    let my_info = ("salary", 1000, "Age", 20); // my_info is a tuple
    let (salary_label, salary_value, age_label, age_value) = my_info; // Corrected destructuring

    let unit = (); // unit is a tuple with no elements

    my_function("This is my function"); // This will print: Hello, World!
    let str = "Function call with a variable"; // str is a string
    my_function(str); // This will print: Function call with a variable
    let result: (i32, i32, i32, f32, i32) = basic_math(10, 20);

    let full_name: String = {
        let first_name = "John";
        let last_name = "Doe";
        format!("{} {}", first_name, last_name)
    };

    check_marks(85);
    // Example usage of the check_marks function

    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    //---------------------------------------------------
    //                  Ownership Basics
    //------------------------------------------------

    /*
    1. Each value has a variable thats's its "owner"
    2. There can only be one owner at a time
    3. When the owner goes out of scope, the value is cleaned up
    */

    let s1 = String::from("hello"); // s1 is the owner of the string "hello"
    let s2 = s1.clone(); // s2 is now the owner of the cloned string "hello"
    println!("{s2}"); // This will print: hello
    println!("{s1}"); // This will print: hello

    let s = String::from("hello"); // s is the owner of the string "hello"
    my_function(&s); // This will print: hello
    println!("{s}"); // This will print: hello

    let a = 10; // a is an integer
    let b = 20; // b is an integer
    let result = multiolication(a, b); // result is the product of a and b
    println!("{result}"); // This will print: 200

    let (sum, difference, product, quotient, remainder) = basic_math(10, 20);
    println!("Sum: {sum}"); // This will print: Sum: 30
    println!("Difference: {difference}"); // This will print: Difference: -10
    println!("Product: {product}"); // This will print: Product: 200
    println!("Quotient: {quotient}"); // This will print: Quotient: 0.5
    println!("Remainder: {remainder}"); // This will print: Remainder: 10

    let traveler_number: u64 = 282369289;
    // Exempel på traveler number
    match find_xy_for_traveler_number(traveler_number) {
        Some((x, y)) => println!("Hittade en matchning: x = {}, y = {}", x, y),
        None => println!("Ingen matchning hittades inom den angivna gränsen."),
    }

    //---------------------------------------------------
    //                  Ownership in Functions
    //---------------------------------------------------

    let vec_1 = vec![1, 2, 3, 4, 5]; // vec_1 is a vector of integers
    takes_ownership(vec_1); // Ownership of vec_1 is transferred to the function

    let vec_2 = gives_ownership(); // Ownership of the returned vector is transferred to vec_2

    let mut vec_3 = vec![1, 2, 3, 4, 5]; // vec_3 is a vector of integers
    takes_and_gives_ownership(vec_3); // Ownership of vec_3 is transferred to the function

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

    let mut vec_1 = vec![4, 5, 6]; // vec_1 is a vector of integers
    let ref1 = &vec_1; // ref1 is a mutable reference to vec_1
    let ref2 = &vec_1; // ref2 is a mutable reference to vec_1
    println!("ref1: {:?}, ref2: {:?}", ref1, ref2); // This will print: ref1: [4, 5, 6], ref2: [4, 5, 6]
                                                    //26
}

fn takes_and_gives_ownership(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(10); // This will add 10 to the vector
    vec // Ownership of the vector is transferred to the caller
}

fn takes_ownership(vec: Vec<i32>) {
    println!("vec is: {:?}", vec); // This will print the vector
}

fn gives_ownership() -> Vec<i32> {
    vec![4, 5, 6]
}

fn find_xy_for_traveler_number(traveler_number: u64) -> Option<(i32, i32)> {
    let max_limit = 100; // Sätt en övre gräns för sökningen

    for x in 0..max_limit {
        for y in 0..max_limit {
            if 3u64.pow(x) * 2u64.pow(5 * y) == traveler_number {
                // Konvertera x och y till i32 här
                return Some((x as i32, y as i32));
            }
        }
    }
    None // Returnera None om ingen matchning hittas
}

// Function definition
fn my_function(s: &str) {
    println!("{s}"); // This will print the value of s
}

fn multiolication(a: i32, b: i32) -> i32 {
    println!("{} * {} = {}", a, b, a * b); // This will print: a * b = a * b
    a * b
}

fn basic_math(a: i32, b: i32) -> (i32, i32, i32, f32, i32) {
    let sum = a + b; // sum is the sum of a and b
    let difference = a - b; // difference is the difference of a and b
    let product = a * b; // product is the product of a and b
    let quotient = a as f32 / b as f32; // quotient is the quotient of a and b, cast to f32 for floating-point division
    let remainder = a % b; // remainder is the remainder of a and b

    return (sum, difference, product, quotient, remainder);
}

fn check_marks(marks: u32) {
    match marks {
        0..=40 => println!("Fail"),          // marks between 0 and 40
        41..=60 => println!("Pass"),         // marks between 41 and 60
        61..=75 => println!("Merit"),        // marks between 61 and 75
        76..=100 => println!("Distinction"), // marks between 76 and 100
        _ => println!("Invalid marks"),      // marks less than 0 or greater than 100
    }
}
