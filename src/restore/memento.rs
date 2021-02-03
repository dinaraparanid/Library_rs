use crate::{
    books::{book_sys::BookSystem, genres::Genres},
    reading::read_base::ReaderBase,
};

/// Collects data about
/// reader base, book system and genres

#[derive(Debug, Clone)]
pub(crate) struct Memento {
    pub(crate) state: (ReaderBase, BookSystem, Genres),
}

impl Memento {
    /// Creates new state

    #[inline]
    pub fn new(reader_base: &ReaderBase, book_system: &BookSystem, genres: &Genres) -> Self {
        Memento {
            state: (reader_base.clone(), book_system.clone(), genres.clone()),
        }
    }

    /// Gets state

    #[inline]
    pub fn get_state(&self) -> &(ReaderBase, BookSystem, Genres) {
        &self.state
    }
}
