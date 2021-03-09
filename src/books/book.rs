extern crate chrono;
extern crate fltk;

use crate::{
    actions::read::utils::get_book_ind,
    books::{book_sys::BookSystem, date::Date, the_book::TheBook, ResultSelf},
    change::{input2::Input2, Inputable},
    reading::reader::Reader,
    Lang,
};

use chrono::Datelike;

use fltk::{app, app::App, input::IntInput, prelude::*};

use std::{
    cell::RefCell,
    fmt::{Debug, Formatter, Result},
    rc::{Rc, Weak},
};

/// Simple Book structure, which contains
/// title, author, amount of pages, using status,
/// location (number of cabinet and it's shelf)
/// and readers with start and finish dates

#[derive(Default)]
pub struct Book {
    pub(crate) the_book: Option<Weak<RefCell<TheBook>>>,
    pub(crate) is_using: bool,
    pub(crate) cabinet: u16,
    pub(crate) shelf: u8,
    pub(crate) readers: Vec<(Weak<RefCell<Reader>>, (Date, Date))>,
}

impl Drop for Book {
    /// Destructor for simple book.
    /// It is used to debug code.

    #[inline]
    fn drop(&mut self) {
        println!(
            "Book ({} cab, {} shelf) is deleted",
            self.cabinet, self.shelf
        )
    }
}

impl Debug for Book {
    /// Print for simple book.
    /// It is used to debug code.

    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Book")
            .field("title", &self.title())
            .field("author", &self.author())
            .field("pages", &self.pages())
            .field("is using", &self.is_using)
            .field("cabinet", &self.cabinet)
            .field("shelf", &self.shelf)
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
                            + format!(
                                "{}",
                                (*((*x).0).upgrade().unwrap()).borrow().birth.to_string()
                            )
                            .as_str()
                    })
                    .collect::<Vec<_>>(),
            )
            .finish()
    }
}

impl PartialEq for Book {
    /// Compares simple books on equality with pointers' address

    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self as *const Book == other as *const Book
    }
}

/// Compares simple books on equality with pointers' address

impl Eq for Book {}

impl Book {
    /// Constructs new simple book.
    /// By default it has no readers and it isn't used.
    /// If there are some input errors,
    /// it'll return None

    #[inline]
    pub(crate) fn new(the_book: Rc<RefCell<TheBook>>, app: &App, lang: Lang) -> Option<Self> {
        let (s2, r2) = app::channel();
        let mut inp = Input2::<IntInput, IntInput>::new(
            match lang {
                Lang::English => "Location",
                Lang::Russian => "Местонахождения",
            },
            match lang {
                Lang::English => "Cabinet's number",
                Lang::Russian => "Номер шкафа",
            },
            match lang {
                Lang::English => "Shelf's number",
                Lang::Russian => "Номер полки",
            },
        );

        inp.show();
        (*inp.ok).borrow_mut().emit(s2, true);

        while app.wait() {
            if let Some(message) = r2.recv() {
                if message {
                    inp.hide();

                    if let Ok(location) = inp.set_input() {
                        return Some(Book {
                            the_book: Some(Rc::downgrade(&the_book)),
                            is_using: false,
                            cabinet: location.first().unwrap().trim().parse().unwrap(),
                            shelf: location.last().unwrap().trim().parse().unwrap(),
                            readers: vec![],
                        });
                    }
                }
                return None;
            } else if !inp.shown() {
                return None;
            }
        }

        None
    }

    /// Constructs book with known params.

    #[inline]
    #[allow(dead_code)]
    pub(crate) fn restore(
        _the_book: Rc<RefCell<TheBook>>,
        _is_using: bool,
        _cabinet: u16,
        _shelf: u8,
    ) -> Self {
        Book {
            the_book: Some(Rc::downgrade(&_the_book)),
            is_using: _is_using,
            cabinet: _cabinet,
            shelf: _shelf,
            readers: vec![],
        }
    }

    /// Gets title

    #[inline]
    pub(crate) fn title(&self) -> String {
        (*self.the_book.as_ref().unwrap().upgrade().unwrap())
            .borrow()
            .title
            .clone()
    }

    /// Gets author

    #[inline]
    pub(crate) fn author(&self) -> String {
        (*self.the_book.as_ref().unwrap().upgrade().unwrap())
            .borrow()
            .author
            .clone()
    }

    /// Gets amount of pages

    #[inline]
    pub(crate) fn pages(&self) -> u16 {
        (*self.the_book.as_ref().unwrap().upgrade().unwrap())
            .borrow()
            .pages
    }

    /// Searches reader.
    /// If it isn't found, it' ll return the amount of all readers.
    /// else it will return reader index of the first occurrence

    #[inline]
    #[allow(dead_code)]
    pub(crate) fn find_reader_first(&self, reader: &Rc<RefCell<Reader>>) -> Option<usize> {
        self.readers
            .iter()
            .position(|x| (*(x.0).upgrade().unwrap()).as_ptr() == (**reader).as_ptr())
    }

    /// Searches reader.
    /// If it isn't found, it' ll return the amount of all readers.
    /// else it will return reader index of the last occurrence

    #[inline]
    #[allow(dead_code)]
    pub(crate) fn find_reader_last(&self, reader: &Rc<RefCell<Reader>>) -> Option<usize> {
        self.readers
            .iter()
            .rev()
            .position(|x| (*(x.0).upgrade().unwrap()).as_ptr() == (**reader).as_ptr())
    }

    /// Removes reader

    #[inline]
    pub(crate) fn remove_reader(&mut self, reader: &Reader) -> &mut Self {
        if self.is_using {
            (*self.readers.last_mut().unwrap().0.upgrade().unwrap())
                .borrow_mut()
                .reading = None;
            self.is_using = false;
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
    pub(crate) fn remove_all_readers(&mut self) -> &mut Self {
        if self.is_using {
            let ind = (*(self.readers.last_mut().unwrap().0).upgrade().unwrap())
                .borrow_mut()
                .reading
                .as_ref()
                .unwrap()
                .iter()
                .position(|b| (*b.upgrade().unwrap()).as_ptr() == self as *mut _)
                .unwrap();

            (*(self.readers.last_mut().unwrap().0).upgrade().unwrap())
                .borrow_mut()
                .reading
                .as_mut()
                .unwrap()
                .remove(ind);

            if {
                let check = (*(self.readers.last().unwrap().0).upgrade().unwrap())
                    .borrow()
                    .reading
                    .as_ref()
                    .unwrap()
                    .is_empty();
                check
            } {
                (*(self.readers.last().unwrap().0).upgrade().unwrap())
                    .borrow_mut()
                    .reading = None;
            }
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
    pub(crate) fn start_reading(&mut self, reader: &Rc<RefCell<Reader>>, date: Date) -> &mut Self {
        let now = chrono::Local::now();

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
    pub(crate) fn finish_reading(&mut self) -> ResultSelf<Self> {
        self.is_using = false;

        let now = Date::from(chrono::Local::now());
        let was = ((*self.readers.last().unwrap()).1).1;

        if now > was {
            Err(1) // Reader is late
        } else {
            ((*self.readers.last_mut().unwrap()).1).1 = now;
            Ok(self)
        }
    }

    /// Changes cabinet's and shelf's number
    /// where book is located.

    #[inline]
    pub(crate) fn change_location(&mut self, new_cabinet: u16, new_shelf: u8) -> &mut Self {
        self.cabinet = new_cabinet;
        self.shelf = new_shelf;
        self
    }

    /// Clones simple book
    /// with empty readers

    #[inline]
    pub(crate) fn clone(&self, the_book: Rc<RefCell<TheBook>>) -> Self {
        Book {
            the_book: Some(Rc::downgrade(&the_book)),
            is_using: self.is_using,
            cabinet: self.cabinet,
            shelf: self.shelf,
            readers: vec![],
        }
    }

    /// Represent Book as String with next format:
    ///
    /// {title} {author} {amount of pages} ({order number})

    #[inline]
    pub(crate) fn to_string(&self, book_system: &BookSystem) -> String {
        format!(
            "{} {} {} ({})",
            self.title(),
            self.author(),
            self.pages(),
            get_book_ind(book_system, self as *const _)
        )
    }
}
