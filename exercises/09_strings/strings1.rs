// TODO: Fix the compiler error without changing the function signature.
fn current_favorite_color() -> String {
     // We need to convert the string literal (&str) into an owned String
    "blue".to_string()
}

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {answer}");
}
