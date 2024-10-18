// tests1.rs
//
// Tests are important to ensure that your code does what you think it should
// do. Tests can be run on this file with the following command: rustlings run
// tests1
//
// This test has a problem with it -- make the test compile! Make the test pass!
// Make the test fail!
//
// Execute `rustlings hint tests1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // When writing unit tests, it is common to import everything from the outer
    // module (`super`) using a wildcard.
    use super::*;

    #[test]
    fn you_can_assert() {
        assert!(is_even(0));
        assert!(!is_even(-1));
        //      ^ You can assert `false` using the negation operator `!`.
    }
}