extern crate chrono;
extern crate yaml_rust;
use crate::reader::{Reader, ReaderBase};
use chrono::Datelike;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter, Result};
use std::fs::File;
use std::io::{Read, Write};
use std::rc::{Rc, Weak};
use yaml_rust::yaml::Array;
use yaml_rust::yaml::Hash;
use yaml_rust::YamlLoader;
use yaml_rust::{Yaml, YamlEmitter};

pub(crate) type ResultSelf<'a, T> = std::result::Result<&'a mut T, u8>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Date {
    pub(crate) day: u8,
    pub(crate) month: u8,
    pub(crate) year: u16,
}

pub(crate) struct Book {
    pub(crate) title: String,
    pub(crate) author: String,
    pub(crate) pages: u16,
    pub(crate) is_using: bool,
    pub(crate) readers: Vec<(Weak<RefCell<Reader>>, (Date, Date))>,
}

pub(crate) struct TheBook {
    pub(crate) title: String,
    pub(crate) author: String,
    pub(crate) pages: u16,
    pub(crate) books: Vec<Rc<RefCell<Book>>>,
}

pub struct BookSystem {
    pub(crate) books: Vec<Rc<RefCell<TheBook>>>,
}

pub(crate) trait BookInterface {
    fn change_title(&mut self, new_title: String) -> &mut Self;
    fn change_author(&mut self, new_author: String) -> &mut Self;
    fn change_pages(&mut self, new_pages: u16) -> &mut Self;
}

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

impl Ord for Date {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Date {
    #[inline]
    pub fn new(new_day: u8, new_month: u8, new_year: u16) -> Option<Self> {
        let date = Date {
            day: new_day,
            month: new_month,
            year: new_year,
        };

        return if date.correct() { Some(date) } else { None };
    }

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

impl Drop for Book {
    #[inline]
    fn drop(&mut self) {
        println!("Book {} {} is deleted", self.title, self.author)
    }
}

impl Debug for Book {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Book")
            .field("title", &self.title)
            .field("author", &self.author)
            .field("pages", &self.pages)
            .field("is using", &self.is_using)
            .field(
                "readers.yaml",
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

impl BookInterface for Book {
    #[inline]
    fn change_title(&mut self, new_title: String) -> &mut Self {
        self.title = new_title;
        self
    }

    #[inline]
    fn change_author(&mut self, new_author: String) -> &mut Self {
        self.author = new_author;
        self
    }

    #[inline]
    fn change_pages(&mut self, new_pages: u16) -> &mut Self {
        self.pages = new_pages;
        self
    }
}

impl Book {
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

    #[inline]
    pub fn remove_reader(&mut self, reader: *const Reader) -> &mut Self {
        self.readers = self
            .readers
            .clone()
            .into_iter()
            .filter(|x| ((*x).0).upgrade().unwrap().as_ptr() as *const Reader != reader)
            .collect();
        self
    }

    #[inline]
    pub fn remove_all_readers(&mut self) -> &mut Self {
        while !self.readers.is_empty() {
            (*((*self.readers.last_mut().unwrap()).0).upgrade().unwrap())
                .borrow_mut()
                .remove_book(self as *const Book);
            self.readers.pop();
        }
        self
    }

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

impl Drop for TheBook {
    #[inline]
    fn drop(&mut self) {
        println!("The Book {} {} is deleted", self.title, self.author)
    }
}

impl PartialEq for TheBook {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title && self.author == other.author && self.pages == other.pages
    }
}

impl Eq for TheBook {}

impl Debug for TheBook {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("The Book")
            .field("title", &self.title)
            .field("author", &self.author)
            .field("pages", &self.pages)
            .field(
                "books.yaml",
                &self
                    .books
                    .iter()
                    .map(|x| format!("{:?}", *(**x).borrow()))
                    .collect::<Vec<String>>(),
            )
            .finish()
    }
}

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

    #[inline]
    pub fn add_book(&mut self) -> &mut Self {
        self.books.push(Rc::new(RefCell::new(Book::new(
            self.title.clone(),
            self.author.clone(),
            self.pages,
        ))));
        self
    }

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

impl Debug for BookSystem {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Book System")
            .field(
                "books.yaml",
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
    #[inline]
    pub const fn new() -> Self {
        BookSystem { books: vec![] }
    }

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

    #[inline]
    pub fn add_books(
        &mut self,
        title: String,
        author: String,
        pages: u16,
        amount: usize,
    ) -> ResultSelf<Self> {
        let find = self.find_book(&title, &author, pages);

        return if find == self.books.len() {
            Err(1) // the book is not found
        } else {
            unsafe {
                let size = (*self.books.get_unchecked(find)).borrow_mut().books.len() as u128;

                if size + amount as u128 > usize::MAX as u128 {
                    return Err(0); // too much books.yaml
                }
            }

            for _ in 0..amount {
                unsafe {
                    (*self.books.get_unchecked(find)).borrow_mut().add_book();
                }
            }
            Ok(self)
        };
    }

    #[inline]
    pub fn add_book(&mut self, title: String, author: String, pages: u16) -> ResultSelf<Self> {
        return if !self.books.is_empty()
            && self.find_book(&title, &author, pages) < self.books.len()
        {
            Err(0)
        } else {
            self.books
                .push(Rc::new(RefCell::new(TheBook::new(title, author, pages))));
            Ok(self)
        };
    }

    #[inline]
    pub fn remove_one_book(
        &mut self,
        title: &String,
        author: &String,
        pages: u16,
        ind: usize,
    ) -> ResultSelf<Self> {
        let find = self.find_book(title, author, pages);

        return if find == self.books.len() {
            Err(0) // the book is not found
        } else {
            unsafe {
                if let Err(_) = (*self.books.get_unchecked_mut(find))
                    .borrow_mut()
                    .remove_book(ind)
                {
                    return Err(1); // ind error
                }
            }
            Ok(self)
        };
    }

    #[inline]
    pub fn remove_book(&mut self, title: &String, author: &String, pages: u16) -> ResultSelf<Self> {
        let find = self.find_book(title, author, pages);

        return if find == self.books.len() {
            Err(0)
        } else {
            unsafe {
                (**self.books.get_unchecked(find))
                    .borrow_mut()
                    .remove_all_books();
            }
            self.books.remove(find);
            Ok(self)
        };
    }

    #[inline]
    pub fn change_title(
        &mut self,
        title: &String,
        author: &String,
        pages: u16,
        new_title: String,
    ) -> ResultSelf<Self> {
        let find = self.find_book(title, author, pages);

        return if find == self.books.len() {
            Err(0) // not found
        } else {
            if self.find_book(&new_title, author, pages) < self.books.len() {
                Err(1) // already exists
            } else {
                unsafe {
                    (**self.books.get_unchecked_mut(find))
                        .borrow_mut()
                        .change_title(new_title);
                }
                Ok(self)
            }
        };
    }

    #[inline]
    pub fn change_author(
        &mut self,
        title: &String,
        author: &String,
        pages: u16,
        new_author: String,
    ) -> ResultSelf<Self> {
        let find = self.find_book(title, author, pages);

        return if find == self.books.len() {
            Err(0) // not found
        } else {
            if self.find_book(title, &new_author, pages) < self.books.len() {
                Err(1) // already exists
            } else {
                unsafe {
                    (**self.books.get_unchecked_mut(find))
                        .borrow_mut()
                        .change_author(new_author);
                }
                Ok(self)
            }
        };
    }

    #[inline]
    pub fn change_pages(
        &mut self,
        title: &String,
        author: &String,
        pages: u16,
        new_pages: String,
    ) -> ResultSelf<Self> {
        let new_pages_num;

        match new_pages.trim().parse::<u16>() {
            Ok(x) => new_pages_num = x,
            Err(_) => return Err(0), // parse error
        }

        let find = self.find_book(title, author, pages);

        return if find == self.books.len() {
            Err(1) // not found
        } else {
            if self.find_book(title, author, new_pages_num) < self.books.len() {
                Err(2) // already exists
            } else {
                unsafe {
                    (**self.books.get_unchecked_mut(find))
                        .borrow_mut()
                        .change_pages(new_pages_num);
                }
                Ok(self)
            }
        };
    }

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

    #[inline]
    pub fn load(&mut self, reader_base: &mut ReaderBase) {
        let mut file = File::open("books.yaml").unwrap();
        let mut string = String::new();
        file.read_to_string(&mut string).unwrap();

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
