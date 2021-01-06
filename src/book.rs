extern crate chrono;
extern crate yaml_rust;
use crate::reader::*;
use chrono::Datelike;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter, Result};
use std::fs::File;
use std::io::{Read, Write};
use std::rc::{Rc, Weak};
use yaml_rust::yaml::*;
use yaml_rust::{Yaml, YamlEmitter, YamlLoader};

/// Error-handling type.
/// If everything is ok, it should return self (but it's not necessary),
/// else it will return err with code
/// (that's will help you to correctly understand error)

pub(crate) type ResultSelf<'a, T> = std::result::Result<&'a mut T, u8>;

/// Date structure, which contains day, month and year.
/// It's a copyable type like i32 (no move).
/// You can clone, debug and compare as == / !=

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Date {
    pub(crate) day: u8,
    pub(crate) month: u8,
    pub(crate) year: u16,
}

/// Simple Book structure, which contains
/// title, author, amount of pages, using status
/// and readers with start and finish dates

pub(crate) struct Book {
    pub(crate) title: String,
    pub(crate) author: String,
    pub(crate) pages: u16,
    pub(crate) is_using: bool,
    pub(crate) readers: Vec<(Weak<RefCell<Reader>>, (Date, Date))>,
}

/// Interface Book structure, which contains
/// title, author, amount of pages, and simple books

pub(crate) struct TheBook {
    pub(crate) title: String,
    pub(crate) author: String,
    pub(crate) pages: u16,
    pub(crate) books: Vec<Rc<RefCell<Book>>>,
}

/// Reader Base structure,
/// which contains only Book interfaces

pub struct BookSystem {
    pub(crate) books: Vec<Rc<RefCell<TheBook>>>,
}

/// Trait, which used to params of books
/// like title, author and amount of pages

pub(crate) trait BookInterface {
    fn change_title(&mut self, new_title: String) -> &mut Self;
    fn change_author(&mut self, new_author: String) -> &mut Self;
    fn change_pages(&mut self, new_pages: u16) -> &mut Self;
}

/// Dates can be compared as >, <, >=, <=
/// (as it works in real world)

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return if self.year < other.year {
            Some(Ordering::Less)
        } else if self.year > other.year {
            Some(Ordering::Greater)
        } else {
            if self.month < other.month {
                Some(Ordering::Less)
            } else if self.month > other.month {
                Some(Ordering::Greater)
            } else {
                self.day.partial_cmp(&other.day)
            }
        };
    }
}

/// Dates can be compared as >, <, >=, <=
/// (as it works in real world)

impl Ord for Date {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Date {
    /// Constructs date. If date params are wrong,
    /// It will return Err.

    #[inline]
    pub fn new(new_day: u8, new_month: u8, new_year: u16) -> std::result::Result<Self, ()> {
        let date = Date {
            day: new_day,
            month: new_month,
            year: new_year,
        };

        return if date.correct() { Ok(date) } else { Err(()) };
    }

    /// Checks if date is correct
    /// according to real world

    #[inline]
    pub fn correct(&self) -> bool {
        return if self.month > 12 || self.month == 0 || self.day == 0 {
            false
        } else {
            const DAYS: [u8; 13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

            if self.month != 2 {
                unsafe { self.day <= *DAYS.get_unchecked(self.month as usize) }
            } else if self.year % 4 == 0 && self.year % 100 != 0 || self.year % 400 == 0 {
                unsafe { self.day <= *DAYS.get_unchecked(2) + 1 }
            } else {
                unsafe { self.day <= *DAYS.get_unchecked(2) }
            }
        };
    }
}

/// Destructor for simple book.
/// It is used to debug code.

impl Drop for Book {
    #[inline]
    fn drop(&mut self) {
        println!("Book {} {} is deleted", self.title, self.author)
    }
}

/// Print for simple book.
/// It is used to debug code.

impl Debug for Book {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Book")
            .field("title", &self.title)
            .field("author", &self.author)
            .field("pages", &self.pages)
            .field("is using", &self.is_using)
            .field(
                "readers",
                &self
                    .readers
                    .iter()
                    .map(|x| {
                        (*((*x).0).upgrade().unwrap()).borrow().name.clone()
                            + " "
                            + (*((*x).0).upgrade().unwrap())
                                .borrow()
                                .family
                                .clone()
                                .as_str()
                            + " "
                            + (*((*x).0).upgrade().unwrap())
                                .borrow()
                                .father
                                .clone()
                                .as_str()
                            + " "
                            + format!("{}", (*((*x).0).upgrade().unwrap()).borrow().age).as_str()
                    })
                    .collect::<Vec<String>>(),
            )
            .finish()
    }
}

/// Implementation of Book Interface trait for simple book

impl BookInterface for Book {
    /// Title changing

    #[inline]
    fn change_title(&mut self, new_title: String) -> &mut Self {
        self.title = new_title;
        self
    }

    /// Author changing

    #[inline]
    fn change_author(&mut self, new_author: String) -> &mut Self {
        self.author = new_author;
        self
    }

    /// Pages changing

    #[inline]
    fn change_pages(&mut self, new_pages: u16) -> &mut Self {
        self.pages = new_pages;
        self
    }
}

impl Book {
    /// Constructs new simple book.
    /// By default it has no readers and it isn't used

    #[inline]
    pub fn new(new_title: String, new_author: String, new_pages: u16) -> Self {
        Book {
            title: new_title,
            author: new_author,
            pages: new_pages,
            is_using: false,
            readers: vec![],
        }
    }

    /// Searches reader.
    /// If it isn't found, it' ll return the amount of all readers.
    /// else it will return reader index

    #[inline]
    pub fn find_reader(&self, reader: &Rc<RefCell<Reader>>) -> usize {
        for it in 0..self.readers.len() {
            let reader_ptr;

            unsafe {
                reader_ptr = ((*self.readers.get_unchecked(it)).0)
                    .upgrade()
                    .unwrap()
                    .as_ptr();
            }

            if reader_ptr.is_null() {
                panic!("nullptr in Book find_reader");
            }

            if reader_ptr == reader.as_ptr() {
                return it;
            }
        }
        self.readers.len()
    }

    /// Removes reader by raw pointer.

    #[inline]
    pub fn remove_reader(&mut self, reader: *mut Reader) -> &mut Self {
        unsafe {
            if (*reader).reading.is_some() {
                ((*reader).reading.as_ref().unwrap().upgrade().unwrap())
                    .borrow_mut()
                    .is_using = false;
            }
        }

        self.readers = self
            .readers
            .clone()
            .into_iter()
            .filter(|x| ((*x).0).upgrade().unwrap().as_ptr() as *const Reader != reader)
            .collect();
        self
    }

    /// Removes all readers of book

    #[inline]
    pub fn remove_all_readers(&mut self) -> &mut Self {
        if self.is_using {
            (*(self.readers.last_mut().unwrap().0).upgrade().unwrap())
                .borrow_mut()
                .reading = None;
        }

        while !self.readers.is_empty() {
            (*((*self.readers.last_mut().unwrap()).0).upgrade().unwrap())
                .borrow_mut()
                .remove_book(self as *mut Book);
            self.readers.pop();
        }
        self
    }

    /// Function that uses after giving book to reader.
    /// It adds reader (converts rc to weak), start and return dates.

    #[inline]
    pub fn start_reading(&mut self, reader: &Rc<RefCell<Reader>>, date: Date) -> &mut Self {
        let now = chrono::Utc::now();

        self.readers.push((
            Rc::downgrade(&reader),
            (
                Date::new(now.day() as u8, now.month() as u8, now.year() as u16).unwrap(),
                date,
            ),
        ));

        self.is_using = true;
        self
    }

    /// Function that uses after returning book from reader.
    /// It changes book's status and finish date  

    #[inline]
    pub fn finish_reading(&mut self) -> ResultSelf<Self> {
        self.is_using = false;
        let now = chrono::Utc::now();
        let was = ((*self.readers.last().unwrap()).1).1;

        if now.day() as u8 > was.day
            || now.month() as u8 > was.month
            || now.year() as u16 > was.year
        {
            Err(1) // Reader is late
        } else {
            ((*self.readers.last_mut().unwrap()).1).1 =
                Date::new(now.day() as u8, now.month() as u8, now.year() as u16).unwrap();
            Ok(self)
        }
    }
}

/// Destructor for TheBook.
/// It is used to debug code

impl Drop for TheBook {
    #[inline]
    fn drop(&mut self) {
        println!("The Book {} {} is deleted", self.title, self.author)
    }
}

/// Compare TheBooks by title, author and pages.

impl PartialEq for TheBook {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title && self.author == other.author && self.pages == other.pages
    }
}

/// Compare TheBooks by title, author and pages.

impl Eq for TheBook {}

/// Print for TheBook.
/// It is used to debug code

impl Debug for TheBook {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("The Book")
            .field("title", &self.title)
            .field("author", &self.author)
            .field("pages", &self.pages)
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

/// Book Interface trait implementation for TheBook.
/// Changing title, author, amount of pages

impl BookInterface for TheBook {
    #[inline]
    fn change_title(&mut self, new_title: String) -> &mut Self {
        self.books = self
            .books
            .iter_mut()
            .map(|x| {
                (**x).borrow_mut().title = new_title.clone();
                x.clone()
            })
            .collect::<Vec<Rc<RefCell<Book>>>>();
        self.title = new_title;
        self
    }

    #[inline]
    fn change_author(&mut self, new_author: String) -> &mut Self {
        self.books = self
            .books
            .iter_mut()
            .map(|x| {
                (**x).borrow_mut().author = new_author.clone();
                x.clone()
            })
            .collect::<Vec<Rc<RefCell<Book>>>>();
        self.title = new_author;
        self
    }

    #[inline]
    fn change_pages(&mut self, new_pages: u16) -> &mut Self {
        self.pages = new_pages;
        self.books = self
            .books
            .iter_mut()
            .map(|x| {
                (**x).borrow_mut().pages = new_pages;
                x.clone()
            })
            .collect::<Vec<Rc<RefCell<Book>>>>();
        self
    }
}

impl TheBook {
    /// Constructs TheBook

    #[inline]
    pub fn new(new_title: String, new_author: String, new_pages: u16) -> Self {
        let mut book = TheBook {
            title: new_title,
            author: new_author,
            pages: new_pages,
            books: vec![],
        };

        book.add_book();
        book
    }

    /// Return index of unused book.
    /// If all are used, it will return amount of books

    #[inline]
    pub fn get_unused(&self) -> usize {
        for i in 0..self.books.len() {
            unsafe {
                if !(**self.books.get_unchecked(i)).borrow().is_using {
                    return i;
                }
            }
        }
        self.books.len()
    }

    /// Finds using book by reader

    #[inline]
    pub fn find_by_reader(&self, reader: &Rc<RefCell<Reader>>) -> usize {
        let weak = Rc::downgrade(reader);

        for ind in 0..self.books.len() {
            unsafe {
                if (*self.books.get_unchecked(ind)).borrow_mut().is_using
                    && ((*self.books.get_unchecked(ind))
                        .borrow_mut()
                        .readers
                        .last()
                        .unwrap())
                    .0
                    .ptr_eq(&weak)
                {
                    return ind;
                }
            }
        }
        self.books.len()
    }

    /// add one simple book

    #[inline]
    pub fn add_book(&mut self) -> &mut Self {
        self.books.push(Rc::new(RefCell::new(Book::new(
            self.title.clone(),
            self.author.clone(),
            self.pages,
        ))));
        self
    }

    /// Remove simple book by index.
    /// If index is incorrect, it will return Err

    #[inline]
    pub fn remove_book(&mut self, ind: usize) -> ResultSelf<Self> {
        return if ind == self.books.len() {
            Err(0)
        } else {
            unsafe {
                (**self.books.get_unchecked_mut(ind))
                    .borrow_mut()
                    .remove_all_readers();
            }

            self.books.remove(ind);
            Ok(self)
        };
    }

    /// Removes all simple books

    #[inline]
    pub fn remove_all_books(&mut self) -> &mut Self {
        while !self.books.is_empty() {
            unsafe {
                (**self.books.get_unchecked(self.books.len() - 1))
                    .borrow_mut()
                    .remove_all_readers();
            }
            self.books.pop().unwrap();
        }
        self
    }
}

/// Print for BookSystem.
/// It is used for debug code

impl Debug for BookSystem {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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
    pub fn find_book(&self, title: &String, author: &String, pages: u16) -> usize {
        unsafe {
            for i in 0..self.books.len() {
                if (**self.books.get_unchecked(i)).borrow().title == *title
                    && (**self.books.get_unchecked(i)).borrow().author == *author
                    && (**self.books.get_unchecked(i)).borrow().pages == pages
                {
                    return i;
                }
            }
            self.books.len()
        }
    }

    /// Adds simple books without any checks

    #[inline]
    pub unsafe fn add_books_unchecked(&mut self, ind: usize, amount: usize) -> ResultSelf<Self> {
        for _ in 0..amount {
            (*self.books.get_unchecked(ind)).borrow_mut().add_book();
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
                let size = (*self.books.get_unchecked(ind)).borrow_mut().books.len() as u128;

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
        return if !self.books.is_empty()
            && self.find_book(&title, &author, pages) < self.books.len()
        {
            Err(0) // already exists
        } else {
            Ok(unsafe { self.add_book_unchecked(title, author, pages) })
        };
    }

    /// Remove one simple book by index without any checks

    #[inline]
    pub unsafe fn remove_one_book_unchecked(&mut self, ind: usize, rind: usize) {
        (*self.books.get_unchecked_mut(ind))
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
                if rind >= (*self.books.get_unchecked(ind)).borrow_mut().books.len() {
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
                if self.find_book(
                    &new_title,
                    &(*self.books.get_unchecked(ind)).borrow_mut().author,
                    (*self.books.get_unchecked(ind)).borrow_mut().pages,
                ) < self.books.len()
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
                if self.find_book(
                    &(*self.books.get_unchecked(ind)).borrow_mut().title,
                    &new_author,
                    (*self.books.get_unchecked(ind)).borrow_mut().pages,
                ) < self.books.len()
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
                if self.find_book(
                    &(*self.books.get_unchecked(ind)).borrow_mut().title,
                    &(*self.books.get_unchecked(ind)).borrow_mut().author,
                    new_pages_num,
                ) < self.books.len()
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
                    Yaml::String((*self.books.get_unchecked(book)).borrow_mut().title.clone()),
                );

                data.insert(
                    Yaml::String("Author".to_string()),
                    Yaml::String(
                        (*self.books.get_unchecked(book))
                            .borrow_mut()
                            .author
                            .clone(),
                    ),
                );

                data.insert(
                    Yaml::String("Pages".to_string()),
                    Yaml::Integer((*self.books.get_unchecked(book)).borrow_mut().pages as i64),
                );

                let mut book_arr = Array::new();

                for simple in &(*self.books.get_unchecked(book)).borrow_mut().books {
                    let readers = (*simple)
                        .borrow_mut()
                        .readers
                        .iter()
                        .map(|x| {
                            let mut hash_reader = Hash::new();

                            hash_reader.insert(
                                Yaml::String("Name".to_string()),
                                Yaml::String(((x.0).upgrade().unwrap()).borrow_mut().name.clone()),
                            );

                            hash_reader.insert(
                                Yaml::String("Family".to_string()),
                                Yaml::String(
                                    ((x.0).upgrade().unwrap()).borrow_mut().family.clone(),
                                ),
                            );

                            hash_reader.insert(
                                Yaml::String("Father".to_string()),
                                Yaml::String(
                                    ((x.0).upgrade().unwrap()).borrow_mut().father.clone(),
                                ),
                            );

                            hash_reader.insert(
                                Yaml::String("Age".to_string()),
                                Yaml::Integer(((x.0).upgrade().unwrap()).borrow_mut().age as i64),
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
                        Yaml::Boolean((*simple).borrow_mut().is_using),
                    );

                    hash_simple.insert(Yaml::String("Readers".to_string()), Yaml::Array(readers));
                    book_arr.push(Yaml::Hash(hash_simple));
                }

                data.insert(
                    Yaml::String("Simple Books".to_string()),
                    Yaml::Array(book_arr),
                );
            }
            array.push(Yaml::Hash(data));
        }

        let mut string = String::new();
        let mut emitter = YamlEmitter::new(&mut string);
        emitter.dump(&Yaml::Array(array)).unwrap();

        let mut file = File::create("books.yaml").unwrap();
        file.write_all(string.as_bytes()).unwrap();
    }

    /// load from .yaml file

    #[inline]
    pub fn load(&mut self, reader_base: &mut ReaderBase) {
        let mut file = File::open("books.yaml").unwrap();
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

                (*self.books.last_mut().unwrap())
                    .borrow_mut()
                    .remove_book(0)
                    .unwrap();

                for simple in d["Simple Books"].as_vec().unwrap().iter() {
                    (*self.books.last_mut().unwrap()).borrow_mut().add_book();

                    (*(*self.books.last_mut().unwrap())
                        .borrow_mut()
                        .books
                        .last_mut()
                        .unwrap())
                    .borrow_mut()
                    .is_using = simple["Using"].as_bool().unwrap();

                    if simple["Using"].as_bool().unwrap() {
                        if let Some(last_reader) = simple["Readers"].as_vec().unwrap().last() {
                            unsafe {
                                let ind = reader_base.find_reader(
                                    &last_reader["Name"].as_str().unwrap().to_string(),
                                    &last_reader["Family"].as_str().unwrap().to_string(),
                                    &last_reader["Father"].as_str().unwrap().to_string(),
                                    last_reader["Age"].as_i64().unwrap() as u8,
                                );

                                (*reader_base.readers.get_unchecked_mut(ind))
                                    .borrow_mut()
                                    .reading = Some(Rc::downgrade(
                                    &(*(*self.books.last_mut().unwrap())
                                        .borrow_mut()
                                        .books
                                        .last_mut()
                                        .unwrap()),
                                ));
                            }
                        }
                    }

                    for reader in simple["Readers"].as_vec().unwrap().iter() {
                        let ind = reader_base.find_reader(
                            &reader["Name"].as_str().unwrap().to_string(),
                            &reader["Family"].as_str().unwrap().to_string(),
                            &reader["Father"].as_str().unwrap().to_string(),
                            reader["Age"].as_i64().unwrap() as u8,
                        );

                        (*(*self.books.last_mut().unwrap())
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
                            (*reader_base.readers.get_unchecked_mut(ind))
                                .borrow_mut()
                                .books
                                .push(Rc::downgrade(
                                    &(*(*self.books.last_mut().unwrap())
                                        .borrow_mut()
                                        .books
                                        .last_mut()
                                        .unwrap()),
                                ));
                        }
                    }
                }
            }
        }
    }
}
