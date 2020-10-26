use std::io;

trait Print {
    fn print(&self) -> String;
}

struct Person {
    first_name: String,
    last_name: String,
    id: i64,
}

impl Person {
    fn new(first_name: String, last_name: String, id: i64) -> Person {
        return Person{first_name, last_name, id};
    }
    
}

impl Print for Person {
    fn print(&self) -> String{
        return format!("Name: {}, {} \nID: {}", self.last_name, self.first_name, self.id);
    }
}

struct Student {
    scores: Vec<i32>,
    person: Person,
}

impl Student {
    fn new(p: Person, scores: Vec<i32>) -> Student{
        return Student{person: p, scores};
    }

    fn calculate(&self) -> char {
        let mut average: i32 = 0;
        for i in 0..self.scores.len() {
            average += self.scores[i];
        }
        average = average / self.scores.len() as i32;
        let grade = match average {
            90..=100 => 'O',
            80..=89 => 'E',
            70..=79 => 'A',
            55..=69 => 'P',
            40..=54 => 'D',
            average if average < 40 => 'T',
            _ => '-',
        };
        return grade;
    }
}

impl Print for Student {
    fn print(&self) -> String {
        return format!("{} \nGrade: {}", self.person.print(), self.calculate());
    }
}

pub fn run() {
    // Get whole input

    // Create structs
    let person: Person = Person::new("Dick".to_string(), "Thunder".to_string(), 567);
    let vector: Vec<i32> = vec![100, 80];
    let student: Student = Student::new(person, vector);
    println!("{}", student.print())
}