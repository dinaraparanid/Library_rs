use crate::books::book::Book;
use crate::books::ResultSelf;
use std::cell::RefCell;
use std::fmt::{Debug, Formatter, Result};
use std::rc::{Rc, Weak};

/// Reader structure, which contains
/// name, family, father, age, ~~simple~~ books he' d read
/// and book which he is reading now (or None)

pub(crate) struct Reader {
    pub(crate) name: String,
    pub(crate) family: String,
    pub(crate) father: String,
    pub(crate) age: u8,
    pub(crate) books: Vec<Weak<RefCell<Book>>>,
    pub(crate) reading: Option<Weak<RefCell<Book>>>,
}
/// Destructor for Reader.
/// It's used to debug code

impl Drop for Reader {
    #[inline]
    fn drop(&mut self) {
        println!(
            "Readers {} {} {} is deleted",
            self.name, self.family, self.father
        );
    }
}

/// Print for Reader.
/// It's used to debug code

impl Debug for Reader {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Reader")
            .field("name", &self.name)
            .field("family", &self.family)
            .field("father", &self.father)
            .field("age", &self.age)
            .field(
                "books",
                &self
                    .books
                    .iter()
                    .map(|x| {
                        (*(*x).upgrade().unwrap()).borrow().title.clone()
                            + " "
                            + (*(*x).upgrade().unwrap()).borrow().author.clone().as_str()
                            + " "
                            + format!("{}", (*(*x).upgrade().unwrap()).borrow().pages).as_str()
                    })
                    .collect::<Vec<String>>(),
            )
            .finish()
    }
}

/// Compare Reader by == / !=

impl PartialEq for Reader {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.family == other.family
            && self.father == other.father
            && self.age == other.age
    }
}

/// Compare Reader by == / !=

impl Eq for Reader {}

impl Reader {
    /// Creates new Reader with chosen
    /// 1-st name, 2-nd name, mid. name and age.
    /// It has no books

    #[inline]
    pub fn new(new_name: String, new_family: String, new_father: String, new_age: u8) -> Self {
        Reader {
            name: new_name,
            family: new_family,
            father: new_father,
            age: new_age,
            books: vec![],
            reading: None,
        }
    }

    /// Find book by smart pointer.
    /// If ok, returns index,
    /// else returns amount of read books

    #[inline]
    pub fn find_book(&self, book: &Rc<RefCell<Book>>) -> usize {
        for i in 0..self.books.len() {
            let book_ptr;

            unsafe {
                book_ptr = self.books.get_unchecked(i).upgrade().unwrap().as_ptr();
            }

            if book_ptr == book.as_ptr() {
                return i;
            }
        }
        self.books.len()
    }

    /// Function, that uses after giving book to reader.
    /// Adds book to books and reading params

    #[inline]
    pub fn start_reading(&mut self, book: &Rc<RefCell<Book>>) -> &mut Self {
        self.books.push(Rc::downgrade(&book));
        self.reading = Some(Rc::downgrade(&book));
        self
    }

    /// Function, that uses after giving book to reader.
    /// Sets reading param as None

    #[inline]
    pub fn finish_reading(&mut self) {
        self.reading = None;
    }

    /// Removes book by raw pointer

    #[inline]
    pub fn remove_book(&mut self, book: *mut Book) -> &mut Self {
        if book.is_null() {
            panic!("nullptr in reader remove_book");
        }

        unsafe {
            if (*book).is_using
                && *(*((*book).readers.last().unwrap().0).upgrade().unwrap()).borrow() == *self
            {
                (*book).is_using = false;
            }
        }

        self.books = self
            .books
            .clone()
            .into_iter()
            .filter(|x| (*(*x).upgrade().unwrap()).as_ptr() != book)
            .collect();

        self
    }

    /// Removes all simple books.
    /// Used to delete reader

    #[inline]
    pub fn remove_all_books(&mut self) -> &mut Self {
        while !self.books.is_empty() {
            if (*self.books.last().unwrap().upgrade().unwrap())
                .borrow()
                .is_using
                && *(*((*self.books.last().unwrap().upgrade().unwrap())
                    .borrow()
                    .readers
                    .last()
                    .unwrap()
                    .0)
                    .upgrade()
                    .unwrap())
                .borrow()
                    == *self
            {
                (*self.books.last().unwrap().upgrade().unwrap())
                    .borrow_mut()
                    .is_using = false;
            }

            (*self.books.last().unwrap().upgrade().unwrap())
                .borrow_mut()
                .remove_reader(self as *mut Reader);
            self.books.pop();
        }
        self
    }

    /// Changes reader's name

    #[inline]
    pub fn change_name(&mut self, new_name: String) -> ResultSelf<Self> {
        return if new_name.is_empty() {
            Err(0)
        } else {
            self.name = new_name;
            Ok(self)
        };
    }

    /// Changes reader's 2-nd name

    #[inline]
    pub fn change_family(&mut self, new_family: String) -> ResultSelf<Self> {
        return if new_family.is_empty() {
            Err(0)
        } else {
            self.family = new_family;
            Ok(self)
        };
    }

    /// Changes reader's mid. name

    #[inline]
    pub fn change_father(&mut self, new_father: String) -> ResultSelf<Self> {
        return if new_father.is_empty() {
            Err(0)
        } else {
            self.father = new_father;
            Ok(self)
        };
    }

    /// Changes reader's age

    #[inline]
    pub fn change_age(&mut self, new_age: u8) -> &mut Self {
        self.age = new_age;
        self
    }
}
