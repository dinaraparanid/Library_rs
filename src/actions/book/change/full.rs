extern crate fltk;

use crate::{
    actions::book::{change::simple::*, utils::check_book},
    books::{book_sys::BookSystem, genres::Genres},
    change::{input1::Input1, input2::Input2, input3::Input3, Inputable},
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

/// Function that changes title
/// of all simple books and TheBook.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn change_title(
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
            Lang::English => "Change title",
            Lang::Russian => "Изменить название",
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
                            change_title_simple(
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

/// Function that changes author
/// of all simple books and TheBook.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn change_author(
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
            Lang::English => "Change author",
            Lang::Russian => "Изменить автора",
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
                            change_author_simple(
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

/// Function that changes pages
/// of all simple books and TheBook.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn change_pages(
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
            Lang::English => "Change amount of pages",
            Lang::Russian => "Изменить количество страниц",
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
                            change_pages_simple(
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

/// Function that changes location
/// of simple book.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn change_location(
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
            Lang::English => "Change book's location",
            Lang::Russian => "Переместить книгу",
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
                        if let Ok(t_ind) = check_book(book_system, &book, lang) {
                            let (s, r) = app::channel();
                            let mut inp2 = Input1::<IntInput>::new(
                                match lang {
                                    Lang::English => "Book's number",
                                    Lang::Russian => "Номер книги",
                                },
                                match lang {
                                    Lang::English => "Book's number",
                                    Lang::Russian => "Номер книги",
                                },
                            );

                            inp2.show();
                            (*inp2.ok).borrow_mut().emit(s, true);

                            while app.wait() {
                                if let Some(mes) = r.recv() {
                                    match mes {
                                        true => {
                                            inp2.hide();

                                            if let Ok(ind) = inp2.set_input() {
                                                match ind.first().unwrap().trim().parse::<usize>() {
                                                    Err(_) => alert(
                                                        500,
                                                        500,
                                                        match lang {
                                                            Lang::English => {
                                                                "Incorrect book's number"
                                                            }
                                                            Lang::Russian => {
                                                                "Некорректный номер книги"
                                                            }
                                                        },
                                                    ),

                                                    Ok(s_ind) => change_location_simple(
                                                        t_ind,
                                                        s_ind,
                                                        book_system,
                                                        reader_base,
                                                        genres,
                                                        caretaker,
                                                        app,
                                                        lang,
                                                    ),
                                                }
                                            }
                                        }
                                        false => (),
                                    }
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
