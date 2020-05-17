mod tests {
    use minigrep::utils::{search};

    #[test]
    fn one_result() {
        let query = "something";
        let contents = "\
        Rust,
        something, something, whatever
        Pick a thing.";

        assert_eq!(vec!["        something, something, whatever"], search(query, contents));
    }
}