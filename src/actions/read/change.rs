extern crate fltk;

use crate::{
    actions::read::utils::check_reader,
    books::{book_sys::BookSystem, genres::Genres},
    change::{input1::Input1, input3::Input3, input4::Input4, Inputable},
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

/// Change name of already known reader

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
            match mes {
                true => {
                    get_name.hide();

                    if let Ok(new_name) = get_name.set_input() {
                        unsafe {
                            return match reader_base
                                .change_name(ind, new_name.get_unchecked(0).clone())
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
                                    Some(new_name.get_unchecked(0).clone())
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
                                    caretaker.pop();
                                    None
                                }

                                Err(1) => {
                                    alert(500, 500, match lang {
										Lang::English => "Reader with same parameters already exists",
										Lang::Russian => "Читатель с предложенными параметрами уже существует",
									});
                                    caretaker.pop();
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
                                    caretaker.pop();
                                    None
                                }
                            };
                        }
                    }
                }
                false => (),
            }
            break;
        } else if !get_name.shown() {
            caretaker.pop();
            return None;
        }
    }

    None
}

/// Change 2-nd name of already known reader

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
            match mes {
                true => {
                    get_family.hide();

                    if let Ok(new_family) = get_family.set_input() {
                        unsafe {
                            return match reader_base
                                .change_family(ind, new_family.get_unchecked(0).clone())
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

                                    Some(new_family.get_unchecked(0).clone())
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
                                    caretaker.pop();
                                    None
                                }

                                Err(1) => {
                                    alert(
										500,
										500,
										match lang {
											Lang::English => "Reader with same parameters already exists",
											Lang::Russian => "Читатель с предложенными параметрами уже существует",
										},
									);
                                    caretaker.pop();
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
                                    caretaker.pop();
                                    None
                                }
                            };
                        }
                    }
                }
                false => (),
            }
            break;
        } else if !get_family.shown() {
            caretaker.pop();
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
            match mes {
                true => {
                    get_father.hide();

                    if let Ok(new_father) = get_father.set_input() {
                        unsafe {
                            return match reader_base
                                .change_father(ind, new_father.get_unchecked(0).clone())
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
                                    Some(new_father.get_unchecked(0).clone())
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
                                    caretaker.pop();
                                    None
                                }

                                Err(1) => {
                                    alert(
										500,
										500,
										match lang {
											Lang::English => "Reader with same parameters already exists",
											Lang::Russian => "Читатель с предложенными параметрами уже существует",
										},
									);
                                    caretaker.pop();
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
                                    caretaker.pop();
                                    None
                                }
                            };
                        }
                    }
                }
                false => (),
            }
            break;
        } else if !get_father.shown() {
            caretaker.pop();
            return None;
        }
    }

    None
}

/// Changes age of already known reader

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
    let (s3, r3) = app::channel();
    let mut get_age = Input3::<IntInput, IntInput, IntInput>::new(
        match lang {
            Lang::English => "Change Age",
            Lang::Russian => "Изменить Возраст",
        },
        match lang {
            Lang::English => "New Birth Day",
            Lang::Russian => "Новый День Рождения",
        },
        match lang {
            Lang::English => "New Birth Month",
            Lang::Russian => "Новый Месяц Рождения",
        },
        match lang {
            Lang::English => "New Birth Year",
            Lang::Russian => "Новый Год Рождения",
        },
    );

    caretaker.add_memento(reader_base, book_system, genres);

    get_age.show();
    (*get_age.ok).borrow_mut().emit(s3, true);

    while app.wait() {
        if let Some(mes) = r3.recv() {
            match mes {
                true => {
                    get_age.hide();

                    if let Ok(new_age) = get_age.set_input() {
                        unsafe {
                            if new_age.get_unchecked(0).is_empty() {
                                alert(
                                    500,
                                    500,
                                    match lang {
                                        Lang::English => "'New Birth Day' is empty",
                                        Lang::Russian => "'Новый День Рождения' пусто",
                                    },
                                );
                                caretaker.pop();
                                return None;
                            }

                            if new_age.get_unchecked(1).is_empty() {
                                alert(
                                    500,
                                    500,
                                    match lang {
                                        Lang::English => "'New Birth Month' is empty",
                                        Lang::Russian => "'Новый Месяц Рождения' пусто",
                                    },
                                );
                                caretaker.pop();
                                return None;
                            }

                            if new_age.get_unchecked(2).is_empty() {
                                alert(
                                    500,
                                    500,
                                    match lang {
                                        Lang::English => "'New Birth Year' is empty",
                                        Lang::Russian => "'Новый Год Рождения' пусто",
                                    },
                                );
                                caretaker.pop();
                                return None;
                            }

                            return match reader_base.change_age(ind, new_age) {
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
                                    Some((**reader_base.readers.get_unchecked(ind)).borrow().age())
                                }

                                Err(0) => {
                                    alert(
                                        500,
                                        500,
                                        match lang {
                                            Lang::English => "'New Age' input error",
                                            Lang::Russian => "Ошибка ввода 'Нового возраста'",
                                        },
                                    );
                                    caretaker.pop();
                                    None
                                }

                                Err(_) => {
                                    alert(
										500,
										500,
										match lang {
											Lang::English => "Reader with same parameters already exists",
											Lang::Russian => "Читатель с предложенными параметрами уже существует",
										}
									);
                                    caretaker.pop();
                                    None
                                }
                            };
                        }
                    }
                }
                false => (),
            }
            break;
        } else if !get_age.shown() {
            caretaker.pop();
            return None;
        }
    }

    None
}

/// Function that changes reader's name.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn change_name(
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
            Lang::English => "Change Name",
            Lang::Russian => "Изменить имя",
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
                        let rind;

                        match check_reader(reader_base, &reader, lang) {
                            Ok(x) => rind = x,
                            Err(_) => return,
                        }

                        change_name_simple(
                            rind,
                            reader_base,
                            book_system,
                            genres,
                            caretaker,
                            app,
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

/// Function that changes reader's second name.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn change_family(
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
            Lang::English => "Change 2-nd Name",
            Lang::Russian => "Изменить фамилию",
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
                        let rind;

                        match check_reader(reader_base, &reader, lang) {
                            Ok(x) => rind = x,
                            Err(_) => return,
                        }

                        change_family_simple(
                            rind,
                            reader_base,
                            book_system,
                            genres,
                            caretaker,
                            app,
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

/// Function that changes reader's middle name.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn change_father(
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
            Lang::English => "Change Middle Name",
            Lang::Russian => "Изменить Отчество",
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
                        let rind;

                        match check_reader(reader_base, &reader, lang) {
                            Ok(x) => rind = x,
                            Err(_) => return,
                        }

                        change_father_simple(
                            rind,
                            reader_base,
                            book_system,
                            genres,
                            caretaker,
                            app,
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

/// Function that changes reader's age.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn change_age(
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
            Lang::English => "Change Age",
            Lang::Russian => "Изменить Возраст",
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
                        let rind;

                        match check_reader(reader_base, &reader, lang) {
                            Ok(x) => rind = x,
                            Err(_) => return,
                        }

                        change_age_simple(
                            rind,
                            reader_base,
                            book_system,
                            genres,
                            caretaker,
                            app,
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
