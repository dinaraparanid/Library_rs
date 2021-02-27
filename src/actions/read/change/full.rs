extern crate fltk;

use crate::{
    actions::read::{change::simple::*, utils::check_reader},
    books::{book_sys::BookSystem, genres::Genres},
    change::{input4::Input4, Inputable},
    reading::read_base::ReaderBase,
    restore::caretaker::Caretaker,
    Lang,
};

use fltk::{app, app::App, input::Input, prelude::*};

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
