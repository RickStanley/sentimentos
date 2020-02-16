use regex::Regex;


pub fn tokenize(input: &str) -> Vec<String> {
    let new_line_regex = Regex::new(r"\n").unwrap();
    let special_characters_regex = Regex::new(r#"[.,/#!?$%\^&\*;:{}=_`"~()]"#).unwrap();
    let spaces_regex = Regex::new(r"\s\s+").unwrap();

    // This is called shadowing (https://doc.rust-lang.org/rust-by-example/variable_bindings/scope.html)
    let string = input.to_lowercase();
    let string = new_line_regex.replace_all(&string, " ");
    let string = special_characters_regex.replace_all(&string, " ");
    let string = spaces_regex.replace_all(&string, " ");

    string.trim().split(" ").map(|x| x.to_owned()).collect()
}

#[cfg(test)]
mod test {
    use super::tokenize;
    #[test]
    fn tokens() {
        let string = "Esse mundo, é uma decepção!";

        assert_eq!(tokenize(string), ["esse", "mundo", "é", "uma", "decepção"]);
    }
}