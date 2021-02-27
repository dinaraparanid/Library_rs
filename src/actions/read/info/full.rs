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
        giveaway::{get_book_known_reader, give_book_known_reader},
        read::{change::*, info::simple::reader_info_simple},
        tables::*,
    },
    books::{book_sys::BookSystem, date::Date, genres::Genres},
    change::{input4::Input4, Inputable},
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
    let mut inp = Input4::<Input, Input, Input, Input>::new(
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
        match lang {
            Lang::English => "Birth Date (D/M/Y)",
            Lang::Russian => "Дата Рождения (Д/М/Г)",
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
                        let it = reader.last().unwrap().trim().split('/').collect::<Vec<_>>();

                        if it.len() != 3 {
                            alert(
                                500,
                                500,
                                match lang {
                                    Lang::English => "Incorrect 'Birth Date' input",
                                    Lang::Russian => "Некорректный ввод 'Даты Рождения'",
                                },
                            );
                            caretaker.pop();
                            return;
                        }

                        let mut it = it.into_iter();

                        unsafe {
                            match it.next().unwrap().trim().parse::<u8>() {
                                Ok(day) => match it.next().unwrap().trim().parse::<u8>() {
                                    Ok(month) => match it.next().unwrap().trim().parse::<u16>() {
                                        Ok(year) => match Date::new(day, month, year) {
                                            Ok(date) => {
                                                let find = (*reader_base).borrow().find_reader(
                                                    reader.get_unchecked(0),
                                                    reader.get_unchecked(1),
                                                    reader.get_unchecked(2),
                                                    date,
                                                );

                                                match find {
                                                    None => alert(
                                                        500,
                                                        500,
                                                        match lang {
                                                            Lang::English => "Reader isn't found",
                                                            Lang::Russian => "Читатель не найден",
                                                        },
                                                    ),

                                                    Some(ind) => reader_info_simple(
                                                        ind,
                                                        &mut *(*reader_base).borrow_mut(),
                                                        &mut *(*book_system).borrow_mut(),
                                                        genres,
                                                        caretaker,
                                                        app,
                                                        lang,
                                                    ),
                                                }
                                            }

                                            Err(_) => {
                                                alert(
                                                    500,
                                                    500,
                                                    match lang {
                                                        Lang::English => "Incorrect date",
                                                        Lang::Russian => "Некорректная дата",
                                                    },
                                                );
                                                caretaker.pop();
                                                return;
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
                                            caretaker.pop();
                                            return;
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
                                        caretaker.pop();
                                        return;
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
                                    caretaker.pop();
                                    return;
                                }
                            }
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
