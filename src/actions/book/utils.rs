extern crate fltk;

use crate::{books::book_sys::BookSystem, Lang};
use fltk::dialog::alert;

/// Function that checks
/// if input of book was empty

#[inline]
pub(crate) fn empty_inp_book(inp: &Vec<String>, lang: Lang) -> bool {
    return if unsafe { inp.get_unchecked(0).is_empty() } {
        alert(
            500,
            500,
            match lang {
                Lang::English => "'Title' is empty",
                Lang::Russian => "'Название' пусто",
            },
        );
        true
    } else if unsafe { inp.get_unchecked(1).is_empty() } {
        alert(
            500,
            500,
            match lang {
                Lang::English => "'Author' is empty",
                Lang::Russian => "'Автор' пусто",
            },
        );
        true
    } else if unsafe { inp.get_unchecked(2).is_empty() } {
        alert(
            500,
            500,
            match lang {
                Lang::English => "'Amount of pages' is empty",
                Lang::Russian => "'Количество страниц' пусто",
            },
        );
        true
    } else {
        false
    };
}

/// Function that checks if input is correct.
/// Returns index of book, if it exists.
/// or calls alert and returns error

#[inline]
pub(crate) fn check_book(
    book_system: &BookSystem,
    books: &Vec<String>,
    lang: Lang,
) -> Result<usize, ()> {
    return match empty_inp_book(books, lang) {
        true => Err(()),
        false => match unsafe { books.get_unchecked(2).trim().parse::<u16>() } {
            Ok(x) => match book_system.find_book(
                unsafe { books.get_unchecked(0) },
                unsafe { books.get_unchecked(1) },
                x,
            ) {
                Some(i) => Ok(i),

                None => {
                    alert(
                        500,
                        500,
                        match lang {
                            Lang::English => "Book isn't found",
                            Lang::Russian => "Книга не найдена",
                        },
                    );
                    Err(())
                }
            },

            Err(_) => {
                alert(
                    500,
                    500,
                    match lang {
                        Lang::English => "Amount of Pages input error",
                        Lang::Russian => "Ошибка ввода количества страниц",
                    },
                );
                Err(())
            }
        },
    };
}
