use crate::{
    lang::{get_directives, Language},
    tokenize::tokenize,
};
use rust_stemmers::{Algorithm as StemLanguage, Stemmer};
use serde::Deserialize;
use serde_json::json;
use std::{collections::HashSet, str::FromStr};
use stopwords::{Language as StopLanguage, Spark, Stopwords};

#[derive(Deserialize)]
struct Negations {
    words: Vec<String>,
}

pub fn analyze(analizable: &str, lang: &str) -> String {
    let language: Language = Language::from_str(lang).unwrap();

    let stop_lang: StopLanguage = StopLanguage::from_str(lang).unwrap();

    // @todo Since there is no implementation of `FromStr`
    // forrust_stemmers's Language, we should create our own later
    let stem_lang: StemLanguage = StemLanguage::Portuguese;

    let (afinn, negations) = get_directives(language);

    let json_dictionary: serde_json::Value = serde_json::from_str(&afinn).unwrap();

    let json_negations: Negations = serde_json::from_str(&negations).unwrap();

    let stops: HashSet<_> = Spark::stopwords(stop_lang).unwrap().iter().collect();
    let stemmer = Stemmer::create(stem_lang);

    let tokens = tokenize(&analizable);

    let tokens: Vec<String> = tokens
        .iter()
        .map(|s| stemmer.stem(s).into_owned())
        .collect();

    let mut tokens: Vec<&str> = tokens.iter().map(|s| &**s).collect();

    tokens.retain(|s| !stops.contains(s));

    let mut score = 0;
    let mut negator = 1;
    let mut num_hits = 0;

    for token in tokens {
        let index = json_negations.words.iter().position(|r| r.contains(token));
        match index {
            Some(_) => {
                negator = -1;
                num_hits += 1;
            }
            None => {
                let score_value = json_dictionary
                    .as_object()
                    .unwrap()
                    .iter()
                    .find(|(key, _)| key.contains(token));
                match score_value {
                    Some((_, value)) => {
                        let valor = value.as_i64().unwrap_or(0);
                        score += negator * valor;
                        num_hits += 1;
                    }
                    None => {}
                };
            }
        }
    }

    json!({
        "score": score,
        "num_hits": num_hits
    })
    .to_string()
}

#[cfg(test)]
mod test {
    use super::analyze;
    use serde_json::json;
    #[test]
    fn basic() {
        let analizable = "Eu não odeio a minha vida...";
        let lang = "portuguese";

        let result = analyze(analizable, lang);
        let expected = json!({
            "score": 1,
            "num_hits": 3
        })
        .to_string();
        assert_eq!(result, expected);
    }
}
