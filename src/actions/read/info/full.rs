extern crate fltk;

use fltk::{
    app,
    app::App,
    dialog::alert,
    draw,
    frame::Frame,
    group::VGrid,
    input::{Input, IntInput},
    menu::{MenuBar, MenuFlag},
    prelude::*,
    table::Table,
    window::SingleWindow,
};

use crate::{
    actions::{
        book::info::simple::book_info_simple,
        giveaway::simple::{get_book_known_reader, give_book_known_reader},
        read::{change::*, info::simple::reader_info_simple, utils::check_reader},
        tables::*,
    },
    books::{book_sys::BookSystem, date::Date, genres::Genres},
    change::{input3::Input3, Inputable},
    reading::read_base::ReaderBase,
    restore::caretaker::Caretaker,
    Lang,
};

use std::{cell::RefCell, cmp::max, rc::Rc};

/// Function that gives info about reader.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn reader_info(
    reader_base: Rc<RefCell<ReaderBase>>,
    book_system: Rc<RefCell<BookSystem>>,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
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
            match message {
                true => {
                    inp.hide();

                    if let Ok(reader) = inp.set_input() {
                        let check = check_reader(&*(*reader_base).borrow(), &reader, app, lang);

                        match check {
                            Some(ind) => reader_info_simple(
                                ind,
                                &mut *(*reader_base).borrow_mut(),
                                &mut *(*book_system).borrow_mut(),
                                genres,
                                caretaker,
                                app,
                                lang,
                            ),

                            None => return,
                        }
                    }
                }
                false => (),
            }
            break;
        } else if !inp.shown() {
            return;
        }
    }
}
