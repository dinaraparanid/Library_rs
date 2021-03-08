extern crate chrono;
extern crate fltk;

use crate::{
    actions::read::utils::get_book_ind,
    books::{book::Book, book_sys::BookSystem, date::Date, genres::Genres, the_book::TheBook},
    reading::read_base::ReaderBase,
    Lang,
};

use chrono::Datelike;

use fltk::{draw, enums::Color, prelude::*};

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

/// Function that draws borders
/// of the table

#[inline]
pub fn draw_header(txt: &str, x: i32, y: i32, w: i32, h: i32) {
    draw::push_clip(x, y, w, h);
    draw::draw_box(FrameType::ThinUpBox, x, y, w, h, Color::FrameDefault);
    draw::set_draw_color(Color::Black);
    draw::draw_text2(txt, x, y, w, h, Align::Center);
    draw::pop_clip();
}

/// Function that draws cells in table

#[inline]
pub fn draw_data(
    txt: &str,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    selected: bool,
    color: Option<fltk::enums::Color>,
) {
    draw::push_clip(x, y, w, h);

    draw::set_draw_color(if color.is_some() {
        if selected {
            fltk::enums::Color::DarkRed
        } else {
            fltk::enums::Color::Red
        }
    } else if selected {
        fltk::enums::Color::from_u32(0xD3D3D3)
    } else {
        fltk::enums::Color::White
    });

    draw::draw_rectf(x, y, w, h);
    draw::set_draw_color(fltk::enums::Color::Gray0);
    draw::draw_text2(txt, x, y, w, h, Align::Center);
    draw::draw_rect(x, y, w, h);
    draw::pop_clip();
}

/// Function that returns String with reader's data and color.
/// If column is 0, it' ll return reader's params,
/// if column is 1, it' ll return books's params (or none)
/// if column is 2, it' ll return start date's params (or none)
/// if column is 2, it' ll return finish date's params (or none).
/// if reader is late, it' ll return some color,
/// else none

#[inline]
pub fn cell_reader(
    x: i32,
    y: i32,
    reader_base: &ReaderBase,
    book_system: &BookSystem,
    lang: Lang,
) -> (String, Option<fltk::enums::Color>) {
    return if y < reader_base.len() as i32 {
        let reader_date =
            match RefCell::borrow(unsafe { &(**reader_base.readers.get_unchecked(y as usize)) })
                .reading
            {
                None => Date::from(chrono::Local::now()),
                Some(_) => Date::new(
                    ((*RefCell::borrow(unsafe {
                        &(**reader_base.readers.get_unchecked(y as usize))
                    })
                    .reading
                    .as_ref()
                    .unwrap()
                    .first()
                    .unwrap()
                    .upgrade()
                    .unwrap())
                    .borrow()
                    .readers
                    .last()
                    .unwrap()
                    .1)
                        .1
                        .day,
                    ((*RefCell::borrow(unsafe {
                        &(**reader_base.readers.get_unchecked(y as usize))
                    })
                    .reading
                    .as_ref()
                    .unwrap()
                    .first()
                    .unwrap()
                    .upgrade()
                    .unwrap())
                    .borrow()
                    .readers
                    .last()
                    .unwrap()
                    .1)
                        .1
                        .month,
                    ((*RefCell::borrow(unsafe {
                        &(**reader_base.readers.get_unchecked(y as usize))
                    })
                    .reading
                    .as_ref()
                    .unwrap()
                    .first()
                    .unwrap()
                    .upgrade()
                    .unwrap())
                    .borrow()
                    .readers
                    .last()
                    .unwrap()
                    .1)
                        .1
                        .year,
                )
                .unwrap(),
            };

        let color = {
            let cur_date = Date::from(chrono::Local::now());

            if cur_date > reader_date {
                Some(Color::Red)
            } else {
                None
            }
        };

        if x == 0 {
            (
                format!(
                    "{} {} {}, ({})",
                    RefCell::borrow(unsafe { &(**reader_base.readers.get_unchecked(y as usize)) })
                        .name,
                    RefCell::borrow(unsafe { &(**reader_base.readers.get_unchecked(y as usize)) })
                        .family,
                    RefCell::borrow(unsafe { &(**reader_base.readers.get_unchecked(y as usize)) })
                        .father,
                    RefCell::borrow(unsafe { &(**reader_base.readers.get_unchecked(y as usize)) })
                        .birth
                ),
                color,
            )
        } else if RefCell::borrow(unsafe { &(**reader_base.readers.get_unchecked(y as usize)) })
            .reading
            .is_some()
        {
            match x {
                1 => (
                    format!(
                        "'{}' {}, {} {} ({})",
                        (*RefCell::borrow(unsafe {
                            &(**reader_base.readers.get_unchecked(y as usize))
                        })
                        .reading
                        .as_ref()
                        .unwrap()
                        .first()
                        .unwrap()
                        .upgrade()
                        .unwrap())
                        .borrow()
                        .title(),
                        (*RefCell::borrow(unsafe {
                            &(**reader_base.readers.get_unchecked(y as usize))
                        })
                        .reading
                        .as_ref()
                        .unwrap()
                        .first()
                        .unwrap()
                        .upgrade()
                        .unwrap())
                        .borrow()
                        .author(),
                        (*RefCell::borrow(unsafe {
                            &(**reader_base.readers.get_unchecked(y as usize))
                        })
                        .reading
                        .as_ref()
                        .unwrap()
                        .first()
                        .unwrap()
                        .upgrade()
                        .unwrap())
                        .borrow()
                        .pages(),
                        match lang {
                            Lang::English => "pages",
                            Lang::Russian => "страниц",
                        },
                        get_book_ind(
                            book_system,
                            (*RefCell::borrow(unsafe {
                                &(**reader_base.readers.get_unchecked(y as usize))
                            })
                            .reading
                            .as_ref()
                            .unwrap()
                            .first()
                            .unwrap()
                            .upgrade()
                            .unwrap())
                            .as_ptr()
                        )
                    ),
                    color,
                ),

                2 => (
                    format!(
                        "{}/{}/{}",
                        ((*RefCell::borrow(unsafe {
                            &(**reader_base.readers.get_unchecked(y as usize))
                        })
                        .reading
                        .as_ref()
                        .unwrap()
                        .first()
                        .unwrap()
                        .upgrade()
                        .unwrap())
                        .borrow()
                        .readers
                        .last()
                        .unwrap()
                        .1)
                            .0
                            .day,
                        ((*RefCell::borrow(unsafe {
                            &(**reader_base.readers.get_unchecked(y as usize))
                        })
                        .reading
                        .as_ref()
                        .unwrap()
                        .first()
                        .unwrap()
                        .upgrade()
                        .unwrap())
                        .borrow()
                        .readers
                        .last()
                        .unwrap()
                        .1)
                            .0
                            .month,
                        ((*RefCell::borrow(unsafe {
                            &(**reader_base.readers.get_unchecked(y as usize))
                        })
                        .reading
                        .as_ref()
                        .unwrap()
                        .first()
                        .unwrap()
                        .upgrade()
                        .unwrap())
                        .borrow()
                        .readers
                        .last()
                        .unwrap()
                        .1)
                            .0
                            .year,
                    ),
                    color,
                ),

                _ => (
                    format!(
                        "{}/{}/{}",
                        reader_date.day, reader_date.month, reader_date.year,
                    ),
                    color,
                ),
            }
        } else {
            (
                match lang {
                    Lang::English => "None",
                    Lang::Russian => "Ничего",
                }
                .to_string(),
                None,
            )
        }
    } else {
        ("".to_string(), None)
    };
}

/// Function that returns String with reader's data and color.
/// If column is 0, it' ll return reader's params,
/// if column is 1, it' ll return books's params (or none)
/// if column is 2, it' ll return start date's params (or none)
/// if column is 2, it' ll return finish date's params (or none).
/// if reader is late, it' ll return some color,
/// else none

#[inline]
pub fn cell_reader2(x: i32, y: i32, book: Weak<RefCell<Book>>) -> String {
    return if y < (*book.upgrade().unwrap()).borrow().readers.len() as i32 {
        match x {
            0 => unsafe {
                (*(*book.upgrade().unwrap())
                    .borrow()
                    .readers
                    .get_unchecked(y as usize)
                    .0
                    .upgrade()
                    .unwrap())
                .borrow()
                .name
                .clone()
            },

            1 => unsafe {
                (*(*book.upgrade().unwrap())
                    .borrow()
                    .readers
                    .get_unchecked(y as usize)
                    .0
                    .upgrade()
                    .unwrap())
                .borrow()
                .family
                .clone()
            },

            2 => unsafe {
                (*(*book.upgrade().unwrap())
                    .borrow()
                    .readers
                    .get_unchecked(y as usize)
                    .0
                    .upgrade()
                    .unwrap())
                .borrow()
                .father
                .clone()
            },

            3 => unsafe {
                (*(*book.upgrade().unwrap())
                    .borrow()
                    .readers
                    .get_unchecked(y as usize)
                    .0
                    .upgrade()
                    .unwrap())
                .borrow()
                .age()
                .to_string()
            },

            4 => unsafe {
                ((*book.upgrade().unwrap())
                    .borrow()
                    .readers
                    .get_unchecked(y as usize)
                    .1)
                    .0
                    .to_string()
            },

            _ => unsafe {
                ((*book.upgrade().unwrap())
                    .borrow()
                    .readers
                    .get_unchecked(y as usize)
                    .1)
                    .1
                    .to_string()
            },
        }
    } else {
        "".to_string()
    };
}

/// Function that returns String book's with data.
/// If column is 0, it' ll return book's title,
/// if column is 1, it' ll return books's author,
/// if column is 2, it' ll return book's amount of pages
/// if column is 2, it' ll return number of all books

#[inline]
pub fn cell_book(x: i32, y: i32, book_system: &BookSystem) -> String {
    return format!(
        "{}",
        if y < book_system.len() as i32 {
            match x {
                0 => RefCell::borrow(unsafe { &(**book_system.books.get_unchecked(y as usize)) })
                    .title
                    .clone(),
                1 => RefCell::borrow(unsafe { &(**book_system.books.get_unchecked(y as usize)) })
                    .author
                    .clone(),
                2 => RefCell::borrow(unsafe { &(**book_system.books.get_unchecked(y as usize)) })
                    .pages
                    .to_string(),
                _ => RefCell::borrow(unsafe { &(**book_system.books.get_unchecked(y as usize)) })
                    .books
                    .iter()
                    .filter(|x| !(***x).borrow().is_using)
                    .count()
                    .to_string(),
            }
        } else {
            "".to_string()
        }
    );
}

/// Function that returns
/// name, 2-nd name, mid name and age
/// of reader with known index

#[inline]
pub fn cell_book2(
    x: i32,
    y: i32,
    ind: usize,
    reader_base: &ReaderBase,
    book_system: &BookSystem,
) -> String {
    return format!(
        "{}",
        if RefCell::borrow(unsafe { &(**reader_base.readers.get_unchecked(ind)) })
            .reading
            .is_some()
            && y < RefCell::borrow(unsafe { &(**reader_base.readers.get_unchecked(ind)) })
                .reading
                .as_ref()
                .unwrap()
                .len() as i32
        {
            match x {
                0 => unsafe {
                    (*RefCell::borrow(&(**reader_base.readers.get_unchecked(ind)))
                        .reading
                        .as_ref()
                        .unwrap()
                        .get_unchecked(y as usize)
                        .upgrade()
                        .unwrap())
                    .borrow()
                    .title()
                    .to_string()
                },
                1 => unsafe {
                    (*RefCell::borrow(&(**reader_base.readers.get_unchecked(ind)))
                        .reading
                        .as_ref()
                        .unwrap()
                        .get_unchecked(y as usize)
                        .upgrade()
                        .unwrap())
                    .borrow()
                    .author()
                    .to_string()
                },
                2 => unsafe {
                    (*RefCell::borrow(&(**reader_base.readers.get_unchecked(ind)))
                        .reading
                        .as_ref()
                        .unwrap()
                        .get_unchecked(y as usize)
                        .upgrade()
                        .unwrap())
                    .borrow()
                    .pages()
                    .to_string()
                },
                _ => unsafe {
                    get_book_ind(
                        book_system,
                        (*RefCell::borrow(&(**reader_base.readers.get_unchecked(ind)))
                            .reading
                            .as_ref()
                            .unwrap()
                            .get_unchecked(y as usize)
                            .upgrade()
                            .unwrap())
                        .as_ptr(),
                    )
                    .to_string()
                },
            }
        } else {
            "".to_string()
        }
    );
}

/// **DEPRECATED**
///
/// Shows all read books
///
/// Function that returns
/// name, 2-nd name, mid name and age
/// of reader with known index

#[deprecated(note = "Shows all read books")]
pub fn cell_book2_old(
    x: i32,
    y: i32,
    ind: usize,
    reader_base: &ReaderBase,
    book_system: &BookSystem,
) -> String {
    return format!(
        "{}",
        if y < RefCell::borrow(unsafe { &(**reader_base.readers.get_unchecked(ind)) })
            .books
            .len() as i32
        {
            match x {
                0 => unsafe {
                    (*RefCell::borrow(&(**reader_base.readers.get_unchecked(ind)))
                        .books
                        .get_unchecked(y as usize)
                        .upgrade()
                        .unwrap())
                    .borrow()
                    .title()
                    .to_string()
                },
                1 => unsafe {
                    (*RefCell::borrow(&(**reader_base.readers.get_unchecked(ind)))
                        .books
                        .get_unchecked(y as usize)
                        .upgrade()
                        .unwrap())
                    .borrow()
                    .author()
                    .to_string()
                },
                2 => unsafe {
                    (*RefCell::borrow(&(**reader_base.readers.get_unchecked(ind)))
                        .books
                        .get_unchecked(y as usize)
                        .upgrade()
                        .unwrap())
                    .borrow()
                    .pages()
                    .to_string()
                },
                _ => unsafe {
                    get_book_ind(
                        book_system,
                        (*RefCell::borrow(&(**reader_base.readers.get_unchecked(ind)))
                            .books
                            .get_unchecked(y as usize)
                            .upgrade()
                            .unwrap())
                        .as_ptr(),
                    )
                    .to_string()
                },
            }
        } else {
            "".to_string()
        }
    );
}

/// Function that returns String book's with spec genre.

#[inline]
pub fn cell_book3(y: i32, books: &Vec<(String, String, u16)>, lang: Lang) -> String {
    return format!(
        "{}",
        {
            if books.is_empty() {
                if y == 0 {
                    match lang {
                        Lang::English => "None",
                        Lang::Russian => "Ничего",
                    }
                } else {
                    ""
                }
            } else {
                if y < books.len() as i32 {
                    unsafe {
                        return format!(
                            "{} {} {}",
                            books.get_unchecked(y as usize).0,
                            books.get_unchecked(y as usize).1,
                            books.get_unchecked(y as usize).2
                        );
                    }
                } else {
                    ""
                }
            }
        }
        .to_string()
    );
}

/// Function that returns date and time as string.

#[inline]
pub fn cell_date_time(x: i32) -> String {
    return format!(
        "{}",
        match x {
            0 => chrono::Local::now().day().to_string(),
            1 => chrono::Local::now().month().to_string(),
            _ => chrono::Local::now().year().to_string(),
        }
    );
}

/// Function that returns book's genre as string.

#[inline]
pub(crate) fn cell_genre(x: i32, book: &Rc<RefCell<TheBook>>, lang: Lang) -> String {
    return {
        format!(
            "{}",
            if x == 0 {
                if let Some(g) = unsafe { &(*(*book).as_ptr()).genres } {
                    g.iter().next().unwrap().as_str()
                } else {
                    match lang {
                        Lang::English => "None",
                        Lang::Russian => "Ничего",
                    }
                }
            } else {
                if let Some(g) = unsafe { &(*(*book).as_ptr()).genres } {
                    if (x as usize) < g.len() {
                        g.iter().skip(x as usize).next().unwrap().as_str()
                    } else {
                        ""
                    }
                } else {
                    ""
                }
            }
        )
    };
}

/// Function that returns genre as string.

#[deprecated]
#[allow(dead_code)]
fn cell_genre2(y: i32, genres: &Genres) -> String {
    return format!(
        "{}",
        if y < genres.len() as i32 {
            genres
                .genres
                .iter()
                .skip(y as usize)
                .next()
                .unwrap()
                .as_str()
        } else {
            ""
        }
    );
}
