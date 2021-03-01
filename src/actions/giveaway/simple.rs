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
    tree::TreeItem,
};

use chrono::Local;
use fltk_calendar::calendar::Calendar;
use std::{cell::RefCell, rc::Weak};

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
                            match msg {
                                true => {
                                    win.hide();

                                    let cal = Calendar::default();
                                    let date = cal.get_date();

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
                                                            Lang::English => {
                                                                "There are no free books"
                                                            }
                                                            Lang::Russian => {
                                                                "Свободных книг не осталось"
                                                            }
                                                        },
                                                    );
                                                    caretaker.pop();
                                                    None
                                                }

                                                Some(sim) => unsafe {
                                                    if (**reader_base.readers.get_unchecked(rind))
                                                        .borrow()
                                                        .reading
                                                        .is_some()
                                                    {
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

                                                        caretaker.pop();
                                                        return None;
                                                    }

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

                                                    Some(
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
                                                        .to_string(book_system),
                                                    )
                                                },
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
                                            caretaker.pop();
                                            None
                                        }
                                    };
                                }

                                false => (),
                            }
                        } else if !win.shown() {
                            caretaker.pop();
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
/// Gives book to known reader (user input version)

#[deprecated]
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

                            let mut win = fltk::window::SingleWindow::new(
                                800,
                                500,
                                200,
                                100,
                                "Choose birth date",
                            );

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
                                    match msg {
                                        true => {
                                            win.hide();

                                            let cal = Calendar::default();
                                            let date = cal.get_date();

                                            return match date {
                                                Some(date) => {
                                                    let date = Date::from(date);

                                                    if date < Date::from(chrono::Local::now()) {
                                                        alert(
                                                            500,
                                                            500,
                                                            match lang {
                                                                Lang::English => "The deadline must be no later than the day of issue",
                                                                Lang::Russian => "Дедлайн должен быть не позднее дня выдачи",
                                                            },
                                                        );
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
                                                                    Lang::English => {
                                                                        "There are no free books"
                                                                    }
                                                                    Lang::Russian => {
                                                                        "Свободных книг не осталось"
                                                                    }
                                                                },
                                                            );
                                                            caretaker.pop();
                                                            None
                                                        }

                                                        Some(sim) => {
                                                            if (**reader_base
                                                                .readers
                                                                .get_unchecked(rind))
                                                            .borrow()
                                                            .reading
                                                            .is_some()
                                                            {
                                                                alert(
                                                                    500,
                                                                    500,
                                                                    match lang {
                                                                        Lang::English => "This reader is already reading book",
                                                                        Lang::Russian => "Этот читатель уже читает книгу",
                                                                    },
                                                                );

                                                                caretaker.pop();
                                                                return None;
                                                            }

                                                            (*reader_base
                                                                .readers
                                                                .get_unchecked(rind))
                                                            .borrow_mut()
                                                            .start_reading(
                                                                (*book_system
                                                                    .books
                                                                    .get_unchecked(bind))
                                                                .borrow_mut()
                                                                .books
                                                                .get_unchecked(sim),
                                                            );

                                                            (*(*book_system
                                                                .books
                                                                .get_unchecked(bind))
                                                            .borrow_mut()
                                                            .books
                                                            .get_unchecked(sim))
                                                            .borrow_mut()
                                                            .start_reading(
                                                                reader_base
                                                                    .readers
                                                                    .get_unchecked(rind),
                                                                date,
                                                            );

                                                            fltk::dialog::message(
                                                                500,
                                                                500,
                                                                match lang {
                                                                    Lang::English => "Book successfully given to reader",
                                                                    Lang::Russian => "Книга успешно выдана читателю",
                                                                },
                                                            );

                                                            book_system.save();
                                                            reader_base.save();

                                                            Some(
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
                                                                .to_string(book_system),
                                                            )
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
                                                    caretaker.pop();
                                                    None
                                                }
                                            };
                                        }

                                        false => (),
                                    }
                                } else if !win.shown() {
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
    lang: Lang,
) -> Option<()> {
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
                .books
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
            Some(())
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

            None
        }
    };
}

/// **DEPRECATED**
///
/// Gets book from known reader (user input version)

#[deprecated]
fn get_book_known_reader_input(
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

                                    let cab = (*(*book_system.books.get_unchecked(bind))
                                        .borrow()
                                        .books
                                        .get_unchecked(sim))
                                    .borrow()
                                    .cabinet;

                                    let shelf = (*(*book_system.books.get_unchecked(bind))
                                        .borrow()
                                        .books
                                        .get_unchecked(sim))
                                    .borrow()
                                    .shelf;

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
                                    Some(())
                                }
                            };
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

/// Changes return date of already known book

#[inline]
pub fn change_return_date_simple(
    book_op: &Option<Weak<RefCell<Book>>>,
    book_system: &mut BookSystem,
    reader_base: &ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) -> Option<()> {
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

            caretaker.pop();
            None
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
                    match message {
                        true => {
                            inp.hide();

                            if let Ok(date) = inp.set_input() {
                                unsafe {
                                    return match date.get_unchecked(0).trim().parse::<u8>() {
                                        Ok(day) => {
                                            match date.get_unchecked(1).trim().parse::<u8>() {
                                                Ok(month) => {
                                                    match date
                                                        .get_unchecked(2)
                                                        .trim()
                                                        .parse::<u16>()
                                                    {
                                                        Ok(year) => {
                                                            match Date::new(day, month, year) {
                                                                Ok(new_date) => {
                                                                    let start = ((*(*book
                                                                        .upgrade()
                                                                        .unwrap())
                                                                    .borrow_mut()
                                                                    .readers
                                                                    .last_mut()
                                                                    .unwrap())
                                                                    .1)
                                                                        .0;

                                                                    if new_date >= start
                                                                        && new_date
                                                                            >= Date::from(
                                                                                Local::now(),
                                                                            )
                                                                    {
                                                                        ((*(*book
                                                                            .upgrade()
                                                                            .unwrap())
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
																			});

                                                                        book_system.save();
                                                                        Some(())
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

                                                                        caretaker.pop();
                                                                        None
                                                                    }
                                                                }

                                                                Err(_) => {
                                                                    alert(
                                                                        500,
                                                                        500,
                                                                        match lang {
                                                                            Lang::English => {
                                                                                "Incorrect date"
                                                                            }
                                                                            Lang::Russian => {
                                                                                "Некорректная дата"
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
                                                                        "Incorrect 'Year' input"
                                                                    }
                                                                    Lang::Russian => {
                                                                        "Некорректный ввод 'Года'"
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
                                                                "Incorrect 'Month' input"
                                                            }
                                                            Lang::Russian => {
                                                                "Некорректный ввод 'Месяца'"
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
                                                    Lang::English => "Incorrect 'Day' input",
                                                    Lang::Russian => "Некорректный ввод 'Дня'",
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
                } else if !inp.shown() {
                    caretaker.pop();
                    return None;
                }
            }

            None
        }
    };
}
