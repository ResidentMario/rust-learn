enum Whiteness {
    Whiteish,
    Alabama
}

// When you attach fields to a enum like this you have to declare those
// field with values when you use them. Remember: an enum is just a fancy struct.
// This is kind of sort of just a struct with another struct field in it that you
// can run a match over.
enum Race {
    White{ whiteness: Whiteness },
    Black,
    Mexican,
    Other,
}

// impl Race {
//     fn poorness(&self) -> String {
//         match self {
//             Race::White => String::from("rich prick"),
//             Race::Black => String::from("poor fuck"),
//             Race::Mexican => String::from("no peso"),
//             Race::Other => String::from("r u asian")
//         }
//     }
// }

impl Person {
    fn poorness(&self) -> String {
        match &self.race {
            Race::White{ whiteness } => {
                match whiteness {
                    Whiteness::Whiteish => String::from("exactly average"),
                    Whiteness::Alabama => String::from("rich prick"),
                }
            }
            Race::Black => String::from("poor fuck"),
            Race::Mexican => String::from("no peso"),
            Race::Other => String::from("r u asian"),
        }
    }
}

struct Person {
    name: String,
    age: u8,
    race: Race,
}

fn main() {
    let name = String::from("Peter");
    let age = 27;
    let race = Race::White { whiteness: Whiteness::Whiteish };
    let peter = Person { name, age, race };

    println!("{}", peter.age);
    println!("{}", peter.poorness());
}
