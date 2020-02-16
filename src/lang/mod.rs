// @todo remove this test module, after version 1
#[cfg(test)]
mod test {
    use rust_stemmers::{Algorithm, Stemmer};
    use std::collections::HashSet;
    use stopwords::{Spark, Language, Stopwords};

    #[test]
    fn pt_stemming() {
        let pt_stemmer = Stemmer::create(Algorithm::Portuguese);

        assert_eq!(pt_stemmer.stem("verdadeiramente"), "verdadeir");
        assert_eq!(pt_stemmer.stem("desvalorização"), "desvaloriz");
        assert_eq!(pt_stemmer.stem("ajudar"), "ajud");
        assert_eq!(pt_stemmer.stem("desagradável"), "desagrad");
    }

    #[test]
    fn rm_stopwords() {
        let stops: HashSet<_> = Spark::stopwords(Language::Portuguese).unwrap().iter().collect();
        let mut tokens = vec!("coco", "e", "merda");
        tokens.retain(|s| !stops.contains(s));
        assert_eq!(tokens, vec!("coco", "merda"));
    }
}