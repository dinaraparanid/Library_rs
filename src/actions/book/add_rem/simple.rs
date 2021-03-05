extern crate fltk;

use crate::{
    books::{book_sys::BookSystem, genres::Genres},
    change::{input1::Input1, Inputable},
    reading::read_base::ReaderBase,
    restore::caretaker::Caretaker,
    Lang,
};

use fltk::{app, app::App, dialog::alert, input::IntInput, prelude::*};

/// Function that removes
/// already known the book
/// and all simple books

#[inline]
pub(crate) fn remove_the_book_simple(
    ind: usize,
    book_system: &mut BookSystem,
    reader_base: &mut ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    lang: Lang,
) {
    caretaker.add_memento(reader_base, book_system, genres);

    match book_system.remove_book(ind) {
        Ok(_) => {
            fltk::dialog::message(
                500,
                500,
                match lang {
                    Lang::English => "Successfully removed",
                    Lang::Russian => "Успешно удалено",
                },
            );

            book_system.save();
            reader_base.save();
        }

        Err(_) => {
            alert(
                500,
                500,
                match lang {
                    Lang::English => "Book's number is incorrect",
                    Lang::Russian => "Номер книги некорректен",
                },
            );
            caretaker.pop().unwrap();
            return;
        }
    }
}

/// Function that adds simple books
/// to already known the book

#[inline]
pub(crate) fn add_books_simple(
    ind: usize,
    book_system: &mut BookSystem,
    reader_base: &ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) -> bool {
    let (s3, r3) = app::channel();
    let mut get_amount = Input1::<IntInput>::new(
        match lang {
            Lang::English => "Books amount",
            Lang::Russian => "Количество книг",
        },
        match lang {
            Lang::English => "Amount of books to add",
            Lang::Russian => "Количество добавляемых книг",
        },
    );

    caretaker.add_memento(reader_base, book_system, genres);

    get_amount.show();
    (*get_amount.ok).borrow_mut().emit(s3, true);

    while app.wait() {
        if let Some(msg) = r3.recv() {
            if msg {
                get_amount.hide();

                if let Ok(amount) = get_amount.set_input() {
                    return match amount.first().unwrap().trim().parse::<usize>() {
                        Ok(x) => match book_system.add_books(ind, x, app, lang) {
                            Ok(_) => {
                                fltk::dialog::message(
                                    500,
                                    500,
                                    match lang {
                                        Lang::English => "Successfully added",
                                        Lang::Russian => "Успешно добавлено",
                                    },
                                );
                                book_system.save();
                                true
                            }

                            Err(_) => {
                                alert(
                                    500,
                                    500,
                                    match lang {
                                        Lang::English => "Too much books",
                                        Lang::Russian => "Слишком много книг",
                                    },
                                );
                                caretaker.pop().unwrap();
                                false
                            }
                        },

                        Err(_) => {
                            alert(
                                500,
                                500,
                                match lang {
                                    Lang::English => "'Amount of books' input error",
                                    Lang::Russian => "Ошибка ввода количества книг",
                                },
                            );
                            caretaker.pop().unwrap();
                            false
                        }
                    };
                }
            }
            break;
        } else if !get_amount.shown() {
            caretaker.pop().unwrap();
            return false;
        }
    }

    false
}

/// Function that removes
/// *one* simple book by *index*
/// from known the book

#[inline]
pub(crate) fn remove_book_simple(
    index: usize,
    book_system: &mut BookSystem,
    reader_base: &mut ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) -> bool {
    let (s3, r3) = app::channel();
    let mut get_ind = Input1::<IntInput>::new(
        match lang {
            Lang::English => "Book's number",
            Lang::Russian => "Номер книги",
        },
        match lang {
            Lang::English => "Book's number",
            Lang::Russian => "Номер книги",
        },
    );

    caretaker.add_memento(reader_base, book_system, genres);

    get_ind.show();
    (*get_ind.ok).borrow_mut().emit(s3, true);

    while app.wait() {
        if let Some(msg) = r3.recv() {
            if msg {
                get_ind.hide();

                if let Ok(ind) = get_ind.set_input() {
                    return match ind.first().unwrap().trim().parse::<usize>() {
                        Ok(x) => {
                            if x == 0 {
                                alert(
                                    500,
                                    500,
                                    match lang {
                                        Lang::English => "Incorrect number of book",
                                        Lang::Russian => "Некорректный номер книги",
                                    },
                                );
                                caretaker.pop().unwrap();
                                false
                            } else {
                                match book_system.remove_one_book(index, x - 1) {
                                    Ok(_) => {
                                        fltk::dialog::message(
                                            500,
                                            500,
                                            match lang {
                                                Lang::English => "Successfully removed",
                                                Lang::Russian => "Успешно удалено",
                                            },
                                        );
                                        book_system.save();
                                        reader_base.save();
                                        true
                                    }

                                    Err(_) => {
                                        alert(
                                            500,
                                            500,
                                            match lang {
                                                Lang::English => "Incorrect number of book",
                                                Lang::Russian => "Некорректный номер книги",
                                            },
                                        );
                                        caretaker.pop().unwrap();
                                        false
                                    }
                                }
                            }
                        }

                        Err(_) => {
                            alert(
                                500,
                                500,
                                match lang {
                                    Lang::English => "Book's number input error",
                                    Lang::Russian => "Ошибка ввода номера книги",
                                },
                            );
                            caretaker.pop().unwrap();
                            false
                        }
                    };
                }
            }
            break;
        } else if !get_ind.shown() {
            caretaker.pop().unwrap();
            return false;
        }
    }

    false
}

/// Function that removes
/// one known simple book
/// from known the book

#[inline]
#[allow(dead_code)]
pub(crate) fn remove_book_simple2(
    index: usize,
    s_index: usize,
    book_system: &mut BookSystem,
    reader_base: &mut ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    lang: Lang,
) {
    caretaker.add_memento(reader_base, book_system, genres);

    unsafe {
        book_system.remove_one_book_unchecked(index, s_index);
    }

    fltk::dialog::message(
        500,
        500,
        match lang {
            Lang::English => "Successfully removed",
            Lang::Russian => "Успешно удалено",
        },
    );

    book_system.save();
    reader_base.save();
}

/// Adds The Book with known params

#[inline]
pub(crate) fn add_book_simple(
    the_book: &Vec<String>,
    book_system: &mut BookSystem,
    reader_base: &ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) {
    let (s, r) = app::channel();
    let mut am = Input1::<IntInput>::new(
        match lang {
            Lang::English => "Amount of Books",
            Lang::Russian => "Количество Книг",
        },
        match lang {
            Lang::English => "Set amount of books",
            Lang::Russian => "Укажите количество книг",
        },
    );

    caretaker.add_memento(reader_base, book_system, genres);

    am.show();
    (*am.ok).borrow_mut().emit(s, true);

    while app.wait() {
        if let Some(mes) = r.recv() {
            if mes {
                am.hide();

                if let Ok(amount) = am.set_input() {
                    match amount.first().unwrap().trim().parse::<usize>() {
                        Ok(amount) => match the_book.last().unwrap().trim().parse::<u16>() {
                            Ok(x) => {
                                match book_system.add_book(
                                    unsafe { the_book.get_unchecked(0).clone() },
                                    unsafe { the_book.get_unchecked(1).clone() },
                                    x,
                                    amount,
                                    app,
                                    lang,
                                ) {
                                    Ok(_) => {
                                        fltk::dialog::message(
                                            500,
                                            500,
                                            match lang {
                                                Lang::English => "Successfully added",
                                                Lang::Russian => "Успешно добавлено",
                                            },
                                        );
                                        book_system.save();
                                    }

                                    Err(_) => {
                                        alert(
                                            500,
                                            500,
                                            match lang {
                                                Lang::English => "Book with same parameters already exists",
                                                Lang::Russian => "Книга с предложенными параметрами уже сузествует",
                                            }
                                        );
                                        caretaker.pop().unwrap();
                                        return;
                                    }
                                }
                            }

                            Err(_) => {
                                alert(
                                    500,
                                    500,
                                    match lang {
                                        Lang::English => "Incorrect 'Amount of Pages' input",
                                        Lang::Russian => "Некорретный ввод количества страниц",
                                    },
                                );
                                caretaker.pop().unwrap();
                                return;
                            }
                        },

                        Err(_) => {
                            alert(
                                500,
                                500,
                                match lang {
                                    Lang::English => "'Amount of Pages' input error",
                                    Lang::Russian => "Ошибка ввода количества страниц",
                                },
                            );
                            caretaker.pop().unwrap();
                            return;
                        }
                    }
                }
            }
            break;
        } else if !am.shown() {
            break;
        }
    }
}
