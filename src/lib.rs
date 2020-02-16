mod lang;
mod tokenize;
mod utils;

use std::path::Path;
use wasm_bindgen::prelude::*;

use serde::Deserialize;

#[derive(Deserialize)]
struct Negations {
    words: Vec<String>,
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, sentimentos-rust!");
}

fn get_negations() -> Negations {
    let prefix = Path::new("src/lang/pt-br/negations.json");
    let file = std::fs::File::open(prefix).expect("file should open read only");

    let json: Negations = serde_json::from_reader(file).expect("file should be proper JSON");

    json
}

fn get_dictionary() -> serde_json::value::Value {
    let prefix = Path::new("src/lang/pt-br/afinn.json");
    let file = std::fs::File::open(prefix).expect("file should open read only");

    let json: serde_json::Value =
        serde_json::from_reader(file).expect("file should de proper JSON");

    json
}

#[cfg(test)]
mod test {
    use super::{get_dictionary, get_negations};
    use crate::tokenize::tokenize;
    use rust_stemmers::{Algorithm, Stemmer};
    use std::collections::HashSet;
    use stopwords::{Language, Spark, Stopwords};
    use serde_json::json;

    #[test]
    fn basico() {
        let pt_stemmer = Stemmer::create(Algorithm::Portuguese);
        let stops: HashSet<_> = Spark::stopwords(Language::Portuguese)
            .unwrap()
            .iter()
            .collect();

        let frase = "Eu não odeio a minha vida...";
        let tokens = tokenize(&frase);

        assert_eq!(tokens, ["eu", "não", "odeio", "a", "minha", "vida"]);

        let tokens: Vec<String> = tokens
            .iter()
            .map(|s| pt_stemmer.stem(s).into_owned())
            .collect();

        assert_eq!(tokens, ["eu", "nã", "odei", "a", "minh", "vid"]);

        let mut tokens: Vec<&str> = tokens.iter().map(|s| &**s).collect();

        tokens.retain(|s| !stops.contains(s));

        assert_eq!(tokens, ["nã", "odei", "minh", "vid"]);

        let negations = get_negations().words;

        let dictionary = get_dictionary();

        let mut score = 0;
        let mut negator = 1;
        let mut num_hits = 0;
        // print!("{:?}", "não".co);
        // for negation in negations {
        //     // print!("{}", negation);
        // }

        for token in tokens {
            let index = negations.iter().position(|r| r.contains(token));
            println!("Palavra: {}", token);
            match index {
                Some(_) => {
                    negator = -1;
                    num_hits += 1;
                }
                None => {
                    let score_value = dictionary
                        .as_object()
                        .unwrap()
                        .iter()
                        .find(|(key, _)| key.contains(token));
                    match score_value {
                        Some((_, value)) => {
                            let valor = value.as_i64().unwrap_or(0);
                            print!("Valor: {}", valor);
                            score += negator * valor;
                            num_hits += 1;
                        }
                        None => {}
                    };
                }
            }
        }
        print!("{:?}", json!({
            "score": score,
            "numhits": num_hits
        }));
    }
}
