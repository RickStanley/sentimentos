use std::str::FromStr;

pub enum Language {
    Portuguese,
}


impl FromStr for Language {
    type Err = ();

    fn from_str(s: &str) -> Result<Language, ()> {
        match s {
            "portuguese" => Ok(Language::Portuguese),
            _ => Err(()),
        }
    }
}

// @todo Every file should be zippe (gzip)
// Portuguese
const PT_AFINN: &str = include_str!("pt-br/afinn.json");
const PT_NEG: &str = include_str!("pt-br/negations.json");

pub fn get_directives(lang: Language) -> (&'static str, &'static str) {
    match lang {
        Language::Portuguese => (PT_AFINN, PT_NEG),
    }
}
