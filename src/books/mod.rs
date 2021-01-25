pub(crate) mod book;
pub mod book_sys;
pub(crate) mod date;
pub mod genres;
pub(crate) mod the_book;

/// Error-handling type.
/// If everything is ok, it should return self (but it's not necessary),
/// else it will return err with code
/// (that's will help you to correctly understand error)

pub(crate) type ResultSelf<'a, T> = std::result::Result<&'a mut T, u8>;

/// Trait, which used to params of books
/// like title, author and amount of pages

pub(crate) trait BookInterface {
    fn change_title(&mut self, new_title: String) -> &mut Self;
    fn change_author(&mut self, new_author: String) -> &mut Self;
    fn change_pages(&mut self, new_pages: u16) -> &mut Self;
}
