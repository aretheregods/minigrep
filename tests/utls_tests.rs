mod tests {
    use minigrep::utils::{search, search_case_insensitive};

    #[test]
    fn search_result() {
        let query = "Thing";
        let contents = "\
        Rust,
        something, something, whatever
        Pick a Thing.";

        assert_eq!(
            vec!["        Pick a Thing."],
            search(query, contents));
    }

    #[test]
    fn search_result_insensitive() {
        let query = "thing";
        let contents = "\
        Rust,
        something, something, whatever
        Pick a Thing.";

        assert_eq!(
            vec!["        something, something, whatever", "        Pick a Thing."],
            search_case_insensitive(query, contents));
    }
}