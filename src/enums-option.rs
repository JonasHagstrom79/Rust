//-----------------------------------------------------
//          Option
// ----------------------------------------------------

// Define a struct for a student with a name and an optional grade
struct Student {
    name: String,
    grade: Option<u32>,
}

// Function to retrieve a student's grade from a database, if it exists
fn get_grade(student_name: &String, student_db: &Vec<Student>) -> Option<u32> {
    // Iterate through the student database
    for student in student_db {
        // Check if the current student's name matches the query
        if student.name == *student_name {
            // Return the student's grade, which may be None or Some(u32)
            return student.grade;
        }
    }
    // Return None if the student is not found in the database
    None
}

// Main function to demonstrate usage of the Option enum with student grades
fn main() {
    // Create a database of students
    let student_db = vec![
        Student {
            name: String::from("Alice"),
            grade: Some(9), // Alice has a grade of 9
        },
        Student {
            name: String::from("Bob"),
            grade: Some(8), // Bob has a grade of 8
        },
        Student {
            name: String::from("Charlie"),
            grade: None, // Charlie's grade is not available
        },
    ];

    // Specify the name of the student to search for
    let student_name = String::from("Alice");
    // Retrieve the grade of the specified student
    let student_grade = get_grade(&student_name, &student_db);

    // Match on the Option<u32> to handle both Some and None cases
    match student_grade {
        Some(grade) => println!("{}'s grade is: {}", student_name, grade),
        None => println!("{}'s grade is not available", student_name),
    }
}

// The Option<T> enum is a part of Rust's standard library, defined as:
// enum Option<T> {
//     None, // Represents absence of a value
//     Some(T), // Represents presence of a value of type T
// }
