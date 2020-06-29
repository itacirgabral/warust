// Write a function greet() that returns "hello world!"
// Remember to use the correct return type 

fn greet() -> &'static str {
    "hello world!"
}

#[test]
fn test_greets_the_world() {
    assert_eq!(greet(), "hello world!", "should return the correct message");
}