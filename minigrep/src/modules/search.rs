pub fn search<'a>(query: &str, contents: &'a str, case_insensitive: bool) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if case_insensitive {
            if line.to_uppercase().contains(&query.to_uppercase()) {
                results.push(line);
                continue;
            }
        }
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents, false));
    }
}