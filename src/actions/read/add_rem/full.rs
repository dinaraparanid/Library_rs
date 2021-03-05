extern crate fltk;
extern crate fltk_calendar;

use fltk::{app, app::App, dialog::alert, input::Input, prelude::*};
use fltk_calendar::calendar::Calendar;

use crate::{
    actions::read::{add_rem::simple::*, utils::*},
    books::{book_sys::BookSystem, date::Date, genres::Genres},
    change::{input3::Input3, Inputable},
    reading::read_base::ReaderBase,
    restore::caretaker::Caretaker,
    Lang,
};

/// Function that adds reader.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn add_reader(
    reader_base: &mut ReaderBase,
    book_system: &mut BookSystem,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) {
    let (s2, r2) = app::channel();

    let mut inp = Input3::<Input, Input, Input>::new(
        match lang {
            Lang::English => "Add Reader",
            Lang::Russian => "Добавить читателя",
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

    caretaker.add_memento(reader_base, book_system, genres);

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            if message {
                inp.hide();

                if let Ok(reader) = inp.set_input() {
                    if empty_inp_reader(&reader, lang) {
                        caretaker.pop().unwrap();
                        return;
                    }

                    let mut win =
                        fltk::window::SingleWindow::new(800, 500, 200, 100, "Choose birth date");

                    let _ = fltk::frame::Frame::new(
                        30,
                        10,
                        150,
                        50,
                        match lang {
                            Lang::English => "Choose birth date",
                            Lang::Russian => "Выберите дату рождения",
                        },
                    );

                    let mut but = fltk::button::Button::new(
                        80,
                        60,
                        60,
                        20,
                        match lang {
                            Lang::English => "OK",
                            Lang::Russian => "ОК",
                        },
                    );

                    win.end();
                    win.show();

                    let (sd, rd) = app::channel();
                    but.emit(sd, true);

                    while app.wait() {
                        if let Some(msg) = rd.recv() {
                            if msg {
                                win.hide();

                                match Calendar::default().get_date() {
                                    Some(date) => {
                                        match reader_base.add_reader(
                                            unsafe { reader.get_unchecked(0).clone() },
                                            unsafe { reader.get_unchecked(1).clone() },
                                            unsafe { reader.get_unchecked(2).clone() },
                                            Date::from(date),
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
                                                reader_base.save();
                                            }

                                            Err(_) => {
                                                alert(
                                                    500,
                                                    500,
                                                    match lang {
                                                        Lang::English => "Reader with same parameters already exists",
                                                        Lang::Russian => "Читатель с предложенными парамтрами уже существует",
                                                    }
                                                );
                                                caretaker.pop().unwrap();
                                                return;
                                            }
                                        }
                                    }

                                    None => {
                                        alert(
                                            500,
                                            500,
                                            match lang {
                                                Lang::English => "Date wasn't selected",
                                                Lang::Russian => "Дата не была выбрана",
                                            },
                                        );
                                        caretaker.pop().unwrap();
                                        return;
                                    }
                                }
                            }
                            break;
                        } else if !win.shown() {
                            caretaker.pop().unwrap();
                            return;
                        }
                    }
                }
            }
            break;
        } else if !inp.shown() {
            caretaker.pop().unwrap();
            return;
        }
    }
}

/// Function that removes reader.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn remove_reader(
    reader_base: &mut ReaderBase,
    book_system: &mut BookSystem,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) {
    let (s2, r2) = app::channel();

    let mut inp = Input3::<Input, Input, Input>::new(
        match lang {
            Lang::English => "Remove Reader",
            Lang::Russian => "Удалить читателя",
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
                    if let Some(rind) = check_reader(reader_base, &reader, app, lang) {
                        remove_reader_simple(
                            rind,
                            reader_base,
                            book_system,
                            genres,
                            caretaker,
                            lang,
                        )
                    }
                }
            }
            break;
        } else if !inp.shown() {
            break;
        }
    }
}
