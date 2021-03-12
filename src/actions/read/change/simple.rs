extern crate fltk;
extern crate fltk_calendar;

use crate::{
    books::{book_sys::BookSystem, date::Date, genres::Genres},
    change::{input1::Input1, Inputable},
    reading::read_base::ReaderBase,
    restore::caretaker::Caretaker,
    Lang,
};

use fltk::{app, app::App, dialog::alert, input::Input, prelude::*};
use fltk_calendar::calendar::Calendar;

/// Function that changes
/// name of already known reader

#[inline]
pub(crate) fn change_name_simple(
    ind: usize,
    reader_base: &mut ReaderBase,
    book_system: &mut BookSystem,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) -> Option<String> {
    let (s3, r3) = app::channel();
    let mut get_name = Input1::<Input>::new(
        match lang {
            Lang::English => "New Name",
            Lang::Russian => "Новое Имя",
        },
        match lang {
            Lang::English => "New Name",
            Lang::Russian => "Новое Имя",
        },
    );

    caretaker.add_memento(reader_base, book_system, genres);

    get_name.show();
    (*get_name.ok).borrow_mut().emit(s3, true);

    while app.wait() {
        if let Some(mes) = r3.recv() {
            if mes {
                get_name.hide();

                if let Ok(new_name) = get_name.set_input(lang) {
                    return match reader_base
                        .change_name(ind, unsafe { new_name.get_unchecked(0).clone() })
                    {
                        Ok(_) => {
                            fltk::dialog::message(
                                500,
                                500,
                                match lang {
                                    Lang::English => "Successfully changed",
                                    Lang::Russian => "Успешно изменено",
                                },
                            );

                            reader_base.save();
                            book_system.save();
                            Some(unsafe { new_name.get_unchecked(0).clone() })
                        }

                        Err(0) => {
                            alert(
                                500,
                                500,
                                match lang {
                                    Lang::English => "Reader is not found",
                                    Lang::Russian => "Читатель не найден",
                                },
                            );
                            caretaker.pop().unwrap();
                            None
                        }

                        Err(1) => {
                            alert(
                                500,
                                500,
                                match lang {
                                    Lang::English => "Reader with same parameters already exists",
                                    Lang::Russian => {
                                        "Читатель с предложенными параметрами уже существует"
                                    }
                                },
                            );
                            caretaker.pop().unwrap();
                            None
                        }

                        Err(_) => {
                            alert(
                                500,
                                500,
                                match lang {
                                    Lang::English => "'New name' is empty",
                                    Lang::Russian => "'Новое имя' пусто",
                                },
                            );
                            caretaker.pop().unwrap();
                            None
                        }
                    };
                }
            }
            break;
        } else if !get_name.shown() {
            caretaker.pop().unwrap();
            return None;
        }
    }

    None
}

/// Function that changes
/// 2-nd name of already known reader

#[inline]
pub(crate) fn change_family_simple(
    ind: usize,
    reader_base: &mut ReaderBase,
    book_system: &mut BookSystem,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) -> Option<String> {
    let (s3, r3) = app::channel();
    let mut get_family = Input1::<Input>::new(
        match lang {
            Lang::English => "'New 2-nd Name",
            Lang::Russian => "Новая Фамилия",
        },
        match lang {
            Lang::English => "'New 2-nd Name",
            Lang::Russian => "Новая Фамилия",
        },
    );

    caretaker.add_memento(reader_base, book_system, genres);

    get_family.show();
    (*get_family.ok).borrow_mut().emit(s3, true);

    while app.wait() {
        if let Some(mes) = r3.recv() {
            if mes {
                get_family.hide();

                if let Ok(new_family) = get_family.set_input(lang) {
                    return match reader_base
                        .change_family(ind, unsafe { new_family.get_unchecked(0).clone() })
                    {
                        Ok(_) => {
                            fltk::dialog::message(
                                500,
                                500,
                                match lang {
                                    Lang::English => "Successfully changed",
                                    Lang::Russian => "Успешно изменено",
                                },
                            );

                            reader_base.save();
                            book_system.save();

                            Some(unsafe { new_family.get_unchecked(0).clone() })
                        }

                        Err(0) => {
                            alert(
                                500,
                                500,
                                match lang {
                                    Lang::English => "Reader is not found",
                                    Lang::Russian => "Читатель не найден",
                                },
                            );
                            caretaker.pop().unwrap();
                            None
                        }

                        Err(1) => {
                            alert(
                                500,
                                500,
                                match lang {
                                    Lang::English => "Reader with same parameters already exists",
                                    Lang::Russian => {
                                        "Читатель с предложенными параметрами уже существует"
                                    }
                                },
                            );
                            caretaker.pop().unwrap();
                            None
                        }

                        Err(_) => {
                            alert(
                                500,
                                500,
                                match lang {
                                    Lang::English => "'New 2-nd Name' is empty",
                                    Lang::Russian => "'Новая Фамилия' пусто",
                                },
                            );
                            caretaker.pop().unwrap();
                            None
                        }
                    };
                }
            }
            break;
        } else if !get_family.shown() {
            caretaker.pop().unwrap();
            return None;
        }
    }

    None
}

/// Change middle name of already known reader

#[inline]
pub(crate) fn change_father_simple(
    ind: usize,
    reader_base: &mut ReaderBase,
    book_system: &mut BookSystem,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) -> Option<String> {
    let (s3, r3) = app::channel();
    let mut get_father = Input1::<Input>::new(
        match lang {
            Lang::English => "New Middle Name",
            Lang::Russian => "Новое Отчество",
        },
        match lang {
            Lang::English => "New Middle Name",
            Lang::Russian => "Новое Отчество",
        },
    );

    caretaker.add_memento(reader_base, book_system, genres);

    get_father.show();
    (*get_father.ok).borrow_mut().emit(s3, true);

    while app.wait() {
        if let Some(mes) = r3.recv() {
            if mes {
                get_father.hide();

                if let Ok(new_father) = get_father.set_input(lang) {
                    return match reader_base
                        .change_father(ind, unsafe { new_father.get_unchecked(0).clone() })
                    {
                        Ok(_) => {
                            fltk::dialog::message(
                                500,
                                500,
                                match lang {
                                    Lang::English => "Successfully changed",
                                    Lang::Russian => "Успешно изменено",
                                },
                            );

                            reader_base.save();
                            book_system.save();
                            Some(unsafe { new_father.get_unchecked(0).clone() })
                        }

                        Err(0) => {
                            alert(
                                500,
                                500,
                                match lang {
                                    Lang::English => "Reader is not found",
                                    Lang::Russian => "Читатель не найден",
                                },
                            );
                            caretaker.pop().unwrap();
                            None
                        }

                        Err(1) => {
                            alert(
                                500,
                                500,
                                match lang {
                                    Lang::English => "Reader with same parameters already exists",
                                    Lang::Russian => {
                                        "Читатель с предложенными параметрами уже существует"
                                    }
                                },
                            );
                            caretaker.pop().unwrap();
                            None
                        }

                        Err(_) => {
                            alert(
                                500,
                                500,
                                match lang {
                                    Lang::English => "'New Mid. Name' is empty",
                                    Lang::Russian => "'Новое Отчество' пусто",
                                },
                            );
                            caretaker.pop().unwrap();
                            None
                        }
                    };
                }
            }
            break;
        } else if !get_father.shown() {
            caretaker.pop().unwrap();
            return None;
        }
    }

    None
}

/// Change middle name of already known reader

#[inline]
pub(crate) fn change_info_simple(
    ind: usize,
    reader_base: &mut ReaderBase,
    book_system: &mut BookSystem,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) -> Option<String> {
    let (s3, r3) = app::channel();
    let mut get_info = Input1::<Input>::new(
        match lang {
            Lang::English => "New Info (< 50 symb)",
            Lang::Russian => "Инфо (< 50 симв.)",
        },
        match lang {
            Lang::English => "New Info (< 50 symb)",
            Lang::Russian => "Инфо (< 50 симв.)",
        },
    );

    caretaker.add_memento(reader_base, book_system, genres);

    get_info.show();
    (*get_info.ok).borrow_mut().emit(s3, true);

    while app.wait() {
        if let Some(mes) = r3.recv() {
            if mes {
                get_info.hide();

                if let Ok(new_info) = get_info.set_input(lang) {
                    return match reader_base
                        .change_info(ind, unsafe { new_info.get_unchecked(0).clone() })
                    {
                        Ok(_) => {
                            fltk::dialog::message(
                                500,
                                500,
                                match lang {
                                    Lang::English => "Successfully changed",
                                    Lang::Russian => "Успешно изменено",
                                },
                            );

                            reader_base.save();
                            book_system.save();
                            Some(unsafe { new_info.get_unchecked(0).clone() })
                        }

                        Err(0) => {
                            alert(
                                500,
                                500,
                                match lang {
                                    Lang::English => "Reader is not found",
                                    Lang::Russian => "Читатель не найден",
                                },
                            );
                            caretaker.pop().unwrap();
                            None
                        }

                        Err(_) => {
                            alert(
                                500,
                                500,
                                match lang {
                                    Lang::English => "'New Info' is empty",
                                    Lang::Russian => "'Инфо' пусто",
                                },
                            );
                            caretaker.pop().unwrap();
                            None
                        }
                    };
                }
            }
            break;
        } else if !get_info.shown() {
            caretaker.pop().unwrap();
            return None;
        }
    }

    None
}

/// Function that changes
/// age of already known reader

#[inline]
pub(crate) fn change_age_simple(
    ind: usize,
    reader_base: &mut ReaderBase,
    book_system: &mut BookSystem,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) -> Option<u16> {
    caretaker.add_memento(reader_base, book_system, genres);

    let mut win = fltk::window::SingleWindow::new(800, 500, 250, 100, "Choose new birth date");

    let _ = fltk::frame::Frame::new(
        50,
        10,
        150,
        50,
        match lang {
            Lang::English => "Choose new birth date",
            Lang::Russian => "Выберите новую дату рождения",
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

                return match Calendar::default().get_date() {
                    Some(date) => match reader_base.change_age(ind, Date::from(date)) {
                        Ok(_) => {
                            fltk::dialog::message(
                                500,
                                500,
                                match lang {
                                    Lang::English => "Successfully changed",
                                    Lang::Russian => "Успешно изменено",
                                },
                            );

                            reader_base.save();
                            book_system.save();

                            unsafe {
                                Some((**reader_base.readers.get_unchecked(ind)).borrow().age())
                            }
                        }

                        Err(0) => {
                            alert(
                                500,
                                500,
                                match lang {
                                    Lang::English => "Reader isn't found",
                                    Lang::Russian => "Читатель не найден",
                                },
                            );
                            caretaker.pop().unwrap();
                            None
                        }

                        Err(_) => {
                            alert(
                                500,
                                500,
                                match lang {
                                    Lang::English => "Reader with same parameters already exists",
                                    Lang::Russian => {
                                        "Читатель с предложенными параметрами уже существует"
                                    }
                                },
                            );
                            caretaker.pop().unwrap();
                            None
                        }
                    },

                    None => {
                        alert(
                            500,
                            500,
                            match lang {
                                Lang::English => "Date wasn't selected",
                                Lang::Russian => "Дата не была выбрана",
                            },
                        );
                        None
                    }
                };
            }
            break;
        } else if !win.shown() {
            return None;
        }
    }

    None
}
