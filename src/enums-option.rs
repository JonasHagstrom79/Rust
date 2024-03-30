//-----------------------------------------------------
//          Option
// ----------------------------------------------------

struct Student {
    name: String,
    grade: u32,
}

fn main() {
    let student_db = vec![
        Student {
            name: String::from("Alice"),
            grade: 9,
        },
        Student {
            name: String::from("Bob"),
            grade: 8,
        },
    ];
}
