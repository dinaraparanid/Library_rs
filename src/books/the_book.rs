use crate::books::book::Book;
use crate::books::{BookInterface, ResultSelf};
use crate::reading::reader::Reader;
use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

/// Interface Book structure, which contains
/// title, author, amount of pages, and simple books

pub(crate) struct TheBook {
    pub(crate) title: String,
    pub(crate) author: String,
    pub(crate) pages: u16,
    pub(crate) books: Vec<Rc<RefCell<Book>>>,
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
            .collect();
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
            .collect();
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
            .collect();
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
    pub fn get_unused(&self) -> Option<usize> {
        self.books.iter().position(|x| !(**x).borrow().is_using)
    }

    /// Finds using book by reader

    #[inline]
    pub fn find_by_reader(&self, reader: &Rc<RefCell<Reader>>) -> Option<usize> {
        self.books.iter().position(|x| {
            (**x).borrow().is_using
                && ((**x).borrow().readers.last().unwrap())
                    .0
                    .ptr_eq(&Rc::downgrade(reader))
        })
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
