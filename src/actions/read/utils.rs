extern crate chrono;
extern crate fltk;
extern crate fltk_calendar;

use fltk::{app, app::App, dialog::alert, prelude::*};

use fltk_calendar::calendar::Calendar;

use crate::{
    books::{book::Book, book_sys::BookSystem, date::Date},
    reading::read_base::ReaderBase,
    restore::caretaker::Caretaker,
    Lang,
};

use chrono::Datelike;

/// Function that checks if input was empty

#[inline]
pub(crate) fn empty_inp_reader(inp: &Vec<String>, lang: Lang) -> bool {
    unsafe {
        return if inp.get_unchecked(0).is_empty() {
            alert(
                500,
                500,
                match lang {
                    Lang::English => "'Name' is empty",
                    Lang::Russian => "'Имя' пусто",
                },
            );
            true
        } else if inp.get_unchecked(1).is_empty() {
            alert(
                500,
                500,
                match lang {
                    Lang::English => "'2-nd Name' is empty",
                    Lang::Russian => "'Фамилия' пусто",
                },
            );
            true
        } else if inp.get_unchecked(2).is_empty() {
            alert(
                500,
                500,
                match lang {
                    Lang::English => "'Mid. Name' is empty",
                    Lang::Russian => "'Фамилия' пусто",
                },
            );
            true
        } else {
            false
        };
    }
}

/// Function that checks if input is correct.
/// Returns index of book, if it exists.
/// or calls alert and returns error

#[inline]
pub(crate) fn check_reader(
    reader_base: &ReaderBase,
    reader: &Vec<String>,
    app: &App,
    lang: Lang,
) -> Option<usize> {
    if empty_inp_reader(reader, lang) {
        return None;
    }

    let mut win = fltk::window::SingleWindow::new(800, 500, 200, 100, "Choose birth date");

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
            match msg {
                true => {
                    win.hide();

                    let cal = Calendar::default();
                    let date = cal.get_date();

                    return match date {
                        Some(date) => reader_base.find_reader(
                            unsafe { &reader.get_unchecked(0) },
                            unsafe { &reader.get_unchecked(1) },
                            unsafe { &reader.get_unchecked(2) },
                            Date::from(date),
                        ),

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

                false => (),
            }
        } else if !win.shown() {
            return None;
        }
    }

    None
}

/// Function that returns index of simple book.
/// Panics if book is not in vec of books.

#[inline]
pub(crate) fn get_book_ind(book_system: &BookSystem, book: *const Book) -> usize {
    if book.is_null() {
        panic!("nullptr in actions/read/get_book_ind");
    }

    unsafe {
        match book_system.find_book(
            &(*book).title().to_string(),
            &(*book).author().to_string(),
            (*book).pages(),
        ) {
            None => panic!("Index out of range"),
            Some(ind) => {
                (*(**book_system.books.get_unchecked(ind)).borrow().books)
                    .iter()
                    .position(|x| &*(**x).borrow() as *const Book == book)
                    .unwrap()
                    + 1
            }
        }
    }
}
