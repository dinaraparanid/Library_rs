extern crate chrono;
extern crate fltk;
extern crate fltk_calendar;

use crate::{
    actions::{book::utils::check_book, genres::all_genres},
    books::{book::Book, book_sys::BookSystem, date::Date, genres::Genres},
    change::{input3::Input3, Inputable},
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

use chrono::{Datelike, Local};
use fltk_calendar::calendar::Calendar;
use std::{cell::RefCell, rc::Weak};

/// Function that gives
/// book to known reader

pub(crate) fn give_book_known_reader(
    rind: usize,
    reader_base: &mut ReaderBase,
    book_system: &mut BookSystem,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) -> Option<String> {
    match all_genres(genres, book_system, app, lang) {
        Some(book) => {
            let label = book.label().unwrap();
            let book = label
                .trim()
                .split(' ')
                .take(3)
                .map(|s| s.to_string())
                .collect::<Vec<_>>();

            if book.len() != 3 {
                alert(
                    500,
                    500,
                    match lang {
                        Lang::English => "Book isn't selected",
                        Lang::Russian => "Книга не выбрана",
                    },
                );
                return None;
            }

            let mut book = book.into_iter();

            match book_system.find_book(
                &book.next().unwrap(),
                &book.next().unwrap(),
                book.next().unwrap().parse().unwrap(),
            ) {
                Some(bind) => {
                    caretaker.add_memento(reader_base, book_system, genres);

                    let mut win =
                        fltk::window::SingleWindow::new(800, 500, 200, 100, "Choose birth date");

                    let _ = fltk::frame::Frame::new(
                        30,
                        10,
                        150,
                        50,
                        match lang {
                            Lang::English => "Choose finish date",
                            Lang::Russian => "Выберите срок сдачи",
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

                                let date = Calendar::default().get_date();

                                return match date {
                                    Some(date) => {
                                        let date = Date::from(date);

                                        if date < Date::from(chrono::Local::now()) {
                                            alert(
                                                500,
                                                500,
                                                match lang {
                                                    Lang::English => {
                                                        "The deadline must be no later than the day of issue"
                                                    }
                                                    Lang::Russian => {
                                                        "Дедлайн должен быть не позднее дня выдачи"
                                                    }
                                                },
                                            );
                                            return None;
                                        }

                                        let simple_book = unsafe {
                                            (*book_system.books.get_unchecked(bind))
                                                .borrow_mut()
                                                .get_unused()
                                        };

                                        match simple_book {
                                            None => {
                                                alert(
                                                    500,
                                                    500,
                                                    match lang {
                                                        Lang::English => "There are no free books",
                                                        Lang::Russian => {
                                                            "Свободных книг не осталось"
                                                        }
                                                    },
                                                );
                                                caretaker.pop().unwrap();
                                                None
                                            }

                                            Some(sim) => {
                                                if unsafe {
                                                    (**reader_base.readers.get_unchecked(rind))
                                                        .borrow()
                                                        .reading
                                                        .is_some()
                                                } {
                                                    alert(
                                                        500,
                                                        500,
                                                        match lang {
                                                            Lang::English => {
                                                                "This reader is already reading book"
                                                            }
                                                            Lang::Russian => {
                                                                "Этот читатель уже читает книгу"
                                                            }
                                                        },
                                                    );

                                                    caretaker.pop().unwrap();
                                                    return None;
                                                }

                                                unsafe {
                                                    (*reader_base.readers.get_unchecked(rind))
                                                        .borrow_mut()
                                                        .start_reading(
                                                            (*book_system
                                                                .books
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
                                                        reader_base.readers.get_unchecked(rind),
                                                        date,
                                                    );
                                                }

                                                fltk::dialog::message(
                                                    500,
                                                    500,
                                                    match lang {
                                                        Lang::English => {
                                                            "Book successfully given to reader"
                                                        }
                                                        Lang::Russian => {
                                                            "Книга успешно выдана читателю"
                                                        }
                                                    },
                                                );

                                                book_system.save();
                                                reader_base.save();

                                                Some(unsafe {
                                                    (*reader_base
                                                        .readers
                                                        .get_unchecked(rind)
                                                        .borrow()
                                                        .reading
                                                        .as_ref()
                                                        .unwrap()
                                                        .upgrade()
                                                        .unwrap())
                                                    .borrow()
                                                    .to_string(book_system)
                                                })
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
                                        None
                                    }
                                };
                            }
                        } else if !win.shown() {
                            caretaker.pop().unwrap();
                            return None;
                        }
                    }
                }

                None => {
                    alert(
                        500,
                        500,
                        match lang {
                            Lang::English => "Book isn't selected",
                            Lang::Russian => "Книга не выбрана",
                        },
                    );
                    return None;
                }
            }
        }

        None => {
            alert(500, 500, "Book wasn't selected");
            return None;
        }
    }

    None
}

/// **DEPRECATED**
///
/// Used before. Requires input.
/// Consider using **give_book_known_reader() instead**
///
/// Gives book to known reader (user input version)

#[allow(dead_code)]
#[deprecated(note = "Used before. Requires input. Consider using give_book_known_reader() instead")]
fn give_book_known_reader_input(
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
            if msg {
                inp2.hide();

                if let Ok(book) = inp2.set_input() {
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
                            if mes {
                                if let Ok(dat) = inp3.set_input() {
                                    return match unsafe {
                                        dat.get_unchecked(0).trim().parse::<u8>()
                                    } {
                                        Ok(day) => {
                                            match unsafe {
                                                dat.get_unchecked(1).trim().parse::<u8>()
                                            } {
                                                Ok(month) => {
                                                    match unsafe {
                                                        dat.get_unchecked(2).trim().parse::<u16>()
                                                    } {
                                                        Ok(year) => {
                                                            match Date::new(day, month, year) {
                                                                Err(_) => {
                                                                    alert(
                                                                        500,
                                                                        500,
                                                                        match lang {
                                                                            Lang::English => "Incorrect return date",
                                                                            Lang::Russian => "Некорректная дата возврата",
                                                                        },
                                                                    );
                                                                    caretaker.pop().unwrap();
                                                                    None
                                                                }

                                                                Ok(date) => {
                                                                    if date
                                                                        < Date::from(
                                                                            chrono::Local::now(),
                                                                        )
                                                                    {
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

                                                                    let simple_book = unsafe {
                                                                        (*book_system
                                                                            .books
                                                                            .get_unchecked(bind))
                                                                        .borrow_mut()
                                                                        .get_unused()
                                                                    };

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
                                                                                .pop()
                                                                                .unwrap();
                                                                            None
                                                                        }

                                                                        Some(sim) => {
                                                                            if unsafe {
                                                                                (**reader_base
                                                                                    .readers
                                                                                    .get_unchecked(
                                                                                        rind,
                                                                                    ))
                                                                                .borrow()
                                                                                .reading
                                                                                .is_some()
                                                                            } {
                                                                                alert(
                                                                                    500,
                                                                                    500,
                                                                                    match lang {
                                                                                        Lang::English => concat!(
                                                                                        "This reader is already",
                                                                                        " reading book"),
                                                                                        Lang::Russian => concat!("Этот читатель",
                                                                                        " уже читает книгу"),
                                                                                    },
                                                                                );

                                                                                caretaker
                                                                                    .pop()
                                                                                    .unwrap();
                                                                                return None;
                                                                            }

                                                                            unsafe {
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
                                                                            }

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

                                                                            book_system.save();
                                                                            reader_base.save();

                                                                            Some(unsafe {
                                                                                (*reader_base
                                                                                    .readers
                                                                                    .get_unchecked(
                                                                                        rind,
                                                                                    )
                                                                                    .borrow()
                                                                                    .reading
                                                                                    .as_ref()
                                                                                    .unwrap()
                                                                                    .upgrade()
                                                                                    .unwrap())
                                                                                .borrow()
                                                                                .to_string(
                                                                                    book_system,
                                                                                )
                                                                            })
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
                                                                    Lang::English => {
                                                                        "'Year' input error"
                                                                    }
                                                                    Lang::Russian => {
                                                                        "Ошибка ввода 'Года'"
                                                                    }
                                                                },
                                                            );
                                                            caretaker.pop().unwrap();
                                                            None
                                                        }
                                                    }
                                                }

                                                Err(_) => {
                                                    alert(
                                                        500,
                                                        500,
                                                        match lang {
                                                            Lang::English => "'Month' input error",
                                                            Lang::Russian => {
                                                                "Ошибка ввода 'Месяца'"
                                                            }
                                                        },
                                                    );
                                                    caretaker.pop().unwrap();
                                                    None
                                                }
                                            }
                                        }

                                        Err(_) => {
                                            alert(
                                                500,
                                                500,
                                                match lang {
                                                    Lang::English => "'Day' input error",
                                                    Lang::Russian => "Ошибка ввода 'Дня'",
                                                },
                                            );
                                            caretaker.pop().unwrap();
                                            None
                                        }
                                    };
                                }
                            }
                            break;
                        } else if !inp3.shown() {
                            caretaker.pop().unwrap();
                            return None;
                        }
                    }
                }
            }
            break;
        } else if !inp2.shown() {
            caretaker.pop().unwrap();
            return None;
        }
    }

    None
}

/// Function that gets
/// book from known reader

#[inline]
pub(crate) fn get_book_known_reader(
    rind: usize,
    reader_base: &mut ReaderBase,
    book_system: &mut BookSystem,
    genres: &Genres,
    caretaker: &mut Caretaker,
    lang: Lang,
) -> bool {
    return match reader_base.get_book(rind) {
        Some(book) => {
            caretaker.add_memento(reader_base, book_system, genres);

            let book = {
                (*book.upgrade().unwrap())
                    .borrow()
                    .the_book
                    .as_ref()
                    .unwrap()
                    .upgrade()
                    .unwrap()
            };

            let bind = book_system
                .iter()
                .position(|b| b.as_ptr() == book.as_ptr())
                .unwrap();

            let sim;

            unsafe {
                sim = (*book_system.books.get_unchecked(bind))
                    .borrow_mut()
                    .find_by_reader(reader_base.readers.get_unchecked(rind))
                    .unwrap();

                (*reader_base.readers.get_unchecked_mut(rind))
                    .borrow_mut()
                    .finish_reading();

                match (*(*book_system.books.get_unchecked(bind))
                    .borrow_mut()
                    .books
                    .get_unchecked(sim))
                .borrow_mut()
                .finish_reading()
                {
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
            }

            let cab = unsafe {
                (*(*book_system.books.get_unchecked(bind))
                    .borrow()
                    .books
                    .get_unchecked(sim))
                .borrow()
                .cabinet
            };

            let shelf = unsafe {
                (*(*book_system.books.get_unchecked(bind))
                    .borrow()
                    .books
                    .get_unchecked(sim))
                .borrow()
                .shelf
            };

            fltk::dialog::message(
                500,
                500,
                match lang {
                    Lang::English => format!("Book location: cabinet {}, shelf {}", cab, shelf),
                    Lang::Russian => format!("Расположение книги: шкаф {}, полка {}", cab, shelf),
                }
                .as_str(),
            );

            book_system.save();
            reader_base.save();
            true
        }

        None => {
            alert(
                500,
                500,
                match lang {
                    Lang::English => "This reader wasn't reading searching book",
                    Lang::Russian => "Этот читатель не читает искомую книгу",
                },
            );

            false
        }
    };
}

/// **DEPRECATED**
///
/// Used before. Requires user input.
/// Consider using **get_book_known_reader() instead**
///
/// Function that gets book from known reader

#[allow(dead_code)]
#[deprecated(
    note = "Used before. Requires user input. Consider using get_book_known_reader() instead"
)]
fn get_book_known_reader_input(
    rind: usize,
    reader_base: &mut ReaderBase,
    book_system: &mut BookSystem,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) -> bool {
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
            if msg {
                inp2.hide();

                if let Ok(book) = inp2.set_input() {
                    return match check_book(book_system, &book, lang) {
                        Ok(bind) => {
                            match unsafe {
                                (*book_system.books.get_unchecked(bind))
                                    .borrow_mut()
                                    .find_by_reader(reader_base.readers.get_unchecked(rind))
                            } {
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
                                    caretaker.pop().unwrap();
                                    false
                                }

                                Some(sim) => {
                                    unsafe {
                                        (*reader_base.readers.get_unchecked_mut(rind))
                                            .borrow_mut()
                                            .finish_reading();
                                    }

                                    match unsafe {
                                        (*(*book_system.books.get_unchecked(bind))
                                            .borrow_mut()
                                            .books
                                            .get_unchecked(sim))
                                        .borrow_mut()
                                        .finish_reading()
                                    } {
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

                                    let cab = unsafe {
                                        (*(*book_system.books.get_unchecked(bind))
                                            .borrow()
                                            .books
                                            .get_unchecked(sim))
                                        .borrow()
                                        .cabinet
                                    };

                                    let shelf = unsafe {
                                        (*(*book_system.books.get_unchecked(bind))
                                            .borrow()
                                            .books
                                            .get_unchecked(sim))
                                        .borrow()
                                        .shelf
                                    };

                                    fltk::dialog::message(
                                        500,
                                        500,
                                        match lang {
                                            Lang::English => format!(
                                                "Book location: cabinet {}, shelf {}",
                                                cab, shelf
                                            ),
                                            Lang::Russian => format!(
                                                "Расположение книги: шкаф {}, полка {}",
                                                cab, shelf
                                            ),
                                        }
                                        .as_str(),
                                    );

                                    book_system.save();
                                    reader_base.save();
                                    true
                                }
                            }
                        }
                        Err(_) => false,
                    };
                }
            }
            break;
        } else if !inp2.shown() {
            caretaker.pop().unwrap();
            return false;
        }
    }

    false
}

/// Function that changes
/// return date for already known book

#[inline]
pub fn change_return_date_simple(
    book_op: &Option<Weak<RefCell<Book>>>,
    book_system: &mut BookSystem,
    reader_base: &ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    lang: Lang,
) -> bool {
    caretaker.add_memento(reader_base, book_system, genres);

    return match book_op {
        None => {
            alert(
                500,
                500,
                match lang {
                    Lang::English => "This reader isn't reading anything",
                    Lang::Russian => "Этот читатель ни читает книгу",
                },
            );

            caretaker.pop().unwrap();
            false
        }

        Some(book) => {
            return match Calendar::default().get_date() {
                None => {
                    alert(
                        500,
                        500,
                        match lang {
                            Lang::English => "Date wasn't selected",
                            Lang::Russian => "Дата не была выбрана",
                        },
                    );
                    false
                }

                Some(date) => {
                    let new_date = Date {
                        day: date.day() as u8,
                        month: date.month() as u8,
                        year: date.year() as u16,
                    };

                    let start = ((*(*book.upgrade().unwrap())
                        .borrow_mut()
                        .readers
                        .last_mut()
                        .unwrap())
                    .1)
                        .0;

                    if new_date >= start && new_date >= Date::from(Local::now()) {
                        ((*(*book.upgrade().unwrap())
                            .borrow_mut()
                            .readers
                            .last_mut()
                            .unwrap())
                        .1)
                            .1 = new_date;

                        fltk::dialog::message(
                            500,
                            500,
                            match lang {
                                Lang::English => "Date is successfully changed",
                                Lang::Russian => "Дата успешно изменена",
                            },
                        );

                        book_system.save();
                        true
                    } else {
                        alert(
                            500,
                            500,
                            match lang {
                                Lang::English => {
                                    concat!(
                                        "Date can only be not earlier than deadline",
                                        " and not earlier than today"
                                    )
                                }

                                Lang::Russian => {
                                    concat!(
                                        "Дата обязана быть не позже дедлайна",
                                        " и не позже сегоднешней даты"
                                    )
                                }
                            },
                        );

                        caretaker.pop().unwrap();
                        false
                    }
                }
            }
        }
    };
}

/// **DEPRECATED**
///
/// Used before. Requires user input.
/// Consider using **change_return_date_simple() instead**
///
/// Function that changes
/// return date for already known book

#[allow(dead_code)]
#[deprecated(
    note = "Used before. Requires user input. Consider using change_return_date_simple() instead"
)]
fn change_return_date_simple_input(
    book_op: &Option<Weak<RefCell<Book>>>,
    book_system: &mut BookSystem,
    reader_base: &ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) -> bool {
    caretaker.add_memento(reader_base, book_system, genres);

    return match book_op {
        None => {
            alert(
                500,
                500,
                match lang {
                    Lang::English => "This reader isn't reading anything",
                    Lang::Russian => "Этот читатель ни читает книгу",
                },
            );

            caretaker.pop().unwrap();
            false
        }

        Some(book) => {
            let (s2, r2) = fltk::app::channel();
            let mut inp = Input3::<IntInput, IntInput, IntInput>::new(
                match lang {
                    Lang::English => "Set New Date",
                    Lang::Russian => "Изменить дату",
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

            inp.show();
            (*inp.ok).borrow_mut().emit(s2, true);

            while app.wait() {
                if let Some(message) = r2.recv() {
                    if message {
                        inp.hide();

                        if let Ok(date) = inp.set_input() {
                            return match unsafe { date.get_unchecked(0).trim().parse::<u8>() } {
                                Ok(day) => match unsafe {
                                    date.get_unchecked(1).trim().parse::<u8>()
                                } {
                                    Ok(month) => {
                                        match unsafe { date.get_unchecked(2).trim().parse::<u16>() }
                                        {
                                            Ok(year) => {
                                                match Date::new(day, month, year) {
                                                    Ok(new_date) => {
                                                        let start = ((*(*book.upgrade().unwrap())
                                                            .borrow_mut()
                                                            .readers
                                                            .last_mut()
                                                            .unwrap())
                                                        .1)
                                                            .0;

                                                        if new_date >= start
                                                            && new_date >= Date::from(Local::now())
                                                        {
                                                            ((*(*book.upgrade().unwrap())
                                                                .borrow_mut()
                                                                .readers
                                                                .last_mut()
                                                                .unwrap())
                                                            .1)
                                                                .1 = new_date;

                                                            fltk::dialog::message(
                                                                500,
                                                                500,
                                                                match lang {
                                                                    Lang::English => "Date is successfully changed",
                                                                    Lang::Russian => "Дата успешно изменена",
                                                                }
                                                            );

                                                            book_system.save();
                                                            true
                                                        } else {
                                                            alert(
                                                                500,
                                                                500,
                                                                match lang {
                                                                    Lang::English => {
                                                                        concat!(
                                                                        "Date can only be not earlier than deadline",
                                                                        " and not earlier than today"
                                                                        )
                                                                    }

                                                                    Lang::Russian => {
                                                                        concat!(
                                                                        "Дата обязана быть не позже дедлайна",
                                                                        " и не позже сегоднешней даты"
                                                                        )
                                                                    }
                                                                },
                                                            );

                                                            caretaker.pop().unwrap();
                                                            false
                                                        }
                                                    }

                                                    Err(_) => {
                                                        alert(
                                                            500,
                                                            500,
                                                            match lang {
                                                                Lang::English => "Incorrect date",
                                                                Lang::Russian => {
                                                                    "Некорректная дата"
                                                                }
                                                            },
                                                        );

                                                        caretaker.pop().unwrap();
                                                        false
                                                    }
                                                }
                                            }

                                            Err(_) => {
                                                alert(
                                                    500,
                                                    500,
                                                    match lang {
                                                        Lang::English => "Incorrect 'Year' input",
                                                        Lang::Russian => "Некорректный ввод 'Года'",
                                                    },
                                                );

                                                caretaker.pop().unwrap();
                                                false
                                            }
                                        }
                                    }

                                    Err(_) => {
                                        alert(
                                            500,
                                            500,
                                            match lang {
                                                Lang::English => "Incorrect 'Month' input",
                                                Lang::Russian => "Некорректный ввод 'Месяца'",
                                            },
                                        );

                                        caretaker.pop().unwrap();
                                        false
                                    }
                                },

                                Err(_) => {
                                    alert(
                                        500,
                                        500,
                                        match lang {
                                            Lang::English => "Incorrect 'Day' input",
                                            Lang::Russian => "Некорректный ввод 'Дня'",
                                        },
                                    );

                                    caretaker.pop().unwrap();
                                    false
                                }
                            };
                        }
                    }
                    break;
                } else if !inp.shown() {
                    caretaker.pop().unwrap();
                    return false;
                }
            }

            false
        }
    };
}
