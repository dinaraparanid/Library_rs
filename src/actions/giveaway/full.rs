extern crate fltk;

use crate::{
    actions::{book::*, giveaway::simple::*, read::utils::check_reader},
    books::{book::Book, book_sys::BookSystem, date::Date, genres::Genres},
    change::{input3::Input3, Inputable},
    reading::read_base::ReaderBase,
    restore::caretaker::Caretaker,
    Lang,
};

use fltk::{
    app,
    app::{channel, App},
    dialog::alert,
    input::*,
    prelude::*,
};

/// Changes return date

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
            match mes {
                true => {
                    inp2.hide();

                    if let Ok(reader) = inp2.set_input() {
                        match check_reader(reader_base, &reader, app, lang) {
                            Some(rind) => unsafe {
                                match &(**reader_base.readers.get_unchecked(rind)).borrow().reading
                                {
                                    Some(book) => {
                                        change_return_date_simple(
                                            &Some(book.clone()),
                                            book_system,
                                            reader_base,
                                            genres,
                                            caretaker,
                                            app,
                                            lang,
                                        );
                                    }

                                    None => {
                                        alert(
                                            500,
                                            500,
                                            match lang {
                                                Lang::English => {
                                                    "This reader isn't reading anything"
                                                }
                                                Lang::Russian => "Этот читатель ничего не читает",
                                            },
                                        );

                                        caretaker.pop();
                                        return;
                                    }
                                }
                            },

                            None => {
                                caretaker.pop();
                                return;
                            }
                        }
                    }
                }

                false => (),
            }
        } else if !inp2.shown() {
            caretaker.pop();
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
            match message {
                true => {
                    inp.hide();

                    if let Ok(reader) = inp.set_input() {
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
                                caretaker.pop();
                                return;
                            }
                        }
                    }
                }
                false => (),
            }
            break;
        } else if !inp.shown() {
            caretaker.pop();
            break;
        }
    }
}

/// Function that gives book to reader.
/// It requires you to input
/// info about reader, book and return date.
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
            match message {
                true => {
                    inp.hide();

                    if let Ok(reader) = inp.set_input() {
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
                                caretaker.pop();
                                return;
                            }
                        }
                    }
                }
                false => (),
            }
            break;
        } else if !inp.shown() {
            caretaker.pop();
            break;
        }
    }
}
