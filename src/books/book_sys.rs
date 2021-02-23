extern crate fltk;
extern crate yaml_rust;

use crate::{
    books::{book::Book, date::Date, the_book::TheBook, BookInterface, ResultSelf},
    reading::{read_base::ReaderBase, reader::Reader},
    Lang,
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

use fltk::app::App;

use yaml_rust::{
    yaml::{Array, Hash},
    Yaml, YamlEmitter, YamlLoader,
};

/// Reader Base structure,
/// which contains only Book interfaces

pub struct BookSystem {
    pub(crate) books: Vec<Rc<RefCell<TheBook>>>,
}

impl Debug for BookSystem {
    /// Print for BookSystem.
    /// It is used for debug code

    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Book System")
            .field(
                "books",
                &self
                    .books
                    .iter()
                    .map(|x| format!("{:?}", *(**x).borrow()))
                    .collect::<Vec<_>>(),
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
    pub(crate) fn find_book(&self, title: &String, author: &String, pages: u16) -> Option<usize> {
        self.books.iter().position(|x| {
            (**x).borrow().title == *title
                && (**x).borrow().author == *author
                && (**x).borrow().pages == pages
        })
    }

    /// Adds simple books without any checks

    #[inline]
    pub(crate) unsafe fn add_books_unchecked(
        &mut self,
        ind: usize,
        amount: usize,
        app: &App,
        lang: Lang,
    ) -> ResultSelf<Self> {
        (0..amount).for_each(|_| {
            (**self.books.get_unchecked_mut(ind))
                .borrow_mut()
                .add_book(app, lang);
        });
        Ok(self)
    }

    /// Adds simple books with strong guarantee

    #[inline]
    pub(crate) fn add_books(
        &mut self,
        ind: usize,
        amount: usize,
        app: &App,
        lang: Lang,
    ) -> ResultSelf<Self> {
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
                self.add_books_unchecked(ind, amount, app, lang)
            }
        };
    }

    /// Adds new TheBook and **ONE** simple
    /// (I think it's logical)
    /// No checks provided

    #[inline]
    pub(crate) unsafe fn add_book_unchecked(
        &mut self,
        new_title: String,
        new_author: String,
        new_pages: u16,
        amount: usize,
        app: &App,
        lang: Lang,
    ) -> &mut Self {
        self.books.push(Rc::new(RefCell::new(TheBook::new(
            new_title, new_author, new_pages, amount, app, lang,
        ))));
        self
    }

    /// Adds new TheBook and **ONE** simple
    /// (I think it's logical)

    #[inline]
    pub(crate) fn add_book(
        &mut self,
        new_title: String,
        new_author: String,
        new_pages: u16,
        amount: usize,
        app: &App,
        lang: Lang,
    ) -> ResultSelf<Self> {
        return if !self.books.is_empty()
            && self.find_book(&new_title, &new_author, new_pages).is_some()
        {
            Err(0) // already exists
        } else {
            Ok(unsafe {
                self.add_book_unchecked(new_title, new_author, new_pages, amount, app, lang)
            })
        };
    }

    /// Remove one simple book by index without any checks

    #[inline]
    pub(crate) unsafe fn remove_one_book_unchecked(&mut self, ind: usize, rind: usize) {
        (**self.books.get_unchecked_mut(ind))
            .borrow_mut()
            .remove_book(rind)
            .unwrap();
    }

    /// Remove one simple book by index

    #[inline]
    pub(crate) fn remove_one_book(&mut self, ind: usize, rind: usize) -> ResultSelf<Self> {
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
    pub(crate) unsafe fn remove_book_unchecked(&mut self, ind: usize) -> &mut Self {
        (**self.books.get_unchecked(ind))
            .borrow_mut()
            .remove_all_books();

        self.books.remove(ind);
        self
    }

    /// Removes TheBook and all simple books

    #[inline]
    pub(crate) fn remove_book(&mut self, ind: usize) -> ResultSelf<Self> {
        return if ind >= self.books.len() {
            Err(0) // out of range
        } else {
            Ok(unsafe { self.remove_book_unchecked(ind) })
        };
    }

    /// Changes TheBook's and all simple books' title without any checks

    #[inline]
    pub(crate) unsafe fn change_title_unchecked(
        &mut self,
        ind: usize,
        new_title: String,
    ) -> &mut Self {
        (**self.books.get_unchecked_mut(ind))
            .borrow_mut()
            .change_title(new_title);
        self
    }

    /// Changes TheBook's and all simple books' title

    #[inline]
    pub(crate) fn change_title(&mut self, ind: usize, new_title: String) -> ResultSelf<Self> {
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
    pub(crate) unsafe fn change_author_unchecked(
        &mut self,
        ind: usize,
        new_author: String,
    ) -> &mut Self {
        (**self.books.get_unchecked_mut(ind))
            .borrow_mut()
            .change_author(new_author);
        self
    }

    /// Changes TheBook's and all simple books' title

    #[inline]
    pub(crate) fn change_author(&mut self, ind: usize, new_author: String) -> ResultSelf<Self> {
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
    pub(crate) unsafe fn change_pages_unchecked(
        &mut self,
        ind: usize,
        new_pages: u16,
    ) -> &mut Self {
        (**self.books.get_unchecked_mut(ind))
            .borrow_mut()
            .change_pages(new_pages);
        self
    }

    /// Changes TheBook's and all simple books' title

    #[inline]
    pub(crate) fn change_pages(&mut self, ind: usize, new_pages: String) -> ResultSelf<Self> {
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

    /// Changes simple Book's location without any checks

    #[inline]
    pub(crate) unsafe fn change_location_unchecked(
        &mut self,
        t_ind: usize,
        s_ind: usize,
        new_cabinet: u16,
        new_shelf: u8,
    ) -> &mut Self {
        (**(**self.books.get_unchecked_mut(t_ind))
            .borrow_mut()
            .books
            .get_unchecked_mut(s_ind))
        .borrow_mut()
        .change_location(new_cabinet, new_shelf);
        self
    }

    /// Changes simple Book's location

    #[inline]
    pub(crate) fn change_location(
        &mut self,
        t_ind: usize,
        s_ind: usize,
        new_cabinet: String,
        new_shelf: String,
    ) -> ResultSelf<Self> {
        match new_cabinet.trim().parse::<u16>() {
            Err(_) => {
                return Err(0); // new cabinet err
            }

            Ok(cab) => {
                match new_shelf.trim().parse::<u8>() {
                    Err(_) => {
                        return Err(1); // new shelf err
                    }

                    Ok(shelf) => {
                        if t_ind > self.books.len() || t_ind == 0 {
                            return Err(2); // index out of range (the book)
                        } else if s_ind
                            > unsafe {
                                (**self.books.get_unchecked(t_ind - 1)).borrow().books.len()
                            }
                        {
                            return Err(3); // index out of range (book)
                        } else {
                            Ok(unsafe {
                                self.change_location_unchecked(t_ind - 1, s_ind - 1, cab, shelf)
                            })
                        }
                    }
                }
            }
        }
    }

    /// Deletes all books from current Book System
    /// (But not Books themselves)

    #[inline]
    pub(crate) fn clear(&mut self) -> &mut Self {
        self.books.clear();
        self
    }

    /// Clones Book System
    /// with new smart pointers for The Book

    #[inline]
    pub(crate) fn clone(&self, reader_base: &ReaderBase) -> Self {
        BookSystem {
            books: self
                .books
                .iter()
                .map(|x| Rc::new(RefCell::new((**x).borrow().clone(reader_base))))
                .collect(),
        }
    }

    /// Save to .yaml file

    pub(crate) fn save(&self) {
        let mut array = Array::new();

        (0..self.books.len()).for_each(|book| {
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

                (**self.books.get_unchecked(book))
                    .borrow()
                    .books
                    .iter()
                    .for_each(|simple| {
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
                                        RefCell::borrow(&*((x.0).upgrade().unwrap()))
                                            .family
                                            .clone(),
                                    ),
                                );

                                hash_reader.insert(
                                    Yaml::String("Father".to_string()),
                                    Yaml::String(
                                        RefCell::borrow(&*((x.0).upgrade().unwrap()))
                                            .father
                                            .clone(),
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
                            Yaml::String("Cabinet".to_string()),
                            Yaml::Integer(RefCell::borrow(&(**simple)).cabinet as i64),
                        );

                        hash_simple.insert(
                            Yaml::String("Shelf".to_string()),
                            Yaml::Integer(RefCell::borrow(&(**simple)).shelf as i64),
                        );

                        hash_simple.insert(
                            Yaml::String("Using".to_string()),
                            Yaml::Boolean(RefCell::borrow(&(**simple)).is_using),
                        );

                        hash_simple
                            .insert(Yaml::String("Readers".to_string()), Yaml::Array(readers));
                        book_arr.push(Yaml::Hash(hash_simple));
                    });

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
        });

        let mut string = String::new();
        let mut emitter = YamlEmitter::new(&mut string);
        emitter.dump(&Yaml::Array(array)).unwrap();

        File::create("src/utils/books.yaml")
            .unwrap()
            .write_all(string.as_bytes())
            .unwrap();
    }

    /// load from .yaml file

    #[inline]
    pub fn load(&mut self, reader_base: &mut ReaderBase) {
        let mut string = String::new();

        File::open("src/utils/books.yaml")
            .unwrap()
            .read_to_string(&mut string)
            .unwrap();

        if !string.is_empty() {
            let docs = YamlLoader::load_from_str(string.as_str()).unwrap();
            let doc = docs.first().unwrap().clone().into_vec().unwrap();

            for d in doc {
                self.books.push(Rc::new(RefCell::new(TheBook {
                    title: d["Title"].as_str().unwrap().to_string(),
                    author: d["Author"].as_str().unwrap().to_string(),
                    pages: d["Pages"].as_i64().unwrap() as u16,
                    books: vec![],
                    genres: None,
                })));

                d["Simple Books"]
                    .as_vec()
                    .unwrap()
                    .iter()
                    .for_each(|simple| {
                        let new_title = (**self.books.last().unwrap()).borrow().title.clone();
                        let new_author = (**self.books.last().unwrap()).borrow().author.clone();
                        let new_pages = (**self.books.last().unwrap()).borrow().pages;
                        let new_is_using = simple["Using"].as_bool().unwrap();

                        (*(**self.books.last_mut().unwrap()).borrow_mut())
                            .books
                            .push(Rc::new(RefCell::new(Book {
                                title: new_title,
                                author: new_author,
                                pages: new_pages,
                                is_using: new_is_using,
                                cabinet: simple["Cabinet"].as_i64().unwrap() as u16,
                                shelf: simple["Shelf"].as_i64().unwrap() as u8,
                                readers: vec![],
                            })));

                        if new_is_using {
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

                        simple["Readers"]
                            .as_vec()
                            .unwrap()
                            .iter()
                            .for_each(|reader| {
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
                                    Rc::downgrade(unsafe {
                                        reader_base.readers.get_unchecked(ind)
                                    }),
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
                            });
                    });

                if d["Genres"].is_null() {
                    (**self.books.last_mut().unwrap()).borrow_mut().genres = None;
                } else {
                    (**self.books.last_mut().unwrap()).borrow_mut().genres = Some(HashSet::new());

                    d["Genres"].as_vec().unwrap().iter().for_each(|genres| {
                        (**self.books.last_mut().unwrap())
                            .borrow_mut()
                            .genres
                            .as_mut()
                            .unwrap()
                            .insert(genres.as_str().unwrap().to_string());
                    });
                }
            }
        }
    }
}
