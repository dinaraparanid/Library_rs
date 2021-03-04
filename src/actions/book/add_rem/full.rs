extern crate fltk;

use crate::{
    actions::book::{add_rem::simple::*, utils::*},
    books::{book_sys::BookSystem, genres::Genres},
    change::{input1::Input1, input3::Input3, Inputable},
    reading::read_base::ReaderBase,
    restore::caretaker::Caretaker,
    Lang,
};

use fltk::{
    app,
    app::App,
    dialog::alert,
    input::{Input, IntInput},
    prelude::*,
};

/// Function that add simple books
/// and the book (if it wasn't in library)
/// If number of books to add plus
/// number of existing books
/// is **more than usize::MAX**,
/// than you will receive error.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn add_books(
    book_system: &mut BookSystem,
    reader_base: &ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) {
    let (s2, r2) = app::channel();
    let mut inp = Input3::<Input, Input, IntInput>::new(
        match lang {
            Lang::English => "Add Books",
            Lang::Russian => "Добавить книги",
        },
        match lang {
            Lang::English => "Title",
            Lang::Russian => "Название",
        },
        match lang {
            Lang::English => "Author",
            Lang::Russian => "Автор",
        },
        match lang {
            Lang::English => "Amount of Pages",
            Lang::Russian => "Количество страниц",
        },
    );

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    inp.hide();

                    if let Ok(books) = inp.set_input() {
                        if !empty_inp_book(&books, lang) {
                            match unsafe { books.get_unchecked(2).trim().parse::<u16>() } {
                                Ok(x) => match book_system.find_book(
                                    unsafe { books.get_unchecked(0) },
                                    unsafe { books.get_unchecked(1) },
                                    x,
                                ) {
                                    Some(i) => {
                                        add_books_simple(
                                            i,
                                            book_system,
                                            reader_base,
                                            genres,
                                            caretaker,
                                            app,
                                            lang,
                                        );
                                    }

                                    None => add_book_simple(
                                        &books,
                                        book_system,
                                        reader_base,
                                        genres,
                                        caretaker,
                                        app,
                                        lang,
                                    ),
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
                                }
                            }
                        }
                    }
                }
                false => (),
            }
            break;
        } else if !inp.shown() {
            break;
        }
    }
}

/// Function that removes one simple book.
/// It takes index of book, so be careful.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn remove_book(
    book_system: &mut BookSystem,
    reader_base: &mut ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) {
    let (s2, r2) = app::channel();
    let mut inp = Input3::<Input, Input, IntInput>::new(
        match lang {
            Lang::English => "Remove Book",
            Lang::Russian => "Удалить книгу",
        },
        match lang {
            Lang::English => "Title",
            Lang::Russian => "Название",
        },
        match lang {
            Lang::English => "Author",
            Lang::Russian => "Автор",
        },
        match lang {
            Lang::English => "Amount of Pages",
            Lang::Russian => "Количество страниц",
        },
    );

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    inp.hide();

                    if let Ok(book) = inp.set_input() {
                        if let Ok(index) = check_book(book_system, &book, lang) {
                            remove_book_simple(
                                index,
                                book_system,
                                reader_base,
                                genres,
                                caretaker,
                                app,
                                lang,
                            );
                        }
                    }
                }
                false => (),
            }
            break;
        } else if !inp.shown() {
            break;
        }
    }
}

/// **DEPRECATED**
///
/// Used before to add new TheBook.
/// Complains if TheBook with same params exists.
/// **Use add_books() instead**
///
/// Function that add new book and with some simple books.
/// If you have mistakes in input,
/// program will let you know

#[deprecated(
    note = "Used before to add new TheBook. Complains if TheBook with same params exists. Use add_books() instead"
)]
fn add_book(
    book_system: &mut BookSystem,
    reader_base: &ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) {
    let (s2, r2) = app::channel();
    let mut inp = Input3::<Input, Input, IntInput>::new(
        match lang {
            Lang::English => "Add New Book",
            Lang::Russian => "Добавить новую книгу",
        },
        match lang {
            Lang::English => "Title",
            Lang::Russian => "Название",
        },
        match lang {
            Lang::English => "Author",
            Lang::Russian => "Автор",
        },
        match lang {
            Lang::English => "Amount of Pages",
            Lang::Russian => "Количество страниц",
        },
    );

    caretaker.add_memento(reader_base, book_system, genres);

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            if message {
                inp.hide();

                if let Ok(the_book) = inp.set_input() {
                    if !empty_inp_book(&the_book, lang) {
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

                        am.show();
                        (*am.ok).borrow_mut().emit(s, true);

                        while app.wait() {
                            if let Some(mes) = r.recv() {
                                if mes {
                                    am.hide();

                                    if let Ok(amount) = am.set_input() {
                                        match amount.first().unwrap().trim().parse::<usize>() {
                                            Ok(amount) => {
                                                match the_book.last().unwrap().trim().parse::<u16>()
                                                {
                                                    Ok(x) => unsafe {
                                                        match book_system.add_book(
                                                            the_book.get_unchecked(0).clone(),
                                                            the_book.get_unchecked(1).clone(),
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
                                                                        Lang::English => {
                                                                            "Successfully added"
                                                                        }
                                                                        Lang::Russian => {
                                                                            "Успешно добавлено"
                                                                        }
                                                                    },
                                                                );
                                                                book_system.save();
                                                                return;
                                                            }

                                                            Err(_) => {
                                                                alert(
                                                                    500,
                                                                    500,
                                                                    match lang {
                                                                        Lang::English => "Book with same params already exists",
                                                                        Lang::Russian => concat!("Книга с предложенными",
                                                                        " параметрами уже существует"),
                                                                    }
                                                                );
                                                                caretaker.pop().unwrap();
                                                                return;
                                                            }
                                                        }
                                                    },

                                                    Err(_) => {
                                                        alert(
                                                            500,
                                                            500,
                                                            match lang {
                                                                Lang::English => "Incorrect 'Amount of Pages' input",
                                                                Lang::Russian => "Некорретный ввод количества страниц",
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
                                                        Lang::English => {
                                                            "'Amount of Pages' input error"
                                                        }
                                                        Lang::Russian => {
                                                            "Ошибка ввода количества страниц"
                                                        }
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
                }
            }
            break;
        } else if !inp.shown() {
            break;
        }
    }
}

/// Function that removes
/// all simple books and TheBook.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn remove_the_book(
    book_system: &mut BookSystem,
    reader_base: &mut ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) {
    let (s2, r2) = app::channel();
    let mut inp = Input3::<Input, Input, IntInput>::new(
        match lang {
            Lang::English => "Remove Books",
            Lang::Russian => "Удалить книги",
        },
        match lang {
            Lang::English => "Title",
            Lang::Russian => "Название",
        },
        match lang {
            Lang::English => "Author",
            Lang::Russian => "Автор",
        },
        match lang {
            Lang::English => "Amount of Pages",
            Lang::Russian => "Количество страниц",
        },
    );

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            if message {
                inp.hide();

                if let Ok(the_book) = inp.set_input() {
                    if let Ok(index) = check_book(book_system, &the_book, lang) {
                        remove_the_book_simple(
                            index,
                            book_system,
                            reader_base,
                            genres,
                            caretaker,
                            lang,
                        );
                    }
                }
            }
            break;
        } else if !inp.shown() {
            break;
        }
    }
}
