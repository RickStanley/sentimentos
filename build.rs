use rust_stemmers::{Algorithm, Stemmer};
use serde_json::{json, Value};
use std::{
    error::Error,
    fs::{read_dir, DirEntry, File},
    io::{BufReader, Write},
    path::{Path, PathBuf},
};

const LANG_PATH: &'static str = "src/lang";

fn read_dictionary_from_file<P: AsRef<Path>>(path: P) -> Result<Value, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file.
    let u = serde_json::from_reader(reader)?;

    // Return the Value
    Ok(u)
}

fn write_stemmed_diciotnary(contents: String, folder_path: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(folder_path.to_owned() + "/afinn_stemmed.json")?;
    file.write_all(&contents.into_bytes())?;
    Ok(())
}

fn get_language_folders() -> Result<Vec<String>, Box<dyn Error>> {
    let entries = read_dir(LANG_PATH)?
        .filter_map(|entry: Result<DirEntry, _>| entry.ok().and_then(|e| Some(e.path())))
        .filter(|path: &PathBuf| path.is_dir())
        .filter_map(|path: PathBuf| {
            path.file_name()
                .and_then(|name| name.to_str().map(|string| String::from(string)))
        })
        .collect::<Vec<String>>();
    Ok(entries)
}

fn identify_language(prefix: &str) -> Result<Algorithm, &'static str> {
    match prefix {
        "pt-br" => Ok(Algorithm::Portuguese),
        _ => unimplemented!(),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    for folder in get_language_folders()? {
        let folder_full_path = LANG_PATH.to_owned() + "/" + &folder;

        let dictionary = read_dictionary_from_file(folder_full_path.to_owned() + "/afinn.json")?;
        let stem_lang = identify_language(&folder)?;

        let stemmer = Stemmer::create(stem_lang);

        let iterable_dictionary = dictionary.as_object().unwrap();

        let mut stemmed_dictionary = json!({});

        for (key, value) in iterable_dictionary {
            stemmed_dictionary[stemmer.stem(key).to_string()] = value.to_owned();
        }

        write_stemmed_diciotnary(stemmed_dictionary.to_string(), &folder_full_path)
            .expect("Didn't save the stemmed dictionary.");
    }

    Ok(())
}
