struct Student {
    major: String,
}

// First-order function to assign a new major to a student
fn assign_major(s: &mut Student, new_major: &str) {
    s.major = new_major.to_string();
}

// Higher-order function that applies a function to update majors
fn update_majors(students: &mut Vec<Student>, updater: fn(&mut Student, &str), major: &str) {
    for student in students.iter_mut() {
        updater(student, major);
    }
}

// Helper function to print all majors
fn print_majors(students: &Vec<Student>) {
    print!("Majors: ");
    for (i, student) in students.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }
        print!("{}", student.major);
    }
    println!();
}

fn main() {
    let mut students = vec![
        Student { major: "Undeclared".to_string() },
        Student { major: "Undeclared".to_string() },
        Student { major: "Undeclared".to_string() },
    ];

    println!("Original:");
    print_majors(&students);

    update_majors(&mut students, assign_major, "Computer Science");
    println!("After first update:");
    print_majors(&students);

    update_majors(&mut students, assign_major, "Physics");
    println!("After second update:");
    print_majors(&students);
}
