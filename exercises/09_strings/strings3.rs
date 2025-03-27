fn trim_me(input: &str) -> &str {
    // TODO: Remove whitespace from both ends of a string.
    input.trim()
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There are multiple ways to do this.

    // Option 1: Using the `format!` macro (often preferred for clarity)
    // It creates a new String by formatting the input and the literal together.
    // format!("{} world!", input)

    // Option 2: Using String concatenation with `+`
    // Note: The left side must be a String, the right side a &str.
    // input.to_string() + " world!"

    // Option 3: Using `push_str` (modifies an existing String)
    let mut result = input.to_string();
    result.push_str(" world!");
    result
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons".
    // The `.replace()` method on &str does exactly this.
    // It searches for the first argument ("cars") and replaces all occurrences
    // with the second argument ("balloons"), returning a *new* owned String.
    input.replace("cars", "balloons")
}
fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool",
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons",
        );
    }
}
