use std::io;

fn main() {
    loop {
        println!("Input a number: ");
        let mut num = String::new();

        io::stdin().read_line(&mut num).expect("Failed to read line.");

        let num: u32 = match num.trim().parse() {
            Ok(num) => num,  // the arrow function, comma styling, and use of _ is very JS
            Err(_) => continue,
        };

        println!("Inputted {}", num);
    }
}
