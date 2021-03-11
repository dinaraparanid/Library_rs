extern crate fltk;

use crate::{
    actions::{giveaway::simple::*, read::utils::check_reader},
    books::{book_sys::BookSystem, genres::Genres},
    change::{input3::Input3, Inputable},
    reading::read_base::ReaderBase,
    restore::caretaker::Caretaker,
    Lang,
};

use fltk::{app::App, input::*};

/// Function that changes
/// return date for the book

#[inline]
pub fn change_return_date(
    book_system: &mut BookSystem,
    reader_base: &ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) {
    let (s, r) = fltk::app::channel();
    let mut inp2 = Input3::<Input, Input, Input>::new(
        match lang {
            Lang::English => "Find Reader",
            Lang::Russian => "Поиск Читателя",
        },
        match lang {
            Lang::English => "First Name",
            Lang::Russian => "Имя",
        },
        match lang {
            Lang::English => "Second Name",
            Lang::Russian => "Фамилия",
        },
        match lang {
            Lang::English => "Middle Name",
            Lang::Russian => "Отчество",
        },
    );

    inp2.show();
    (*inp2.ok).borrow_mut().emit(s, true);

    while app.wait() {
        if let Some(mes) = r.recv() {
            if mes {
                inp2.hide();

                if let Ok(reader) = inp2.set_input(lang) {
                    match check_reader(reader_base, &reader, app, lang) {
                        Some(rind) => {
                            change_return_date_simple(
                                rind,
                                book_system,
                                reader_base,
                                genres,
                                caretaker,
                                app,
                                lang,
                            );
                        }

                        None => {
                            caretaker.pop().unwrap();
                            return;
                        }
                    }
                }
            }
        } else if !inp2.shown() {
            caretaker.pop().unwrap();
            return;
        }
    }
}

/// Function that gives book to reader.
/// It requires you to input
/// info about reader, book and return date.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn give_book(
    reader_base: &mut ReaderBase,
    book_system: &mut BookSystem,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) {
    let (s2, r2) = fltk::app::channel();
    let mut inp = Input3::<Input, Input, Input>::new(
        match lang {
            Lang::English => "Find Reader",
            Lang::Russian => "Поиск Читателя",
        },
        match lang {
            Lang::English => "First Name",
            Lang::Russian => "Имя",
        },
        match lang {
            Lang::English => "Second Name",
            Lang::Russian => "Фамилия",
        },
        match lang {
            Lang::English => "Middle Name",
            Lang::Russian => "Отчество",
        },
    );

    caretaker.add_memento(reader_base, book_system, genres);

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            if message {
                inp.hide();

                if let Ok(reader) = inp.set_input(lang) {
                    match check_reader(reader_base, &reader, app, lang) {
                        Some(x) => {
                            give_book_known_reader(
                                x,
                                reader_base,
                                book_system,
                                genres,
                                caretaker,
                                app,
                                lang,
                            );
                        }

                        None => {
                            caretaker.pop().unwrap();
                            return;
                        }
                    }
                }
            }
            break;
        } else if !inp.shown() {
            caretaker.pop().unwrap();
            break;
        }
    }
}

/// Function that gets book from reader.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn get_book(
    reader_base: &mut ReaderBase,
    book_system: &mut BookSystem,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) {
    let (s2, r2) = fltk::app::channel();
    let mut inp = Input3::<Input, Input, Input>::new(
        match lang {
            Lang::English => "Find Reader",
            Lang::Russian => "Поиск Читателя",
        },
        match lang {
            Lang::English => "First Name",
            Lang::Russian => "Имя",
        },
        match lang {
            Lang::English => "Second Name",
            Lang::Russian => "Фамилия",
        },
        match lang {
            Lang::English => "Middle Name",
            Lang::Russian => "Отчество",
        },
    );

    caretaker.add_memento(reader_base, book_system, genres);

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            if message {
                inp.hide();

                if let Ok(reader) = inp.set_input(lang) {
                    match check_reader(reader_base, &reader, app, lang) {
                        Some(x) => {
                            get_book_known_reader(
                                x,
                                reader_base,
                                book_system,
                                genres,
                                caretaker,
                                app,
                                lang,
                            );
                        }

                        None => {
                            caretaker.pop().unwrap();
                            return;
                        }
                    }
                }
            }
            break;
        } else if !inp.shown() {
            caretaker.pop().unwrap();
            break;
        }
    }
}
