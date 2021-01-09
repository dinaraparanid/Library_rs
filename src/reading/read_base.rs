extern crate yaml_rust;
use self::yaml_rust::yaml::Hash;
use self::yaml_rust::{Yaml, YamlEmitter, YamlLoader};
use crate::books::book::Book;
use crate::books::ResultSelf;
use crate::reading::reader::Reader;
use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{Read, Write};
use std::rc::Rc;

/// Reader Base structure,
/// which contains only readers

pub struct ReaderBase {
    pub(crate) readers: Vec<Rc<RefCell<Reader>>>,
}

/// Print for Reader Base.
/// It used to debug code

impl Debug for ReaderBase {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Reader Base")
            .field(
                "readers",
                &self
                    .readers
                    .iter()
                    .map(|x| format!("{:?}", *(**x).borrow()))
                    .collect::<Vec<String>>(),
            )
            .finish()
    }
}

impl ReaderBase {
    /// Creates empty Reader Base

    #[inline]
    pub const fn new() -> Self {
        ReaderBase { readers: vec![] }
    }

    /// Searches reader by his params.
    /// If ok returns index,
    /// else returns amount of readers

    #[inline]
    pub fn find_reader(
        &self,
        name: &String,
        family: &String,
        father: &String,
        age: u8,
    ) -> Option<usize> {
        self.readers.iter().position(|x| {
            (**x).borrow().name == *name
                && (**x).borrow().family == *family
                && (**x).borrow().father == *father
                && (**x).borrow().age == age
        })
    }

    /// Adds reader by params.
    /// No checks provided

    #[inline]
    pub unsafe fn add_reader_unchecked(
        &mut self,
        name: String,
        family: String,
        father: String,
        age: u8,
    ) -> &mut Self {
        self.readers.push(Rc::new(RefCell::new(Reader::new(
            name, family, father, age,
        ))));
        self
    }

    /// Adds reader by params.
    /// If reader with same params exists,
    /// it will report error

    #[inline]
    pub fn add_reader(
        &mut self,
        name: String,
        family: String,
        father: String,
        age: u8,
    ) -> ResultSelf<Self> {
        return if !self.readers.is_empty()
            && self.find_reader(&name, &family, &father, age).is_some()
        {
            Err(0) // already exists
        } else {
            Ok(unsafe { self.add_reader_unchecked(name, family, father, age) })
        };
    }

    /// Removes reader by params.
    /// No checks provided

    #[inline]
    pub unsafe fn remove_reader_unchecked(&mut self, ind: usize) -> &mut Self {
        (**self.readers.get_unchecked_mut(ind))
            .borrow_mut()
            .remove_all_books();
        self.readers.remove(ind);
        self
    }

    /// Removes reader by params.
    /// If reader with same params isn't found,
    /// it will report error

    #[inline]
    pub fn remove_reader(&mut self, ind: usize) -> ResultSelf<Self> {
        return if ind >= self.readers.len() {
            Err(0) // out of range
        } else {
            Ok(unsafe { self.remove_reader_unchecked(ind) })
        };
    }

    /// Changes reader's name.
    /// No checks provided

    pub unsafe fn change_name_unchecked(&mut self, ind: usize, new_name: String) -> &mut Self {
        (**self.readers.get_unchecked_mut(ind))
            .borrow_mut()
            .change_name(new_name)
            .unwrap();
        self
    }

    /// Changes reader's name.
    /// If reader with same params isn't found,
    /// it will report error

    #[inline]
    pub fn change_name(&mut self, ind: usize, new_name: String) -> ResultSelf<Self> {
        return if ind >= self.readers.len() {
            Err(0) // out of range
        } else {
            unsafe {
                if self
                    .find_reader(
                        &new_name,
                        &RefCell::borrow(&(**self.readers.get_unchecked(ind))).family,
                        &RefCell::borrow(&(**self.readers.get_unchecked(ind))).father,
                        RefCell::borrow(&(**self.readers.get_unchecked(ind))).age,
                    )
                    .is_some()
                {
                    Err(1) // already exists
                } else {
                    Ok(self.change_name_unchecked(ind, new_name))
                }
            }
        };
    }

    /// Changes reader's 2-nd name.
    /// No checks provided

    #[inline]
    pub unsafe fn change_family_unchecked(&mut self, ind: usize, new_family: String) -> &mut Self {
        (**self.readers.get_unchecked_mut(ind))
            .borrow_mut()
            .change_family(new_family)
            .unwrap();
        self
    }

    /// Changes reader's 2-nd name.
    /// If reader with same params isn't found,
    /// it will report error

    #[inline]
    pub fn change_family(&mut self, ind: usize, new_family: String) -> ResultSelf<Self> {
        return if ind >= self.readers.len() {
            Err(0) // out of range
        } else {
            unsafe {
                if self
                    .find_reader(
                        &RefCell::borrow(&(**self.readers.get_unchecked(ind))).name,
                        &new_family,
                        &RefCell::borrow(&(**self.readers.get_unchecked(ind))).father,
                        RefCell::borrow(&(**self.readers.get_unchecked(ind))).age,
                    )
                    .is_some()
                {
                    Err(1) // already exists
                } else {
                    Ok(self.change_family_unchecked(ind, new_family))
                }
            }
        };
    }

    /// Changes reader's mid. name.
    /// No checks provided

    #[inline]
    pub unsafe fn change_father_unchecked(&mut self, ind: usize, new_father: String) -> &mut Self {
        (**self.readers.get_unchecked_mut(ind))
            .borrow_mut()
            .change_father(new_father)
            .unwrap();
        self
    }

    /// Changes reader's mid. name.
    /// If reader with same params isn't found,
    /// it will report error

    #[inline]
    pub fn change_father(&mut self, ind: usize, new_father: String) -> ResultSelf<Self> {
        return if ind >= self.readers.len() {
            Err(0) // out of range
        } else {
            unsafe {
                if self
                    .find_reader(
                        &RefCell::borrow(&(**self.readers.get_unchecked(ind))).name,
                        &RefCell::borrow(&(**self.readers.get_unchecked(ind))).family,
                        &new_father,
                        RefCell::borrow(&(**self.readers.get_unchecked(ind))).age,
                    )
                    .is_some()
                {
                    Err(1) // already exists
                } else {
                    Ok(self.change_father_unchecked(ind, new_father))
                }
            }
        };
    }

    /// Changes reader's age.
    /// No checks provided

    #[inline]
    pub unsafe fn change_age_unchecked(&mut self, ind: usize, new_age: u8) -> &mut Self {
        (**self.readers.get_unchecked_mut(ind))
            .borrow_mut()
            .change_age(new_age);
        self
    }

    /// Changes reader's age.
    /// If reader with same params isn't found,
    /// it will report error

    #[inline]
    pub fn change_age(&mut self, ind: usize, new_age: String) -> ResultSelf<Self> {
        let new_age_num;

        match new_age.trim().parse::<u8>() {
            Ok(x) => new_age_num = x,
            Err(_) => return Err(0), // parse error
        }

        return if ind >= self.readers.len() {
            Err(1) // out of range
        } else {
            unsafe {
                if self
                    .find_reader(
                        &RefCell::borrow(&(**self.readers.get_unchecked(ind))).name,
                        &RefCell::borrow(&(**self.readers.get_unchecked(ind))).family,
                        &RefCell::borrow(&(**self.readers.get_unchecked(ind))).father,
                        new_age_num,
                    )
                    .is_some()
                {
                    Err(2) // already exists
                } else {
                    Ok(self.change_age_unchecked(ind, new_age_num))
                }
            }
        };
    }

    /// Returns amount of readers

    #[inline]
    pub fn len(&self) -> usize {
        self.readers.len()
    }

    /// Saves everything to .yaml file

    #[inline]
    pub(crate) fn save(&self) {
        let mut array = yaml_rust::yaml::Array::new();

        for guy in 0..self.readers.len() {
            let mut data = Hash::new();

            unsafe {
                data.insert(Yaml::String("№".to_string()), Yaml::Integer(guy as i64 + 1));

                data.insert(
                    Yaml::String("Name".to_string()),
                    Yaml::String(
                        RefCell::borrow(&(**self.readers.get_unchecked(guy)))
                            .name
                            .clone(),
                    ),
                );

                data.insert(
                    Yaml::String("Family".to_string()),
                    Yaml::String(
                        RefCell::borrow(&(**self.readers.get_unchecked(guy)))
                            .family
                            .clone(),
                    ),
                );

                data.insert(
                    Yaml::String("Father".to_string()),
                    Yaml::String(
                        RefCell::borrow(&(**self.readers.get_unchecked(guy)))
                            .father
                            .clone(),
                    ),
                );

                data.insert(
                    Yaml::String("Age".to_string()),
                    Yaml::Integer(RefCell::borrow(&(**self.readers.get_unchecked(guy))).age as i64),
                );

                data.insert(
                    Yaml::String("Reading".to_string()),
                    Yaml::String(
                        if RefCell::borrow(&(**self.readers.get_unchecked(guy)))
                            .reading
                            .is_some()
                        {
                            format!(
                                "{} {} {}",
                                (*(RefCell::borrow(&(**self.readers.get_unchecked(guy)))
                                    .reading
                                    .as_ref()
                                    .unwrap())
                                .upgrade()
                                .unwrap())
                                .borrow()
                                .title,
                                (*(RefCell::borrow(&(**self.readers.get_unchecked(guy)))
                                    .reading
                                    .as_ref()
                                    .unwrap())
                                .upgrade()
                                .unwrap())
                                .borrow()
                                .author,
                                (*(RefCell::borrow(&(**self.readers.get_unchecked(guy)))
                                    .reading
                                    .as_ref()
                                    .unwrap())
                                .upgrade()
                                .unwrap())
                                .borrow()
                                .pages
                            )
                        } else {
                            "None".to_string()
                        },
                    ),
                );
            }

            array.push(Yaml::Hash(data));
        }

        let mut string = String::new();
        let mut emitter = YamlEmitter::new(&mut string);
        emitter.dump(&Yaml::Array(array)).unwrap();

        let mut file = File::create("readers.yaml").unwrap();
        file.write_all(string.as_bytes()).unwrap();
    }

    /// Loads everything from .yaml file

    #[inline]
    pub fn load(&mut self) {
        let mut file = File::open("readers.yaml").unwrap();
        let mut string = String::new();
        file.read_to_string(&mut string).unwrap();

        if !string.is_empty() {
            let docs = YamlLoader::load_from_str(string.as_str()).unwrap();
            let doc = docs.first().unwrap().clone().into_vec().unwrap();

            for d in doc {
                self.readers.push(Rc::new(RefCell::new(Reader::new(
                    d["Name"].as_str().unwrap().to_string(),
                    d["Family"].as_str().unwrap().to_string(),
                    d["Father"].as_str().unwrap().to_string(),
                    d["Age"].as_i64().unwrap() as u8,
                ))));

                (*self.readers.last_mut().unwrap()).borrow_mut().reading =
                    if d["Reading"].as_str().unwrap() == "None" {
                        None
                    } else {
                        Some(Rc::downgrade(&Rc::new(RefCell::new(Book::new(
                            "".to_string(),
                            "".to_string(),
                            0,
                        )))))
                    }
            }
        }
    }
}