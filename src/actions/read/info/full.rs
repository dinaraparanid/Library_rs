extern crate fltk;

use fltk::{app, app::App, input::Input, prelude::*, table::Table};

use crate::{
    actions::read::{info::simple::*, utils::check_reader},
    books::{book_sys::BookSystem, genres::Genres},
    change::{input3::Input3, Inputable},
    reading::read_base::ReaderBase,
    restore::caretaker::Caretaker,
    Lang,
};

use std::{cell::RefCell, rc::Rc};

/// Function that gives info about reader.
/// Shows only those books that
/// reader is reading now.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn reader_info_reading(
    reader_base: Rc<RefCell<ReaderBase>>,
    book_system: Rc<RefCell<BookSystem>>,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
    main_table: &mut Table,
) {
    let (s2, r2) = app::channel();
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
            Lang::English => "Second Names",
            Lang::Russian => "Фамилия",
        },
        match lang {
            Lang::English => "Middle Name",
            Lang::Russian => "Отчество",
        },
    );

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            if message {
                inp.hide();

                if let Ok(reader) = inp.set_input() {
                    if let Some(ind) = {
                        let check = check_reader(&*(*reader_base).borrow(), &reader, app, lang);
                        check
                    } {
                        reader_info_simple_reading(
                            ind,
                            reader_base.clone(),
                            book_system.clone(),
                            genres,
                            caretaker,
                            app,
                            lang,
                            main_table,
                        )
                    }
                }
            }
            break;
        } else if !inp.shown() {
            return;
        }
    }
}

/// Function that gives info about reader.
/// Shows all read books.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn reader_info_all_books(
    reader_base: Rc<RefCell<ReaderBase>>,
    book_system: Rc<RefCell<BookSystem>>,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
    main_table: &mut Table,
) {
    let (s2, r2) = app::channel();
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
            Lang::English => "Second Names",
            Lang::Russian => "Фамилия",
        },
        match lang {
            Lang::English => "Middle Name",
            Lang::Russian => "Отчество",
        },
    );

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            if message {
                inp.hide();

                if let Ok(reader) = inp.set_input() {
                    if let Some(ind) = {
                        let check = check_reader(&*(*reader_base).borrow(), &reader, app, lang);
                        check
                    } {
                        reader_info_simple_all_books(
                            ind,
                            reader_base.clone(),
                            book_system.clone(),
                            genres,
                            caretaker,
                            app,
                            lang,
                            main_table,
                        )
                    }
                }
            }
            break;
        } else if !inp.shown() {
            return;
        }
    }
}
