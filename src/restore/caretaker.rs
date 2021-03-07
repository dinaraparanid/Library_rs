extern crate fltk;

use crate::{
    books::{book_sys::BookSystem, genres::Genres},
    reading::read_base::ReaderBase,
    restore::memento::Memento,
    Lang,
};

use fltk::dialog::{alert, message};

#[derive(Debug, Default)]
pub struct Caretaker {
    mementos: Vec<Memento>,
    ind: usize,
}

/// Handles with all mementos
/// by returning states (next or last)

impl Caretaker {
    /// Creates new caretaker

    #[inline]
    pub const fn new() -> Self {
        Caretaker {
            mementos: vec![],
            ind: 0,
        }
    }

    /// Amount of mementos

    #[inline]
    pub(crate) fn len(&self) -> usize {
        self.mementos.len()
    }

    /// Gets reference to previous memento

    #[inline]
    pub fn get_memento_back(
        &mut self,
        reader_base: &mut ReaderBase,
        book_system: &mut BookSystem,
        genres: &mut Genres,
        lang: Lang,
    ) {
        if self.ind == 0 {
            alert(
                500,
                500,
                match lang {
                    Lang::English => "First version",
                    Lang::Russian => "Первая версия",
                },
            );
        } else {
            if self.ind == self.len() {
                self.add_memento(reader_base, book_system, genres);
                self.ind -= 1;
            }

            self.ind -= 1;
            let mem = unsafe { self.mementos.get_unchecked(self.ind) }.get_state();

            *reader_base = mem.0.clone();
            *book_system = mem.1.clone(reader_base);
            *genres = mem.2.clone();

            reader_base.save();
            book_system.save();
            genres.save();

            message(
                500,
                500,
                match lang {
                    Lang::English => "Successfully restored",
                    Lang::Russian => "Успешно восстановленно",
                },
            );
        }
    }

    /// Gets reference to next memento

    #[inline]
    pub fn get_memento_forward(
        &mut self,
        reader_base: &mut ReaderBase,
        book_system: &mut BookSystem,
        genres: &mut Genres,
        lang: Lang,
    ) {
        return if self.len() == 0 || self.ind == self.len() - 1 {
            alert(
                500,
                500,
                match lang {
                    Lang::English => "Last version",
                    Lang::Russian => "Последняя версия",
                },
            );
        } else {
            self.ind += 1;
            let mem = unsafe { self.mementos.get_unchecked(self.ind) }.get_state();

            *reader_base = mem.0.clone();
            *book_system = mem.1.clone(reader_base);
            *genres = mem.2.clone();

            reader_base.save();
            book_system.save();
            genres.save();

            message(
                500,
                500,
                match lang {
                    Lang::English => "Successfully restored",
                    Lang::Russian => "Успешно восстановленно",
                },
            )
        };
    }

    /// Adds new state

    #[inline]
    pub fn add_memento(
        &mut self,
        reader_base: &ReaderBase,
        book_system: &BookSystem,
        genres: &Genres,
    ) -> &mut Self {
        self.ind += 1;
        self.mementos
            .resize(self.ind, Memento::new(reader_base, book_system, genres));
        self
    }

    /// Removes last memento

    #[inline]
    pub(crate) fn pop(&mut self) -> Option<Memento> {
        return match self.mementos.pop() {
            None => None,
            Some(val) => {
                self.ind -= 1;
                Some(val)
            }
        };
    }
}
