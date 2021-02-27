extern crate fltk;

use fltk::dialog::alert;

use crate::{
    books::{book::Book, book_sys::BookSystem, date::Date},
    reading::read_base::ReaderBase,
    Lang,
};

/// Function that checks if input was empty

#[inline]
pub(crate) fn empty_inp_reader(inp: &Vec<String>, lang: Lang) -> bool {
    unsafe {
        return if inp.get_unchecked(0).is_empty() {
            alert(
                500,
                500,
                match lang {
                    Lang::English => "'Name' is empty",
                    Lang::Russian => "'Имя' пусто",
                },
            );
            true
        } else if inp.get_unchecked(1).is_empty() {
            alert(
                500,
                500,
                match lang {
                    Lang::English => "'2-nd Name' is empty",
                    Lang::Russian => "'Фамилия' пусто",
                },
            );
            true
        } else if inp.get_unchecked(2).is_empty() {
            alert(
                500,
                500,
                match lang {
                    Lang::English => "'Mid. Name' is empty",
                    Lang::Russian => "'Фамилия' пусто",
                },
            );
            true
        } else if inp.get_unchecked(3).is_empty() {
            alert(
                500,
                500,
                match lang {
                    Lang::English => "'Date of Birth' is empty",
                    Lang::Russian => "'Дата Рождения' пусто",
                },
            );
            true
        } else {
            false
        };
    }
}

/// Function that checks if input is correct.
/// Returns index of book, if it exists.
/// or calls alert and returns error

#[inline]
pub(crate) fn check_reader(
    reader_base: &ReaderBase,
    reader: &Vec<String>,
    lang: Lang,
) -> Result<usize, ()> {
    let age;
    let ind;

    unsafe {
        if empty_inp_reader(reader, lang) {
            return Err(());
        }

        let it = reader.last().unwrap().split('/').collect::<Vec<_>>();

        if it.len() != 3 {
            alert(
                500,
                500,
                match lang {
                    Lang::English => "'Age' input error",
                    Lang::Russian => "Ошибка ввода 'Возраста'",
                },
            );
            return Err(());
        }

        let mut it = it.into_iter();

        match it.next().unwrap().trim().parse::<u8>() {
            Ok(day) => match it.next().unwrap().trim().parse::<u8>() {
                Ok(month) => match it.next().unwrap().trim().parse::<u16>() {
                    Ok(year) => match Date::new(day, month, year) {
                        Err(_) => {
                            alert(
                                500,
                                500,
                                match lang {
                                    Lang::English => "Incorrect date",
                                    Lang::Russian => "Некорректная дата",
                                },
                            );
                            return Err(());
                        }

                        Ok(x) => age = x,
                    },

                    Err(_) => {
                        alert(
                            500,
                            500,
                            match lang {
                                Lang::English => "'Age' input error",
                                Lang::Russian => "Ошибка ввода 'Возраста'",
                            },
                        );
                        return Err(());
                    }
                },

                Err(_) => {
                    alert(
                        500,
                        500,
                        match lang {
                            Lang::English => "'Age' input error",
                            Lang::Russian => "Ошибка ввода 'Возраста'",
                        },
                    );
                    return Err(());
                }
            },

            Err(_) => {
                alert(
                    500,
                    500,
                    match lang {
                        Lang::English => "'Age' input error",
                        Lang::Russian => "Ошибка ввода 'Возраста'",
                    },
                );
                return Err(());
            }
        }

        ind = reader_base.find_reader(
            reader.get_unchecked(0),
            reader.get_unchecked(1),
            reader.get_unchecked(2),
            age,
        );
    }

    return match ind {
        None => {
            alert(
                500,
                500,
                match lang {
                    Lang::English => "Reader isn't found",
                    Lang::Russian => "Читатель не найден",
                },
            );
            Err(())
        }

        Some(i) => Ok(i),
    };
}

/// Function that returns index of simple book.
/// Panics if book is not in vec of books.

#[inline]
pub(crate) fn get_book_ind(book_system: &BookSystem, book: *const Book) -> usize {
    if book.is_null() {
        panic!("nullptr in actions/read/get_book_ind");
    }

    unsafe {
        match book_system.find_book(
            &(*book).title().to_string(),
            &(*book).author().to_string(),
            (*book).pages(),
        ) {
            None => panic!("Index out of range"),
            Some(ind) => {
                (*(**book_system.books.get_unchecked(ind)).borrow().books)
                    .iter()
                    .position(|x| &*(**x).borrow() as *const Book == book)
                    .unwrap()
                    + 1
            }
        }
    }
}
