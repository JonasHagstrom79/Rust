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
}
