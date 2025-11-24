fn main() {
    let workshop = Workshop {
        title: "Rust_Workshop".to_owned(),
        instructor: "kanishk".to_owned(),
        duration: 5,
    };
    print_course_overview(workshop);
    let seminar = Seminar {
        title: "Tech_Seminar".to_owned(),
        speaker: "Alice".to_owned(),
        location: "Auditorium".to_owned(),
    };
    print_course_overview(seminar);
}

trait Course {
    fn get_overview(&self) -> String;
}

struct Workshop {
    title: String,
    instructor: String,
    duration: u32,
}

struct Seminar {
    title: String,
    speaker: String,
    location: String,
}

impl Course for Workshop {
    fn get_overview(&self) -> String {
        format!(
            "Workshop: {}, Instructor: {}, Duration: {} hours",
            self.title, self.instructor, self.duration
        )
    }
}

impl Course for Seminar {
    fn get_overview(&self) -> String {
        format!(
            "Seminar: {}, Speaker: {}, Location: {}",
            self.title, self.speaker, self.location
        )
    }
}

fn print_course_overview<T: Course>(a: T) {
    println!("{}", a.get_overview());
}
