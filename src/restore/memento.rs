use crate::{
    books::{book_sys::BookSystem, genres::Genres},
    reading::read_base::ReaderBase,
};

/// Collects data about
/// reader base, book system and genres

#[derive(Debug)]
pub(crate) struct Memento {
    pub(crate) state: (ReaderBase, BookSystem, Genres),
}

impl Clone for Memento {
    /// Clones Memento with new
    /// Reader Base, Book System and Genres
    /// (with new smart pointers)

    #[inline]
    fn clone(&self) -> Self {
        Memento::new(&self.state.0, &self.state.1, &self.state.2)
    }
}

impl Memento {
    /// Creates new state

    #[inline]
    pub fn new(reader_base: &ReaderBase, book_system: &BookSystem, genres: &Genres) -> Self {
        let rb = reader_base.clone();
        let bs = book_system.clone(&rb);

        Memento {
            state: (rb, bs, genres.clone()),
        }
    }

    /// Gets state

    #[inline]
    pub fn get_state(&self) -> &(ReaderBase, BookSystem, Genres) {
        &self.state
    }
}
