extern crate fltk;

use crate::{
    actions::{book::*, read::*},
    books::{book_sys::BookSystem, date::Date, genres::Genres},
    change::{input3::Input3, input4::Input4, Inputable},
    reading::read_base::ReaderBase,
    restore::caretaker::Caretaker,
    Lang,
};

use fltk::{
    app,
    app::{channel, App},
    dialog::alert,
    input::*,
    WidgetExt,
};

use std::num::ParseIntError;

/// Gives book to known reader

pub(crate) fn give_book_known_reader(
    rind: usize,
    reader_base: &mut ReaderBase,
    book_system: &mut BookSystem,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) -> Option<String> {
    let (s3, r3) = fltk::app::channel();
    let mut inp2 = Input3::<Input, Input, IntInput>::new(
        match lang {
            Lang::English => "Find Book",
            Lang::Russian => "Поиск Книги",
        },
        match lang {
            Lang::English => "Title",
            Lang::Russian => "Название",
        },
        match lang {
            Lang::English => "Author",
            Lang::Russian => "Автор",
        },
        match lang {
            Lang::English => "Amount of Pages",
            Lang::Russian => "Количество страниц",
        },
    );

    caretaker.add_memento(reader_base, book_system, genres);

    inp2.show();
    (*inp2.ok).borrow_mut().emit(s3, true);

    while app.wait() {
        if let Some(msg) = r3.recv() {
            match msg {
                true => {
                    inp2.hide();

                    if let Ok(book) = inp2.set_input() {
                        unsafe {
                            let bind;

                            match check_book(book_system, &book, lang) {
                                Ok(x) => bind = x,
                                Err(_) => return None,
                            }

                            let (s4, r4) = fltk::app::channel();
                            let mut inp3 = Input3::<IntInput, IntInput, IntInput>::new(
                                match lang {
                                    Lang::English => "Set Return Date",
                                    Lang::Russian => "Дедлайн",
                                },
                                match lang {
                                    Lang::English => "Day (number)",
                                    Lang::Russian => "День (номер)",
                                },
                                match lang {
                                    Lang::English => "Month (number)",
                                    Lang::Russian => "Месяц (номер)",
                                },
                                match lang {
                                    Lang::English => "Year",
                                    Lang::Russian => "Год",
                                },
                            );

                            inp3.show();
                            (*inp3.ok).borrow_mut().emit(s4, true);

                            while app.wait() {
                                if let Some(mes) = r4.recv() {
                                    match mes {
                                        true => {
                                            let date_params = inp3.set_input();
                                            inp3.hide();

                                            if let Ok(dat) = date_params {
                                                return match dat
                                                    .get_unchecked(0)
                                                    .trim()
                                                    .parse::<u8>()
                                                {
                                                    Ok(day) => {
                                                        match dat
                                                            .get_unchecked(1)
                                                            .trim()
                                                            .parse::<u8>()
                                                        {
                                                            Ok(month) => {
                                                                match dat
                                                                    .get_unchecked(2)
                                                                    .trim()
                                                                    .parse::<u16>()
                                                                {
                                                                    Ok(year) => {
                                                                        match Date::new(
                                                                            day, month, year,
                                                                        ) {
                                                                            Err(_) => {
                                                                                alert(
                                                                                    500,
                                                                                    500,
                                                                                    match lang {
                                                                                        Lang::English => "Incorrect return date",
                                                                                        Lang::Russian => "Некорректная дата возврата",
                                                                                    },
                                                                                );
                                                                                caretaker.pop();
                                                                                None
                                                                            }

                                                                            Ok(date) => {
                                                                                if date < Date::from(chrono::Local::now()) {
                                                                                    alert(
                                                                                        500,
                                                                                        500,
                                                                                        match lang {
                                                                                            Lang::English =>
                                                                                                concat!("The deadline must be no later",
                                                                                                " than the day of issue"),
                                                                                            Lang::Russian =>
                                                                                                concat!("Дедлайн должен быть не",
                                                                                                " позднее дня выдачи"),
                                                                                        });
                                                                                    return None;
                                                                                }

                                                                                let simple_book =
                                                                                    (*book_system.books.get_unchecked(bind))
                                                                                        .borrow_mut()
                                                                                        .get_unused();

                                                                                match simple_book {
                                                                                    None => {
                                                                                        alert(
                                                                                            500,
                                                                                            500,
                                                                                            match lang {
                                                                                                Lang::English => "There are no free books",
                                                                                                Lang::Russian =>
                                                                                                    "Свободных книг не осталось",
                                                                                            },
                                                                                        );
                                                                                        caretaker
                                                                                            .pop();
                                                                                        None
                                                                                    }

                                                                                    Some(sim) => {
                                                                                        if (**reader_base.readers
                                                                                                .get_unchecked(rind))
                                                                                            .borrow()
                                                                                            .reading
                                                                                            .is_some() {
                                                                                            alert(
                                                                                                500,
                                                                                                500,
                                                                                                match lang {
                                                                                                    Lang::English => concat!(
                                                                                                    "This reader is already",
                                                                                                    " reading book"),
                                                                                                    Lang::Russian => 
                                                                                                        concat!("Этот читатель",
                                                                                                        " уже читает книгу"),
                                                                                                },
                                                                                            );

                                                                                            caretaker.pop();
                                                                                            return None;
                                                                                        }

                                                                                        (*reader_base.readers.get_unchecked(rind))
                                                                                            .borrow_mut()
                                                                                            .start_reading(
                                                                                            (*book_system.books
                                                                                                    .get_unchecked(bind))
                                                                                                .borrow_mut()
                                                                                                .books
                                                                                                .get_unchecked(sim),
                                                                                        );

                                                                                        (*(*book_system.books.get_unchecked(bind))
                                                                                                .borrow_mut()
                                                                                                .books
                                                                                                .get_unchecked(sim))
                                                                                            .borrow_mut()
                                                                                            .start_reading(
                                                                                            reader_base.readers
                                                                                                .get_unchecked(rind),
                                                                                            date,
                                                                                        );

                                                                                        fltk::dialog::message(
                                                                                            500,
                                                                                            500,
                                                                                            match lang {
                                                                                                Lang::English =>
                                                                                                    "Book successfully given to reader",
                                                                                                Lang::Russian =>
                                                                                                    "Книга успешно выдана читателю",
                                                                                            },
                                                                                        );

                                                                                        book_system
                                                                                            .save();
                                                                                        reader_base
                                                                                            .save();

                                                                                        Some((*reader_base
                                                                                                .readers
                                                                                                .get_unchecked(rind)
                                                                                                .borrow()
                                                                                                .reading
                                                                                                .as_ref()
                                                                                                .unwrap()
                                                                                                .upgrade()
                                                                                                .unwrap())
                                                                                            .borrow()
                                                                                            .to_string(book_system))
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }

                                                                    Err(_) => {
                                                                        alert(
                                                                            500,
                                                                            500,
                                                                            match lang {
                                                                                Lang::English => "'Year' input error",
                                                                                Lang::Russian => "Ошибка ввода 'Года'",
                                                                            },
                                                                        );
                                                                        caretaker.pop();
                                                                        None
                                                                    }
                                                                }
                                                            }

                                                            Err(_) => {
                                                                alert(
                                                                    500,
                                                                    500,
                                                                    match lang {
                                                                        Lang::English => {
                                                                            "'Month' input error"
                                                                        }
                                                                        Lang::Russian => {
                                                                            "Ошибка ввода 'Месяца'"
                                                                        }
                                                                    },
                                                                );
                                                                caretaker.pop();
                                                                None
                                                            }
                                                        }
                                                    }

                                                    Err(_) => {
                                                        alert(
                                                            500,
                                                            500,
                                                            match lang {
                                                                Lang::English => {
                                                                    "'Day' input error"
                                                                }
                                                                Lang::Russian => {
                                                                    "Ошибка ввода 'Дня'"
                                                                }
                                                            },
                                                        );
                                                        caretaker.pop();
                                                        None
                                                    }
                                                };
                                            }
                                        }
                                        false => (),
                                    }
                                } else if !inp3.shown() {
                                    caretaker.pop();
                                    return None;
                                }
                            }
                        }
                    }
                }
                false => (),
            }
        } else if !inp2.shown() {
            caretaker.pop();
            return None;
        }
    }

    None
}

/// Gets book from known reader

#[inline]
pub(crate) fn get_book_known_reader(
    rind: usize,
    reader_base: &mut ReaderBase,
    book_system: &mut BookSystem,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) -> Option<()> {
    let (s3, r3) = fltk::app::channel();
    let mut inp2 = Input3::<Input, Input, IntInput>::new(
        match lang {
            Lang::English => "Find Book",
            Lang::Russian => "Поиск Книги",
        },
        match lang {
            Lang::English => "Title",
            Lang::Russian => "Название книги",
        },
        match lang {
            Lang::English => "Author",
            Lang::Russian => "Автор",
        },
        match lang {
            Lang::English => "Amount of Pages",
            Lang::Russian => "Количество страниц",
        },
    );

    caretaker.add_memento(reader_base, book_system, genres);

    inp2.show();
    (*inp2.ok).borrow_mut().emit(s3, true);

    while app.wait() {
        if let Some(msg) = r3.recv() {
            match msg {
                true => {
                    inp2.hide();

                    if let Ok(book) = inp2.set_input() {
                        let bind;

                        match check_book(book_system, &book, lang) {
                            Ok(x) => bind = x,
                            Err(_) => return None,
                        }

                        unsafe {
                            let simple = (*book_system.books.get_unchecked(bind))
                                .borrow_mut()
                                .find_by_reader(reader_base.readers.get_unchecked(rind));

                            return match simple {
                                None => {
                                    alert(
                                        500,
                                        500,
                                        match lang {
                                            Lang::English => {
                                                "This reader wasn't reading searching book"
                                            }

                                            Lang::Russian => {
                                                "Этот читатель не читает искомую книгу"
                                            }
                                        },
                                    );
                                    caretaker.pop();
                                    None
                                }

                                Some(sim) => {
                                    (*reader_base.readers.get_unchecked_mut(rind)).borrow_mut().finish_reading();

                                    match (*(*book_system.books.get_unchecked(bind))
                                            .borrow_mut()
                                            .books
                                            .get_unchecked(sim))
                                        .borrow_mut()
                                        .finish_reading() {
                                        Ok(_) => fltk::dialog::message(
                                            500,
                                            500,
                                            match lang {
                                                Lang::English => "Book is returned",
                                                Lang::Russian => "Книга возвращена",
                                            },
                                        ),

                                        Err(_) => fltk::dialog::message(
                                            500,
                                            500,
                                            match lang {
                                                Lang::English => "Book is returned after deadline",
                                                Lang::Russian => "Книга возвращена после дедлайна",
                                            },
                                        ),
                                    }

                                    book_system.save();
                                    reader_base.save();
                                    Some(())
                                }
                            }
                        }
                    }
                }
                false => (),
            }
        } else if !inp2.shown() {
            caretaker.pop();
            return None;
        }
    }

    None
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
    let mut inp = Input4::<Input, Input, Input, IntInput>::new(
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
        match lang {
            Lang::English => "Age",
            Lang::Russian => "Возраст",
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
                        match check_reader(reader_base, &reader, lang) {
                            Ok(x) => {
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

                            Err(_) => return,
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
    let mut inp = Input4::<Input, Input, Input, IntInput>::new(
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
        match lang {
            Lang::English => "Age",
            Lang::Russian => "Возраст",
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
                        match check_reader(reader_base, &reader, lang) {
                            Ok(x) => {
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

                            Err(_) => return,
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
