extern crate yaml_rust;

use crate::{
    books::{date::Date, the_book::TheBook, BookInterface, ResultSelf},
    reading::{read_base::ReaderBase, reader::Reader},
};

use std::{
    borrow::*,
    cell::RefCell,
    collections::HashSet,
    fmt::{Debug, Formatter},
    fs::File,
    io::{Read, Write},
    iter::FromIterator,
    rc::{Rc, Weak},
};

use yaml_rust::{
    yaml::{Array, Hash},
    Yaml, YamlEmitter, YamlLoader,
};

/// Reader Base structure,
/// which contains only Book interfaces

pub struct BookSystem {
    pub(crate) books: Vec<Rc<RefCell<TheBook>>>,
}

/// Print for BookSystem.
/// It is used for debug code

impl Debug for BookSystem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Book System")
            .field(
                "books",
                &self
                    .books
                    .iter()
                    .map(|x| format!("{:?}", *(**x).borrow()))
                    .collect::<Vec<String>>(),
            )
            .finish()
    }
}

impl BookSystem {
    /// Constructs empty Book System

    #[inline]
    pub const fn new() -> Self {
        BookSystem { books: vec![] }
    }

    /// Finds The Book.
    /// If book is not found, it' ll return TheBooks amount

    #[inline]
    pub fn find_book(&self, title: &String, author: &String, pages: u16) -> Option<usize> {
        self.books.iter().position(|x| {
            (**x).borrow().title == *title
                && (**x).borrow().author == *author
                && (**x).borrow().pages == pages
        })
    }

    /// Adds simple books without any checks

    #[inline]
    pub unsafe fn add_books_unchecked(&mut self, ind: usize, amount: usize) -> ResultSelf<Self> {
        for _ in 0..amount {
            (**self.books.get_unchecked_mut(ind))
                .borrow_mut()
                .add_book();
        }

        Ok(self)
    }

    /// Adds simple books with strong guarantee

    #[inline]
    pub fn add_books(&mut self, ind: usize, amount: usize) -> ResultSelf<Self> {
        return if ind >= self.books.len() {
            Err(1) // out of range
        } else {
            unsafe {
                let size = RefCell::borrow(&(**self.books.get_unchecked(ind)))
                    .books
                    .len() as u128;

                if size + amount as u128 > usize::MAX as u128 {
                    return Err(0); // too much books
                }
                self.add_books_unchecked(ind, amount)
            }
        };
    }

    /// Adds new TheBook and **ONE** simple
    /// (I think it's logical)
    /// No checks provided

    #[inline]
    pub unsafe fn add_book_unchecked(
        &mut self,
        title: String,
        author: String,
        pages: u16,
    ) -> &mut Self {
        self.books
            .push(Rc::new(RefCell::new(TheBook::new(title, author, pages))));
        self
    }

    /// Adds new TheBook and **ONE** simple
    /// (I think it's logical)

    #[inline]
    pub fn add_book(&mut self, title: String, author: String, pages: u16) -> ResultSelf<Self> {
        return if !self.books.is_empty() && self.find_book(&title, &author, pages).is_some() {
            Err(0) // already exists
        } else {
            Ok(unsafe { self.add_book_unchecked(title, author, pages) })
        };
    }

    /// Remove one simple book by index without any checks

    #[inline]
    pub unsafe fn remove_one_book_unchecked(&mut self, ind: usize, rind: usize) {
        (**self.books.get_unchecked_mut(ind))
            .borrow_mut()
            .remove_book(rind)
            .unwrap();
    }

    /// Remove one simple book by index

    #[inline]
    pub fn remove_one_book(&mut self, ind: usize, rind: usize) -> ResultSelf<Self> {
        return if ind >= self.books.len() {
            Err(0) // search ind (TheBook) out of range
        } else {
            unsafe {
                if rind
                    >= RefCell::borrow(&(**self.books.get_unchecked(ind)))
                        .books
                        .len()
                {
                    return Err(1); // remove ind (simple) out of range
                }

                self.remove_one_book_unchecked(ind, rind);
            }
            Ok(self)
        };
    }

    /// Removes TheBook and all simple books without any checks

    #[inline]
    pub unsafe fn remove_book_unchecked(&mut self, ind: usize) -> &mut Self {
        (**self.books.get_unchecked(ind))
            .borrow_mut()
            .remove_all_books();

        self.books.remove(ind);
        self
    }

    /// Removes TheBook and all simple books

    #[inline]
    pub fn remove_book(&mut self, ind: usize) -> ResultSelf<Self> {
        return if ind >= self.books.len() {
            Err(0) // out of range
        } else {
            Ok(unsafe { self.remove_book_unchecked(ind) })
        };
    }

    /// Changes TheBook's and all simple books' title without any checks

    #[inline]
    pub unsafe fn change_title_unchecked(&mut self, ind: usize, new_title: String) -> &mut Self {
        (**self.books.get_unchecked_mut(ind))
            .borrow_mut()
            .change_title(new_title);
        self
    }

    /// Changes TheBook's and all simple books' title

    #[inline]
    pub fn change_title(&mut self, ind: usize, new_title: String) -> ResultSelf<Self> {
        return if ind >= self.books.len() {
            Err(0) // out of range
        } else {
            unsafe {
                if self
                    .find_book(
                        &new_title,
                        &RefCell::borrow(&(**self.books.get_unchecked(ind))).author,
                        RefCell::borrow(&(**self.books.get_unchecked(ind))).pages,
                    )
                    .is_some()
                {
                    Err(1) // already exists
                } else {
                    Ok(self.change_title_unchecked(ind, new_title))
                }
            }
        };
    }

    /// Changes TheBook's and all simple books' title without any checks

    #[inline]
    pub unsafe fn change_author_unchecked(&mut self, ind: usize, new_author: String) -> &mut Self {
        (**self.books.get_unchecked_mut(ind))
            .borrow_mut()
            .change_author(new_author);
        self
    }

    /// Changes TheBook's and all simple books' title

    #[inline]
    pub fn change_author(&mut self, ind: usize, new_author: String) -> ResultSelf<Self> {
        return if ind >= self.books.len() {
            Err(0) // out of range
        } else {
            unsafe {
                if self
                    .find_book(
                        &RefCell::borrow(&(**self.books.get_unchecked(ind))).title,
                        &new_author,
                        RefCell::borrow(&(**self.books.get_unchecked(ind))).pages,
                    )
                    .is_some()
                {
                    Err(1) // already exists
                } else {
                    Ok(self.change_author_unchecked(ind, new_author))
                }
            }
        };
    }

    /// Changes TheBook's and all simple books' title without any checks

    #[inline]
    pub unsafe fn change_pages_unchecked(&mut self, ind: usize, new_pages: u16) -> &mut Self {
        (**self.books.get_unchecked_mut(ind))
            .borrow_mut()
            .change_pages(new_pages);
        self
    }

    /// Changes TheBook's and all simple books' title

    #[inline]
    pub fn change_pages(&mut self, ind: usize, new_pages: String) -> ResultSelf<Self> {
        let new_pages_num;

        match new_pages.trim().parse::<u16>() {
            Ok(x) => new_pages_num = x,
            Err(_) => return Err(0), // parse error
        }

        return if ind == self.books.len() {
            Err(1) // not found
        } else {
            unsafe {
                if self
                    .find_book(
                        &RefCell::borrow(&(**self.books.get_unchecked(ind))).title,
                        &RefCell::borrow(&(**self.books.get_unchecked(ind))).author,
                        new_pages_num,
                    )
                    .is_some()
                {
                    Err(2) // already exists
                } else {
                    Ok(self.change_pages_unchecked(ind, new_pages_num))
                }
            }
        };
    }

    /// Save to .yaml file

    #[inline]
    pub fn save(&self) {
        let mut array = Array::new();

        for book in 0..self.books.len() {
            let mut data = Hash::new();

            unsafe {
                data.insert(
                    Yaml::String("№".to_string()),
                    Yaml::Integer(book as i64 + 1),
                );

                data.insert(
                    Yaml::String("Title".to_string()),
                    Yaml::String(
                        RefCell::borrow(&(**self.books.get_unchecked(book)))
                            .title
                            .clone(),
                    ),
                );

                data.insert(
                    Yaml::String("Author".to_string()),
                    Yaml::String(
                        RefCell::borrow(&(**self.books.get_unchecked(book)))
                            .author
                            .clone(),
                    ),
                );

                data.insert(
                    Yaml::String("Pages".to_string()),
                    Yaml::Integer(
                        RefCell::borrow(&(**self.books.get_unchecked(book))).pages as i64,
                    ),
                );

                let mut book_arr = Array::new();

                for simple in &(**self.books.get_unchecked(book)).borrow().books {
                    let readers = (**simple)
                        .borrow_mut()
                        .readers
                        .iter()
                        .map(|x: &(Weak<RefCell<Reader>>, (Date, Date))| {
                            let mut hash_reader = Hash::new();

                            hash_reader.insert(
                                Yaml::String("Name".to_string()),
                                Yaml::String(
                                    RefCell::borrow(&*((x.0).upgrade().unwrap())).name.clone(),
                                ),
                            );

                            hash_reader.insert(
                                Yaml::String("Family".to_string()),
                                Yaml::String(
                                    RefCell::borrow(&*((x.0).upgrade().unwrap())).family.clone(),
                                ),
                            );

                            hash_reader.insert(
                                Yaml::String("Father".to_string()),
                                Yaml::String(
                                    RefCell::borrow(&*((x.0).upgrade().unwrap())).father.clone(),
                                ),
                            );

                            hash_reader.insert(
                                Yaml::String("Age".to_string()),
                                Yaml::Integer(
                                    RefCell::borrow(&*((x.0).upgrade().unwrap())).age as i64,
                                ),
                            );

                            hash_reader.insert(
                                Yaml::String("Start date".to_string()),
                                Yaml::Array(vec![
                                    Yaml::Integer((x.1).0.day as i64),
                                    Yaml::Integer((x.1).0.month as i64),
                                    Yaml::Integer((x.1).0.year as i64),
                                ]),
                            );

                            hash_reader.insert(
                                Yaml::String("Finish date".to_string()),
                                Yaml::Array(vec![
                                    Yaml::Integer((x.1).1.day as i64),
                                    Yaml::Integer((x.1).1.month as i64),
                                    Yaml::Integer((x.1).1.year as i64),
                                ]),
                            );

                            Yaml::Hash(hash_reader)
                        })
                        .collect::<Array>();

                    let mut hash_simple = Hash::new();

                    hash_simple.insert(
                        Yaml::String("Using".to_string()),
                        Yaml::Boolean(RefCell::borrow(&(**simple)).is_using),
                    );

                    hash_simple.insert(Yaml::String("Readers".to_string()), Yaml::Array(readers));
                    book_arr.push(Yaml::Hash(hash_simple));
                }

                data.insert(
                    Yaml::String("Simple Books".to_string()),
                    Yaml::Array(book_arr),
                );

                data.insert(
                    Yaml::String("Genres".to_string()),
                    if let Some(g) = &RefCell::borrow(&(**self.books.get_unchecked(book))).genres {
                        Yaml::Array(Array::from_iter(
                            g.iter()
                                .map(|x| Yaml::String(x.clone()))
                                .collect::<Vec<_>>(),
                        ))
                    } else {
                        Yaml::Null
                    },
                );
            }
            array.push(Yaml::Hash(data));
        }

        let mut string = String::new();
        let mut emitter = YamlEmitter::new(&mut string);
        emitter.dump(&Yaml::Array(array)).unwrap();

        let mut file = File::create("src/utils/books.yaml").unwrap();
        file.write_all(string.as_bytes()).unwrap();
    }

    /// load from .yaml file

    #[inline]
    pub fn load(&mut self, reader_base: &mut ReaderBase) {
        let mut file = File::open("src/utils/books.yaml").unwrap();
        let mut string = String::new();
        file.read_to_string(&mut string).unwrap();

        if !string.is_empty() {
            let docs = YamlLoader::load_from_str(string.as_str()).unwrap();
            let doc = docs.first().unwrap().clone().into_vec().unwrap();

            for d in doc {
                self.books.push(Rc::new(RefCell::new(TheBook::new(
                    d["Title"].as_str().unwrap().to_string(),
                    d["Author"].as_str().unwrap().to_string(),
                    d["Pages"].as_i64().unwrap() as u16,
                ))));

                (**self.books.last_mut().unwrap())
                    .borrow_mut()
                    .remove_book(0)
                    .unwrap();

                for simple in d["Simple Books"].as_vec().unwrap().iter() {
                    (**self.books.last_mut().unwrap()).borrow_mut().add_book();

                    (**(**self.books.last_mut().unwrap())
                        .borrow_mut()
                        .books
                        .last_mut()
                        .unwrap())
                    .borrow_mut()
                    .is_using = simple["Using"].as_bool().unwrap();

                    if simple["Using"].as_bool().unwrap() {
                        if let Some(last_reader) = simple["Readers"].as_vec().unwrap().last() {
                            unsafe {
                                let ind = reader_base
                                    .find_reader(
                                        &last_reader["Name"].as_str().unwrap().to_string(),
                                        &last_reader["Family"].as_str().unwrap().to_string(),
                                        &last_reader["Father"].as_str().unwrap().to_string(),
                                        last_reader["Age"].as_i64().unwrap() as u8,
                                    )
                                    .unwrap();

                                (**reader_base.readers.get_unchecked_mut(ind))
                                    .borrow_mut()
                                    .reading = Some(Rc::downgrade(
                                    &(*(**self.books.last_mut().unwrap())
                                        .borrow_mut()
                                        .books
                                        .last_mut()
                                        .unwrap()),
                                ));
                            }
                        }
                    }

                    for reader in simple["Readers"].as_vec().unwrap().iter() {
                        let ind = reader_base
                            .find_reader(
                                &reader["Name"].as_str().unwrap().to_string(),
                                &reader["Family"].as_str().unwrap().to_string(),
                                &reader["Father"].as_str().unwrap().to_string(),
                                reader["Age"].as_i64().unwrap() as u8,
                            )
                            .unwrap();

                        (**(**self.books.last_mut().unwrap())
                            .borrow_mut()
                            .books
                            .last_mut()
                            .unwrap())
                        .borrow_mut()
                        .readers
                        .push((
                            Rc::downgrade(unsafe { reader_base.readers.get_unchecked(ind) }),
                            (
                                Date::new(
                                    reader["Start date"][0].as_i64().unwrap() as u8,
                                    reader["Start date"][1].as_i64().unwrap() as u8,
                                    reader["Start date"][2].as_i64().unwrap() as u16,
                                )
                                .unwrap(),
                                Date::new(
                                    reader["Finish date"][0].as_i64().unwrap() as u8,
                                    reader["Finish date"][1].as_i64().unwrap() as u8,
                                    reader["Finish date"][2].as_i64().unwrap() as u16,
                                )
                                .unwrap(),
                            ),
                        ));

                        unsafe {
                            (**reader_base.readers.get_unchecked_mut(ind))
                                .borrow_mut()
                                .books
                                .push(Rc::downgrade(
                                    &(*(**self.books.last_mut().unwrap())
                                        .borrow_mut()
                                        .books
                                        .last_mut()
                                        .unwrap()),
                                ));
                        }
                    }
                }

                if d["Genres"].is_null() {
                    (**self.books.last_mut().unwrap()).borrow_mut().genres = None;
                } else {
                    (**self.books.last_mut().unwrap()).borrow_mut().genres = Some(HashSet::new());

                    for genres in d["Genres"].as_vec().unwrap().iter() {
                        (**self.books.last_mut().unwrap())
                            .borrow_mut()
                            .genres
                            .as_mut()
                            .unwrap()
                            .insert(genres.as_str().unwrap().to_string());
                    }
                }
            }
        }
    }
}
