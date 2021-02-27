extern crate fltk;

use fltk::{
    app,
    app::App,
    dialog::alert,
    input::{Input, IntInput},
    prelude::*,
};

use crate::{
    actions::read::utils::*,
    books::{book_sys::BookSystem, date::Date, genres::Genres},
    change::{input4::Input4, Inputable},
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

    let mut inp = Input4::<Input, Input, Input, Input>::new(
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
        match lang {
            Lang::English => "Birth Date (D/M/Y)",
            Lang::Russian => "Дата Рождения (Д/М/Г)",
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
                        if empty_inp_reader(&reader, lang) {
                            caretaker.pop();
                            return;
                        }

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
                                            Ok(date) => match reader_base.add_reader(
                                                reader.get_unchecked(0).clone(),
                                                reader.get_unchecked(1).clone(),
                                                reader.get_unchecked(2).clone(),
                                                date,
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
                                                    caretaker.pop();
                                                    return;
                                                }
                                            },

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
            caretaker.pop();
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

    let mut inp = Input4::<Input, Input, Input, Input>::new(
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
                    let rem_reader_params = inp.set_input();
                    inp.hide();

                    if let Ok(reader) = rem_reader_params {
                        let rind;

                        match check_reader(reader_base, &reader, lang) {
                            Ok(x) => rind = x,
                            Err(_) => return,
                        }

                        remove_reader_simple(
                            rind,
                            reader_base,
                            book_system,
                            genres,
                            caretaker,
                            lang,
                        );
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
