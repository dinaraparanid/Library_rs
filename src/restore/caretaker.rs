extern crate fltk;

use crate::{
    books::{book_sys::BookSystem, genres::Genres},
    reading::read_base::ReaderBase,
    restore::memento::Memento,
};

use fltk::dialog::{alert, message};

#[derive(Debug)]
pub struct Caretaker {
    mementos: Vec<Memento>,
    ind: usize,
}

/// Handles with all mementos
/// by returning states (next or last)

impl Caretaker {
    /// Creates new caretaker

    #[inline]
    pub fn new(reader_base: &ReaderBase, book_system: &BookSystem, genres: &Genres) -> Self {
        Caretaker {
            mementos: vec![Memento::new(reader_base, book_system, genres)],
            ind: 0,
        }
    }

    /// Gets reference to previous memento

    #[inline]
    pub fn get_memento_back(
        &mut self,
        reader_base: &mut ReaderBase,
        book_system: &mut BookSystem,
        genres: &mut Genres,
    ) {
        if self.ind == 0 {
            alert(500, 500, "Last version");
        } else {
            self.ind -= 1;
            let mem = unsafe { self.mementos.get_unchecked(self.ind) }.get_state();

            *reader_base = mem.0.clone();
            *book_system = mem.1.clone();
            *genres = mem.2.clone();

            reader_base.save();
            book_system.save();
            genres.save();

            message(500, 500, "Successfully restored")
        }
    }

    /// Gets reference to next memento

    #[inline]
    pub fn get_memento_forward(
        &mut self,
        reader_base: &mut ReaderBase,
        book_system: &mut BookSystem,
        genres: &mut Genres,
    ) {
        return if self.ind == self.mementos.len() - 1 {
            alert(500, 500, "First version");
        } else {
            self.ind += 1;
            let mem = unsafe { self.mementos.get_unchecked(self.ind) }.get_state();

            *reader_base = mem.0.clone();
            *book_system = mem.1.clone();
            *genres = mem.2.clone();

            reader_base.save();
            book_system.save();
            genres.save();

            message(500, 500, "Successfully restored")
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
        self.mementos
            .resize(self.ind + 1, Memento::new(reader_base, book_system, genres));
        self.mementos
            .push(Memento::new(reader_base, book_system, genres));
        self.ind += 1;
        self
    }
}