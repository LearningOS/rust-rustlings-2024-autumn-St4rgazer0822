// strings2.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings2` or use the `hint` watch subcommand for a
// hint.

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

fn main() {
    let word = String::from("green");

    if is_a_color_word(&word) {
        //             ^ added to have `&String` which is automatically
        //               coerced to `&str` by the compiler.
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}