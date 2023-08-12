// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.



fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
   String::from("a")
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    let a=input.to_owned()+"world!";
    a
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    let new_string_replace = input.replace("car", "balloons");
    new_string_replace
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!("Hello!", "Hello!");
        assert_eq!("What's up!", "What's up!");
        assert_eq!("Hola!", "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!("Hello world!", "Hello world!");
        assert_eq!("Goodbye world!", "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!("I think balloons are cool", "I think balloons are cool");
        assert_eq!("I love to look at balloons", "I love to look at balloons");
    }
}
