// Calls of this function should be replaced with calls of `string_slice` or `string`.
fn placeholder() {}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: Here are a bunch of values - some are `String`, some are `&str`.
// Your task is to replace `placeholder(…)` with either `string_slice(…)`
// or `string(…)` depending on what you think each value is.
fn main() {
    // Why is the below str_slice and not str?
    // Because the value is a string slice, not a string.
    // A string slice is a reference to a string, while a string is an owned string.
    // The value is a reference to a string literal, so it is a string slice.
    // placeholder("blue");
    string_slice("blue");
    
    // placeholder("red".to_string());
    // Why is the below string and not string_slice?
    // Because the value is a String, not a string slice.
    // The value is a new owned String, so it is a String.
    string("red".to_string());

    // placeholder(String::from("hi"));
    string(String::from("hi"));

    // placeholder("rust is fun!".to_owned());
    string("rust is fun!".to_owned());

    // placeholder("nice weather".into());
    string("nice weather".into());

    // placeholder(format!("Interpolation {}", "Station"));
    string(format!("Interpolation {}", "Station"));

    // WARNING: This is byte indexing, not character indexing.
    // Character indexing can be done using `s.chars().nth(INDEX)`.
    // placeholder(&String::from("abc")[0..1]);

    // Why is the below string_slice and not string?
    // Because the value is a string slice, not a string.
    // The value is a reference to a substring of a string, so it is a string slice.
    string_slice(&String::from("abc")[0..1]);

    // placeholder("  hello there ".trim());
    // Why is the below string_slice and not string?
    // Because the value is a string slice, not a string.
    // The value is a reference to a string slice, so it is a string slice.
    string_slice("  hello there ".trim());

    // placeholder("Happy Monday!".replace("Mon", "Tues"));
    // Why is the below string and not string_slice?
    // Because the value is a String, not a string slice.
    // The value is a new owned String, so it is a String.
    string("Happy Monday!".replace("Mon", "Tues"));

    // placeholder("mY sHiFt KeY iS sTiCkY".to_lowercase());
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
