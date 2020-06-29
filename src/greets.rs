#[cfg(test)]
mod greets {
    fn greets() -> &'static str {
        "hello world!"
    }

    #[test]
    fn test_greets_the_world() {
        assert_eq!(greets(), "hello world!", "should return the correct message");
    }
}