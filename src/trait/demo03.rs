trait GetName {
    fn get_name(&self) -> &String;
}

trait GetAge {
    fn get_age(&self) -> u32;
}

struct PeopleMatchInformation<T, U> {
    master: T,
    student: U,
}

impl<T: GetName + GetAge, U: GetName + GetAge> PeopleMatchInformation<T, U> {
    fn print_all_information(&self) {
        println!("Master Name: {}", self.master.get_name());
        println!("Student Name: {}", self.student.get_name());
    }
}

struct Teacher {
    name: String,
    age: u32,
}

impl GetName for Teacher {
    fn get_name(&self) -> &String {
        &self.name
    }
}

impl GetAge for Teacher {
    fn get_age(&self) -> u32 {
        self.age
    }
}

struct Student {
    name: String,
    age: u32,
}

impl GetName for Student {
    fn get_name(&self) -> &String {
        &self.name
    }
}

impl GetAge for Student {
    fn get_age(&self) -> u32 {
        self.age
    }
}

fn main() {
    let s = Student {
        name: String::from("Alex"),
        age: 18,
    };
    let t = Teacher {
        name: String::from("John"),
        age: 18,
    };
    let m = PeopleMatchInformation {
        master: t,
        student: s,
    };

    m.print_all_information();
}
