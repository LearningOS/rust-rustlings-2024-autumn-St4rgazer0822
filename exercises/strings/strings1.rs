// strings1.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a
// hint.

fn current_favorite_color() -> String {
    // Equivalent to `String::from("blue")`
    "blue".to_string()
}

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {answer}");
}
