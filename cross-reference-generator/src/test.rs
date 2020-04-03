struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tupple struct
struct AnotherColor(u8, u8, u8);

struct Person {

    first_name : String,
    last_name : String,
}

impl Person {
    // Construct the person
    fn new (first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    fn full_name (&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name (&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Set name as tuple
    fn set_as_tuple (self) -> (String, String) {

        (self.first_name, self.last_name)
    }
}

pub fn run () {

    let mut color = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    println!("{}, {}, {}", color.red, color.green, color.blue);

    color.blue = 200;

    println!("{}, {}, {}", color.red, color.green, color.blue);

    let mut c = AnotherColor(255, 69, 123);

    println!("{} {} {}", c.0, c.1, c.2);

    c.1 = 31;

    println!("{} {} {}", c.0, c.1, c.2);

    let mut person = Person::new("John", "Doe");

    println!("{}", person.full_name());

    person.set_last_name("Jane");

    println!("{}", person.full_name());

    println!("{:?}", person.set_as_tuple());

}
