use crate::books::{book::Book, date::Date, ResultSelf};

use std::{
    cell::RefCell,
    fmt::{Debug, Formatter, Result},
    rc::{Rc, Weak},
};

/// Reader structure, which contains
/// name, family, father, age, ~~simple~~ books he' d read
/// and books which he is reading now (or None)

pub struct Reader {
    pub(crate) name: String,
    pub(crate) family: String,
    pub(crate) father: String,
    pub(crate) birth: Date,
    pub(crate) books: Vec<Weak<RefCell<Book>>>,
    pub(crate) reading: Option<Vec<Weak<RefCell<Book>>>>,
}

impl Drop for Reader {
    /// Destructor for Reader.
    /// It's used to debug code

    #[inline]
    fn drop(&mut self) {
        println!(
            "Readers {} {} {} is deleted",
            self.name, self.family, self.father
        );
    }
}

impl Debug for Reader {
    /// Print for Reader.
    /// It's used to debug code

    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Reader")
            .field("name", &self.name)
            .field("family", &self.family)
            .field("father", &self.father)
            .field("date of birth", &self.birth.to_string())
            .field(
                "books",
                &self
                    .books
                    .iter()
                    .map(|x| {
                        (*(*x).upgrade().unwrap()).borrow().title().to_string()
                            + " "
                            + (*(*x).upgrade().unwrap()).borrow().author().as_str()
                            + " "
                            + format!("{}", (*(*x).upgrade().unwrap()).borrow().pages()).as_str()
                    })
                    .collect::<Vec<_>>(),
            )
            .finish()
    }
}

impl PartialEq for Reader {
    /// Compare Reader by == / !=
    /// by their params

    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.family == other.family
            && self.father == other.father
            && self.birth == other.birth
    }
}

/// Compare Reader by == / !=
/// by their params

impl Eq for Reader {}

impl Clone for Reader {
    /// Clones reader
    /// Clears all books pointer.
    /// After that Book System also **MUST** be cloned

    #[inline]
    fn clone(&self) -> Self {
        Reader {
            name: self.name.clone(),
            family: self.family.clone(),
            father: self.father.clone(),
            birth: self.birth.clone(),
            books: vec![],
            reading: None,
        }
    }

    /// Clones reader from another reader

    #[inline]
    fn clone_from(&mut self, other: &Self) {
        *self = other.clone()
    }
}

impl Reader {
    /// Creates new Reader with chosen
    /// 1-st name, 2-nd name, mid. name and age.
    /// It has no books

    #[inline]
    pub(crate) const fn new(
        new_name: String,
        new_family: String,
        new_father: String,
        new_birth: Date,
    ) -> Self {
        Reader {
            name: new_name,
            family: new_family,
            father: new_father,
            birth: new_birth,
            books: vec![],
            reading: None,
        }
    }

    #[inline]
    pub(crate) fn age(&self) -> u16 {
        Date::from(chrono::Local::now()) - self.birth
    }

    /// Find book by smart pointer.
    /// If ok, returns index of the first occurrence,
    /// else none

    #[inline]
    #[allow(dead_code)]
    pub(crate) fn find_book_first(&self, book: &Rc<RefCell<Book>>) -> Option<usize> {
        self.books
            .iter()
            .position(|x| x.upgrade().unwrap().as_ptr() == book.as_ptr())
    }

    /// Find book by smart pointer.
    /// If ok, returns index of the last occurrence,
    /// else none

    #[inline]
    #[allow(dead_code)]
    pub(crate) fn find_book_last(&self, book: &Rc<RefCell<Book>>) -> Option<usize> {
        self.books
            .iter()
            .rev()
            .position(|x| x.upgrade().unwrap().as_ptr() == book.as_ptr())
    }

    /// Function, that uses after giving book to reader.
    /// Adds book to books and reading params without checks
    ///
    /// #Safety
    /// Reader can read only one book with same title, author and pages.

    #[inline]
    pub(crate) unsafe fn start_reading_unchecked(
        &mut self,
        book: &Rc<RefCell<Book>>,
        date: &Date,
    ) -> &mut Self {
        match &mut self.reading {
            None => self.reading = Some(vec![Rc::downgrade(&book)]),
            Some(books) => {
                if let Err(ind) = books.binary_search_by(|b| {
                    ((*b.upgrade().unwrap()).borrow().readers.last().unwrap().1)
                        .1
                        .cmp(date)
                }) {
                    books.insert(ind, Rc::downgrade(&book));
                }
            }
        }

        self.books.push(Rc::downgrade(&book));
        self
    }

    /// Function, that uses after giving book to reader.
    /// Adds book to books and reading params.
    ///
    /// **NOTE**: Reader can read only one book with same title, author and pages.

    #[inline]
    pub(crate) fn start_reading(
        &mut self,
        book: &Rc<RefCell<Book>>,
        date: &Date,
    ) -> ResultSelf<Self> {
        match &mut self.reading {
            None => self.reading = Some(vec![Rc::downgrade(&book)]),
            Some(books) => {
                match books.iter().position(|b| {
                    (*b.upgrade().unwrap()).borrow().title() == (**book).borrow().title()
                        && (*b.upgrade().unwrap()).borrow().author() == (**book).borrow().author()
                        && (*b.upgrade().unwrap()).borrow().pages() == (**book).borrow().pages()
                }) {
                    None => books.insert(
                        books
                            .binary_search_by(|b| {
                                ((*b.upgrade().unwrap()).borrow().readers.last().unwrap().1)
                                    .1
                                    .cmp(date)
                            })
                            .unwrap_err(),
                        Rc::downgrade(&book),
                    ),
                    Some(_) => return Err(0), // already reading this kind of book
                }
            }
        }

        self.books.push(Rc::downgrade(&book));
        Ok(self)
    }

    /// Function, that uses after giving book to reader.
    /// Sets reading param as None if there are no books to read

    #[inline]
    pub(crate) fn finish_reading(&mut self, book: Weak<RefCell<Book>>) {
        let bs = self
            .reading
            .as_ref()
            .unwrap()
            .binary_search_by(|b| {
                ((*b.upgrade().unwrap()).borrow().readers.last().unwrap().1)
                    .1
                    .cmp(
                        &((*book.upgrade().unwrap())
                            .borrow()
                            .readers
                            .last()
                            .unwrap()
                            .1)
                            .1,
                    )
            })
            .unwrap();

        self.reading.as_mut().unwrap().remove(bs);

        if self.reading.as_ref().unwrap().is_empty() {
            self.reading = None;
        }
    }

    /// Removes book

    #[inline]
    pub(crate) fn remove_book(&mut self, book: &mut Book) -> &mut Self {
        if book.is_using
            && (*(book.readers.last().unwrap().0).upgrade().unwrap()).as_ptr() == self as *mut _
        {
            let ind = self
                .reading
                .as_ref()
                .unwrap()
                .iter()
                .position(|b| &*(*b.upgrade().unwrap()).borrow() as *const _ == book as *const _)
                .unwrap();

            self.reading.as_mut().unwrap().remove(ind);
            self.reading = None;
            book.is_using = false;
        }

        self.books = self
            .books
            .clone()
            .into_iter()
            .filter(|x| (*(*x).upgrade().unwrap()).as_ptr() != book as *mut _)
            .collect();

        self
    }

    /// Removes all simple books.
    /// Used to delete reader

    #[inline]
    pub(crate) fn remove_all_books(&mut self) -> &mut Self {
        while !self.books.is_empty() {
            if (*self.books.last().unwrap().upgrade().unwrap())
                .borrow()
                .is_using
                && (*((*self.books.last().unwrap().upgrade().unwrap())
                    .borrow()
                    .readers
                    .last()
                    .unwrap()
                    .0)
                    .upgrade()
                    .unwrap())
                .as_ptr()
                    == self as *mut _
            {
                (*self.books.last().unwrap().upgrade().unwrap())
                    .borrow_mut()
                    .is_using = false;
            }

            (*self.books.last().unwrap().upgrade().unwrap())
                .borrow_mut()
                .remove_reader(self);
            self.books.pop();
        }
        self
    }

    /// Changes reader's name

    #[inline]
    pub(crate) fn change_name(&mut self, new_name: String) -> ResultSelf<Self> {
        return if new_name.is_empty() {
            Err(0)
        } else {
            self.name = new_name;
            Ok(self)
        };
    }

    /// Changes reader's 2-nd name

    #[inline]
    pub(crate) fn change_family(&mut self, new_family: String) -> ResultSelf<Self> {
        return if new_family.is_empty() {
            Err(0)
        } else {
            self.family = new_family;
            Ok(self)
        };
    }

    /// Changes reader's mid. name

    #[inline]
    pub(crate) fn change_father(&mut self, new_father: String) -> ResultSelf<Self> {
        return if new_father.is_empty() {
            Err(0)
        } else {
            self.father = new_father;
            Ok(self)
        };
    }

    /// Changes reader's birthday

    #[inline]
    pub(crate) fn change_age(&mut self, new_birth: Date) -> &mut Self {
        self.birth = new_birth;
        self
    }
}
