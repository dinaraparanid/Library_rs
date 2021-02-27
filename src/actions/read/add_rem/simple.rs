extern crate fltk;

use crate::{
    books::{book_sys::BookSystem, genres::Genres},
    reading::read_base::ReaderBase,
    restore::caretaker::Caretaker,
    Lang,
};

use fltk::dialog::alert;

/// Removes already known reader

#[inline]
pub(crate) fn remove_reader_simple(
    ind: usize,
    reader_base: &mut ReaderBase,
    book_system: &mut BookSystem,
    genres: &Genres,
    caretaker: &mut Caretaker,
    lang: Lang,
) {
    caretaker.add_memento(reader_base, book_system, genres);

    match reader_base.remove_reader(ind) {
        Ok(_) => {
            fltk::dialog::message(
                500,
                500,
                match lang {
                    Lang::English => "Successfully removed",
                    Lang::Russian => "Успешно удалён",
                },
            );

            reader_base.save();
            book_system.save();
        }

        Err(_) => {
            alert(
                500,
                500,
                match lang {
                    Lang::English => "Reader is not found",
                    Lang::Russian => "Читатель не найден",
                },
            );
            caretaker.pop();
            return;
        }
    }
}
