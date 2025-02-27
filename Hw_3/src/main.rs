use std::fs::File;
use std::io::{self, Read, Write};
struct Student {
    name: String,
    id: u32,
}
impl Student {
    fn from_file(path: &str) -> Student {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut lines = contents.lines();
        let name = lines.next().unwrap().to_string();
        let id = lines.next().unwrap().parse().unwrap();

        Student { name, id }
    }
}
fn reading_from_console() {
    let mut buffer = String::new();

    print!("Enter student name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let name = buffer.trim().to_string();
    buffer.clear();
    print!("Enter student ID: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let id = buffer.trim().parse().unwrap();
    let student = Student { name, id };
    println!("Student Name: {}, ID: {}", student.name, student.id);
}

fn reading_from_file() {
    let student = Student::from_file("config.txt");
    println!("Student Name: {}", student.name);
    println!("Student ID: {}", student.id);
}
fn main() {
    reading_from_console();
    reading_from_file();
}