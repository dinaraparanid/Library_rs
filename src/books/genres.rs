extern crate yaml_rust;

use std::{
    collections::{BTreeSet, HashSet},
    fs::File,
    io::{Read, Write},
    iter::FromIterator,
};

use yaml_rust::{yaml::Array, Yaml, YamlEmitter, YamlLoader};

/// All existing genres.yaml of books

#[derive(Debug, Default, Clone)]
pub struct Genres {
    pub(crate) genres: HashSet<String>,
}

impl Genres {
    /// Creates empty set of genres.yaml

    #[inline]
    pub fn new() -> Self {
        Genres::default()
    }

    /// Adds new genre to set.
    /// If this genre is already exists,
    /// it will return false
    /// else true

    #[inline]
    pub fn add(&mut self, new_genre: String) -> bool {
        self.genres.insert(new_genre.to_lowercase())
    }

    /// Removes genre from set.
    /// If this genre is found,
    /// it will return true
    /// else false

    #[inline]
    pub fn remove(&mut self, genre: &String) -> bool {
        self.genres.remove(genre.to_lowercase().as_str())
    }
    
    /// Deletes all genres
    
    #[inline]
    pub(crate) fn clear(&mut self) -> &mut Self {
        self.genres.clear();
        self
    }

    /// Saves all genres to yaml file

    #[inline]
    pub fn save(&self) {
        let mut array = Array::new();

        if self.genres.is_empty() {
            array.push(Yaml::String("None".to_string()));
        } else {
            for g in &self.genres {
                array.push(Yaml::String(g.clone()))
            }
        }

        let mut string = String::new();
        let mut emitter = YamlEmitter::new(&mut string);

        let mut hash = yaml_rust::yaml::Hash::new();
        hash.insert(Yaml::String("Genres".to_string()), Yaml::Array(array));
        emitter.dump(&Yaml::Hash(hash)).unwrap();

        let mut file = File::create("src/utils/genres.yaml").unwrap();
        file.write_all(string.as_bytes()).unwrap();
    }

    /// Loads from yaml file

    #[inline]
    pub fn load(&mut self) {
        let mut file = File::open("src/utils/genres.yaml").unwrap();
        let mut string = String::new();
        file.read_to_string(&mut string).unwrap();

        if !string.is_empty() {
            let docs = YamlLoader::load_from_str(string.as_str()).unwrap();
            let doc = docs.first().unwrap()["Genres"].as_vec().unwrap();

            if doc.first().unwrap().as_str().unwrap().to_string() != "None" {
                self.genres = doc
                    .iter()
                    .map(|x| x.as_str().unwrap().to_string())
                    .collect();
            }
        }
    }
}

impl FromIterator<String> for Genres {
    /// Constructs set of genres from iterator of strings

    #[inline]
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        Genres {
            genres: HashSet::from_iter(iter.into_iter().map(|x| x.to_lowercase())),
        }
    }
}

impl From<Vec<String>> for Genres {
    /// Constructs set of genres from vector of strings

    #[inline]
    fn from(vec: Vec<String>) -> Self {
        Genres::from_iter(vec.into_iter())
    }
}

impl From<HashSet<String>> for Genres {
    /// Constructs set of genres from vector of strings

    #[inline]
    fn from(set: HashSet<String>) -> Self {
        Genres::from_iter(set.into_iter())
    }
}

impl From<BTreeSet<String>> for Genres {
    /// Constructs set of genres from vector of strings

    #[inline]
    fn from(set: BTreeSet<String>) -> Self {
        Genres::from_iter(set.into_iter())
    }
}
