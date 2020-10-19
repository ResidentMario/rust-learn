// Option is a prelude enum defined thusly:
// enum Option<T> {
//     Some(T),
//     None
// }
// T is generic syntax which, I don't know anything about at time of writing.
// A

fn main() {
    let some_number = Some(5);
    let absent_number: Option<i32> = None;
    
    match some_number {
        None => println!("Matched None"),
        Some(v) => println!("Matched Some, no you can do something with it.")
    }
}
