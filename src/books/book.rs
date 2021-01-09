extern crate chrono;
use crate::books::date::Date;
use crate::books::{BookInterface, ResultSelf};
use crate::reading::reader::Reader;
use chrono::Datelike;
use std::cell::RefCell;
use std::fmt::{Debug, Formatter, Result};
use std::rc::{Rc, Weak};

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
    /// else it will return reader index of the first occurrence

    #[inline]
    pub fn find_reader_first(&self, reader: &Rc<RefCell<Reader>>) -> Option<usize> {
        self.readers
            .iter()
            .position(|x| (*(x.0).upgrade().unwrap()).as_ptr() == (**reader).as_ptr())
    }

    /// Searches reader.
    /// If it isn't found, it' ll return the amount of all readers.
    /// else it will return reader index of the last occurrence

    #[inline]
    pub fn find_reader_last(&self, reader: &Rc<RefCell<Reader>>) -> Option<usize> {
        self.readers
            .iter()
            .rev()
            .position(|x| (*(x.0).upgrade().unwrap()).as_ptr() == (**reader).as_ptr())
    }

    /// Removes reader

    #[inline]
    pub fn remove_reader(&mut self, reader: &Reader) -> &mut Self {
        if reader.reading.is_some() {
            (reader.reading.as_ref().unwrap().upgrade().unwrap())
                .borrow_mut()
                .is_using = false;
        }

        self.readers = self
            .readers
            .clone()
            .into_iter()
            .filter(|x| {
                ((*x).0).upgrade().unwrap().as_ptr() as *const Reader != reader as *const Reader
            })
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
                .remove_book(self);
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
