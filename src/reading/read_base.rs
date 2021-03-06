extern crate yaml_rust;

use crate::{
    books::{book::Book, date::Date, ResultSelf},
    reading::reader::Reader,
};

use std::{
    cell::RefCell,
    fmt::{Debug, Formatter},
    fs::File,
    io::{Read, Write},
    iter::FromIterator,
    rc::{Rc, Weak},
};

use yaml_rust::{yaml::Hash, Yaml, YamlEmitter, YamlLoader};

/// Reader Base structure,
/// which contains only readers

pub struct ReaderBase {
    pub(crate) readers: Vec<Rc<RefCell<Reader>>>,
}

impl Debug for ReaderBase {
    /// Print for Reader Base.
    /// It used to debug code

    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Reader Base")
            .field(
                "readers",
                &self
                    .iter()
                    .map(|x| format!("{:?}", *(**x).borrow()))
                    .collect::<Vec<_>>(),
            )
            .finish()
    }
}

impl Clone for ReaderBase {
    /// Clones Reader Base
    /// with new readers' smart pointers

    #[inline]
    fn clone(&self) -> Self {
        ReaderBase {
            readers: self
                .iter()
                .map(|x| Rc::new(RefCell::new((**x).borrow().clone())))
                .collect(),
        }
    }
}

impl IntoIterator for ReaderBase {
    type Item = Rc<RefCell<Reader>>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    /// Converts Reader Base to iterator

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.readers.into_iter()
    }
}

impl FromIterator<Rc<RefCell<Reader>>> for ReaderBase {
    /// Create Reader Base from iterator

    #[inline]
    fn from_iter<T: IntoIterator<Item = Rc<RefCell<Reader>>>>(iter: T) -> Self {
        ReaderBase {
            readers: iter.into_iter().collect(),
        }
    }
}

impl ReaderBase {
    /// Creates empty Reader Base

    #[inline]
    pub const fn new() -> Self {
        ReaderBase { readers: vec![] }
    }

    /// Iterate on Book System with smart pointers of The Book

    #[inline]
    pub(crate) fn iter(&self) -> std::slice::Iter<Rc<RefCell<Reader>>> {
        self.readers.iter()
    }

    /// Returns amount of readers

    #[inline]
    pub fn len(&self) -> usize {
        self.readers.len()
    }

    /// Searches reader by his params.
    /// If ok returns index,
    /// else returns amount of readers

    #[inline]
    pub(crate) fn find_reader(
        &self,
        name: &String,
        family: &String,
        father: &String,
        birth: Date,
    ) -> Option<usize> {
        self.iter().position(|x| {
            (**x).borrow().name == *name
                && (**x).borrow().family == *family
                && (**x).borrow().father == *father
                && (**x).borrow().birth == birth
        })
    }

    /// Adds reader by params
    /// in an ascending order.
    /// No checks provided

    #[inline]
    pub(crate) unsafe fn add_reader_unchecked(
        &mut self,
        name: String,
        family: String,
        father: String,
        info: String,
        birth: Date,
    ) -> &mut Self {
        let reader = Rc::new(RefCell::new(Reader::new(name, family, father, info, birth)));

        if self.readers.is_empty() {
            self.readers.push(reader);
        } else {
            self.readers.insert(
                self.readers
                    .binary_search_by(|r| {
                        format!(
                            "{} {} {} {}",
                            (**r).borrow().name,
                            (**r).borrow().family,
                            (**r).borrow().father,
                            (**r).borrow().birth.to_string()
                        )
                        .cmp(&format!(
                            "{} {} {} {}",
                            (*reader).borrow().name,
                            (*reader).borrow().family,
                            (*reader).borrow().father,
                            (*reader).borrow().birth.to_string()
                        ))
                    })
                    .unwrap_err(),
                reader,
            );
        }

        self
    }

    /// Adds reader by params in an ascending order.
    /// If reader with same params exists,
    /// it will report error

    #[inline]
    pub(crate) fn add_reader(
        &mut self,
        name: String,
        family: String,
        father: String,
        info: String,
        birth: Date,
    ) -> ResultSelf<Self> {
        return if !self.readers.is_empty()
            && self.find_reader(&name, &family, &father, birth).is_some()
        {
            Err(0) // already exists
        } else {
            Ok(unsafe { self.add_reader_unchecked(name, family, father, info, birth) })
        };
    }

    /// Removes reader by params.
    /// No checks provided

    #[inline]
    pub(crate) unsafe fn remove_reader_unchecked(&mut self, ind: usize) -> &mut Self {
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
    pub(crate) fn remove_reader(&mut self, ind: usize) -> ResultSelf<Self> {
        return if ind >= self.len() {
            Err(0) // out of range
        } else {
            Ok(unsafe { self.remove_reader_unchecked(ind) })
        };
    }

    /// Changes reader's name.
    /// No checks provided

    #[inline]
    pub(crate) unsafe fn change_name_unchecked(
        &mut self,
        ind: usize,
        new_name: String,
    ) -> &mut Self {
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
    pub(crate) fn change_name(&mut self, ind: usize, new_name: String) -> ResultSelf<Self> {
        return if ind >= self.len() {
            Err(0) // out of range
        } else {
            unsafe {
                if self
                    .find_reader(
                        &new_name,
                        &RefCell::borrow(&(**self.readers.get_unchecked(ind))).family,
                        &RefCell::borrow(&(**self.readers.get_unchecked(ind))).father,
                        RefCell::borrow(&(**self.readers.get_unchecked(ind))).birth,
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
    pub(crate) unsafe fn change_family_unchecked(
        &mut self,
        ind: usize,
        new_family: String,
    ) -> &mut Self {
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
    pub(crate) fn change_family(&mut self, ind: usize, new_family: String) -> ResultSelf<Self> {
        return if ind >= self.len() {
            Err(0) // out of range
        } else {
            unsafe {
                if self
                    .find_reader(
                        &RefCell::borrow(&(**self.readers.get_unchecked(ind))).name,
                        &new_family,
                        &RefCell::borrow(&(**self.readers.get_unchecked(ind))).father,
                        RefCell::borrow(&(**self.readers.get_unchecked(ind))).birth,
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
    pub(crate) unsafe fn change_father_unchecked(
        &mut self,
        ind: usize,
        new_father: String,
    ) -> &mut Self {
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
    pub(crate) fn change_father(&mut self, ind: usize, new_father: String) -> ResultSelf<Self> {
        return if ind >= self.len() {
            Err(0) // out of range
        } else {
            unsafe {
                if self
                    .find_reader(
                        &RefCell::borrow(&(**self.readers.get_unchecked(ind))).name,
                        &RefCell::borrow(&(**self.readers.get_unchecked(ind))).family,
                        &new_father,
                        RefCell::borrow(&(**self.readers.get_unchecked(ind))).birth,
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

    /// Changes reader's info.
    /// No checks provided

    #[inline]
    pub(crate) unsafe fn change_info_unchecked(
        &mut self,
        ind: usize,
        new_info: String,
    ) -> &mut Self {
        (**self.readers.get_unchecked_mut(ind))
            .borrow_mut()
            .change_info(new_info)
            .unwrap();
        self
    }

    /// Changes reader's info.
    /// If reader with same params isn't found,
    /// it will report error

    #[inline]
    pub(crate) fn change_info(&mut self, ind: usize, new_info: String) -> ResultSelf<Self> {
        return if ind >= self.len() {
            Err(0) // out of range
        } else {
            Ok(unsafe { self.change_info_unchecked(ind, new_info) })
        };
    }

    /// Changes reader's age.
    /// No checks provided

    #[inline]
    pub(crate) unsafe fn change_age_unchecked(&mut self, ind: usize, new_birth: Date) -> &mut Self {
        (**self.readers.get_unchecked_mut(ind))
            .borrow_mut()
            .change_age(new_birth);
        self
    }

    /// Changes reader's age.
    /// If reader with same params isn't found,
    /// it will report error

    #[inline]
    pub(crate) fn change_age(&mut self, ind: usize, new_birth: Date) -> ResultSelf<Self> {
        return if ind >= self.len() {
            Err(0) // out of range
        } else {
            unsafe {
                if self
                    .find_reader(
                        &RefCell::borrow(&(**self.readers.get_unchecked(ind))).name,
                        &RefCell::borrow(&(**self.readers.get_unchecked(ind))).family,
                        &RefCell::borrow(&(**self.readers.get_unchecked(ind))).father,
                        new_birth,
                    )
                    .is_some()
                {
                    Err(1) // already exists
                } else {
                    Ok(self.change_age_unchecked(ind, new_birth))
                }
            }
        };
    }

    /// Returns book (or None) which is read by reader now

    #[inline]
    pub fn get_book(&self, ind: usize) -> Option<Vec<Weak<RefCell<Book>>>> {
        return match unsafe { &(**self.readers.get_unchecked(ind)).borrow().reading } {
            None => None,
            Some(books) => Some(books.clone()),
        };
    }

    /// Deletes all readers from current Reade Base
    /// (But not readers themselves)

    #[inline]
    #[allow(dead_code)]
    pub(crate) fn clear(&mut self) -> &mut Self {
        self.readers.clear();
        self
    }

    /// Saves everything to .yaml file

    pub(crate) fn save(&self) {
        let mut array = yaml_rust::yaml::Array::new();

        (0..self.len()).for_each(|guy| {
            let mut data = Hash::new();

            unsafe {
                data.insert(Yaml::String("№".to_string()), Yaml::Integer(guy as i64 + 1));

                data.insert(
                    Yaml::String("Name".to_string()),
                    Yaml::String((**self.readers.get_unchecked(guy)).borrow().name.clone()),
                );

                data.insert(
                    Yaml::String("Family".to_string()),
                    Yaml::String((**self.readers.get_unchecked(guy)).borrow().family.clone()),
                );

                data.insert(
                    Yaml::String("Father".to_string()),
                    Yaml::String((**self.readers.get_unchecked(guy)).borrow().father.clone()),
                );

                data.insert(
                    Yaml::String("Info".to_string()),
                    Yaml::String((**self.readers.get_unchecked(guy)).borrow().info.clone()),
                );

                data.insert(
                    Yaml::String("Day".to_string()),
                    Yaml::Integer((**self.readers.get_unchecked(guy)).borrow().birth.day as i64),
                );

                data.insert(
                    Yaml::String("Month".to_string()),
                    Yaml::Integer(
                        RefCell::borrow(&(**self.readers.get_unchecked(guy)))
                            .birth
                            .month as i64,
                    ),
                );

                data.insert(
                    Yaml::String("Year".to_string()),
                    Yaml::Integer(
                        RefCell::borrow(&(**self.readers.get_unchecked(guy)))
                            .birth
                            .year as i64,
                    ),
                );

                data.insert(
                    Yaml::String("Reading".to_string()),
                    Yaml::Array(
                        if RefCell::borrow(&(**self.readers.get_unchecked(guy)))
                            .reading
                            .is_some()
                        {
                            (**self.readers.get_unchecked(guy))
                                .borrow()
                                .reading
                                .as_ref()
                                .unwrap()
                                .iter()
                                .map(|b| {
                                    Yaml::String(format!(
                                        "{} {} {}",
                                        (*b.upgrade().unwrap()).borrow().title(),
                                        (*b.upgrade().unwrap()).borrow().author(),
                                        (*b.upgrade().unwrap()).borrow().pages()
                                    ))
                                })
                                .collect()
                        } else {
                            vec![Yaml::String("None".to_string())]
                        },
                    ),
                );
            }

            array.push(Yaml::Hash(data));
        });

        let mut string = String::new();
        let mut emitter = YamlEmitter::new(&mut string);
        emitter.dump(&Yaml::Array(array)).unwrap();

        File::create("src/utils/readers.yaml")
            .unwrap()
            .write_all(string.as_bytes())
            .unwrap();
    }

    /// Loads everything from .yaml file

    #[inline]
    pub fn load(&mut self) {
        let mut file = File::open("src/utils/readers.yaml").unwrap();
        let mut string = String::new();
        file.read_to_string(&mut string).unwrap();

        if !string.is_empty() {
            let doc = YamlLoader::load_from_str(string.as_str())
                .unwrap()
                .first()
                .unwrap()
                .clone()
                .into_vec()
                .unwrap();

            doc.into_iter().for_each(|d| {
                self.readers.push(Rc::new(RefCell::new(Reader::new(
                    d["Name"].as_str().unwrap().to_string(),
                    d["Family"].as_str().unwrap().to_string(),
                    d["Father"].as_str().unwrap().to_string(),
                    d["Info"].as_str().unwrap().to_string(),
                    Date::new(
                        d["Day"].as_i64().unwrap() as u8,
                        d["Month"].as_i64().unwrap() as u8,
                        d["Year"].as_i64().unwrap() as u16,
                    )
                    .unwrap(),
                ))));

                (*self.readers.last_mut().unwrap()).borrow_mut().reading = None;
            });
        }
    }
}
