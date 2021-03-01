extern crate yaml_rust;

use std::{
    collections::{BTreeSet, HashSet},
    fs::File,
    io::{Read, Write},
    iter::FromIterator,
};

use std::collections::hash_set::Iter;
use yaml_rust::{yaml::Array, Yaml, YamlEmitter, YamlLoader};

/// All existing genres.yaml of books

#[derive(Debug, Default, Clone)]
pub struct Genres {
    pub(crate) genres: HashSet<String>,
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

impl IntoIterator for Genres {
    type Item = String;
    type IntoIter = std::collections::hash_set::IntoIter<Self::Item>;

    /// Convert genres to iterator

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.genres.into_iter()
    }
}

impl From<Vec<String>> for Genres {
    /// Constructs genres from vector

    #[inline]
    fn from(vec: Vec<String>) -> Self {
        Genres::from_iter(vec.into_iter())
    }
}

impl From<Genres> for Vec<String> {
    /// Constructs vector from genres

    #[inline]
    fn from(g: Genres) -> Self {
        g.into_iter().collect()
    }
}

impl From<HashSet<String>> for Genres {
    /// Constructs genres from hash set

    #[inline]
    fn from(set: HashSet<String>) -> Self {
        Genres::from_iter(set.into_iter())
    }
}

impl From<Genres> for HashSet<String> {
    /// Constructs hash set from genres

    #[inline]
    fn from(g: Genres) -> Self {
        g.genres
    }
}

impl From<BTreeSet<String>> for Genres {
    /// Constructs genres from btree set

    #[inline]
    fn from(set: BTreeSet<String>) -> Self {
        Genres::from_iter(set.into_iter())
    }
}

impl From<Genres> for BTreeSet<String> {
    /// Constructs btree set from genres

    #[inline]
    fn from(g: Genres) -> Self {
        g.into_iter().collect()
    }
}

impl Genres {
    /// Creates empty set of genres.yaml

    #[inline]
    pub fn new() -> Self {
        Genres::default()
    }

    /// iterator over Genres

    #[inline]
    pub fn iter(&self) -> Iter<String> {
        self.genres.iter()
    }

    /// Adds new genre to set.
    /// If this genre is already exists,
    /// it will return false
    /// else true

    #[inline]
    pub(crate) fn add(&mut self, new_genre: String) -> bool {
        self.genres.insert(new_genre.to_lowercase())
    }

    /// Removes genre from set.
    /// If this genre is found,
    /// it will return true
    /// else false

    #[inline]
    pub(crate) fn remove(&mut self, genre: &String) -> bool {
        self.genres.remove(genre.to_lowercase().as_str())
    }

    /// Deletes all genres from current genres keeper

    #[inline]
    pub(crate) fn clear(&mut self) -> &mut Self {
        self.genres.clear();
        self
    }

    /// Saves all genres to yaml file

    #[inline]
    pub(crate) fn save(&self) {
        let mut array = Array::new();

        if self.genres.is_empty() {
            array.push(Yaml::String("None".to_string()));
        } else {
            self.genres
                .iter()
                .for_each(|g| array.push(Yaml::String(g.clone())));
        }

        let mut string = String::new();
        let mut emitter = YamlEmitter::new(&mut string);

        let mut hash = yaml_rust::yaml::Hash::new();
        hash.insert(Yaml::String("Genres".to_string()), Yaml::Array(array));
        emitter.dump(&Yaml::Hash(hash)).unwrap();

        File::create("src/utils/genres.yaml")
            .unwrap()
            .write_all(string.as_bytes())
            .unwrap();
    }

    /// Loads from yaml file

    #[inline]
    pub fn load(&mut self) {
        let mut string = String::new();

        File::open("src/utils/genres.yaml")
            .unwrap()
            .read_to_string(&mut string)
            .unwrap();

        if !string.is_empty() {
            let doc = YamlLoader::load_from_str(string.as_str())
                .unwrap()
                .first()
                .unwrap()["Genres"]
                .as_vec()
                .unwrap()
                .clone();

            if doc.first().unwrap().as_str().unwrap().to_string() != "None" {
                self.genres = doc
                    .iter()
                    .map(|x| x.as_str().unwrap().to_string())
                    .collect();
            }
        }
    }
}
