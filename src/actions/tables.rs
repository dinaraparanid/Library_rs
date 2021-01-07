extern crate fltk;
use crate::actions::read::get_book_ind;
use crate::book::{Book, BookSystem, TheBook};
use crate::reader::{Reader, ReaderBase};
use fltk::draw;
use fltk::prelude::*;
use fltk::table::Table;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

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
pub fn draw_data(txt: &str, x: i32, y: i32, w: i32, h: i32, selected: bool) {
    draw::push_clip(x, y, w, h);

    draw::set_draw_color(if selected {
        Color::from_u32(0xD3D3D3)
    } else {
        Color::White
    });

    draw::draw_rectf(x, y, w, h);
    draw::set_draw_color(Color::Gray0);
    draw::draw_text2(txt, x, y, w, h, Align::Center);
    draw::draw_rect(x, y, w, h);
    draw::pop_clip();
}

/// Function that returns String with reader's data.
/// If column is 0, it' ll return reader's params,
/// if column is 1, it' ll return books's params (or none)
/// if column is 2, it' ll return start date's params (or none)
/// if column is 2, it' ll return finish date's params (or none)

#[inline]
pub fn cell_reader(x: i32, y: i32, reader_base: &ReaderBase, book_system: &BookSystem) -> String {
    unsafe {
        return if y < reader_base.readers.len() as i32 {
            if x == 0 {
                format!(
                    "{} {} {}, {} years old",
                    RefCell::borrow(&(**reader_base.readers.get_unchecked(y as usize))).name,
                    RefCell::borrow(&(**reader_base.readers.get_unchecked(y as usize))).family,
                    RefCell::borrow(&(**reader_base.readers.get_unchecked(y as usize))).father,
                    RefCell::borrow(&(**reader_base.readers.get_unchecked(y as usize))).age
                )
            } else if RefCell::borrow(&(**reader_base.readers.get_unchecked(y as usize)))
                .reading
                .is_some()
            {
                match x {
                    1 => format!(
                        "'{}' {}, {} pages ({})",
                        (*RefCell::borrow(&(**reader_base.readers.get_unchecked(y as usize)))
                            .reading
                            .as_ref()
                            .unwrap()
                            .upgrade()
                            .unwrap())
                        .borrow()
                        .title,
                        (*RefCell::borrow(&(**reader_base.readers.get_unchecked(y as usize)))
                            .reading
                            .as_ref()
                            .unwrap()
                            .upgrade()
                            .unwrap())
                        .borrow()
                        .author,
                        (*RefCell::borrow(&(**reader_base.readers.get_unchecked(y as usize)))
                            .reading
                            .as_ref()
                            .unwrap()
                            .upgrade()
                            .unwrap())
                        .borrow()
                        .pages,
                        get_book_ind(
                            book_system,
                            (*RefCell::borrow(&(**reader_base.readers.get_unchecked(y as usize)))
                                .reading
                                .as_ref()
                                .unwrap()
                                .upgrade()
                                .unwrap())
                            .as_ptr()
                        )
                    ),

                    2 => format!(
                        "{}/{}/{}",
                        ((*RefCell::borrow(&(**reader_base.readers.get_unchecked(y as usize)))
                            .reading
                            .as_ref()
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
                        ((*RefCell::borrow(&(**reader_base.readers.get_unchecked(y as usize)))
                            .reading
                            .as_ref()
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
                        ((*RefCell::borrow(&(**reader_base.readers.get_unchecked(y as usize)))
                            .reading
                            .as_ref()
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

                    _ => format!(
                        "{}/{}/{}",
                        ((*RefCell::borrow(&(**reader_base.readers.get_unchecked(y as usize)))
                            .reading
                            .as_ref()
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
                        ((*RefCell::borrow(&(**reader_base.readers.get_unchecked(y as usize)))
                            .reading
                            .as_ref()
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
                        ((*RefCell::borrow(&(**reader_base.readers.get_unchecked(y as usize)))
                            .reading
                            .as_ref()
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
                    ),
                }
            } else {
                "None".to_string()
            }
        } else {
            "".to_string()
        };
    }
}

/// Function that returns String book's with data.
/// If column is 0, it' ll return book's title,
/// if column is 1, it' ll return books's author,
/// if column is 2, it' ll return book's amount of pages
/// if column is 2, it' ll return number of all books

#[inline]
pub fn cell_book(x: i32, y: i32, book_system: &'static BookSystem) -> String {
    return format!(
        "{}",
        if y < book_system.books.len() as i32 {
            unsafe {
                match x {
                    0 => RefCell::borrow(&(**book_system.books.get_unchecked(y as usize)))
                        .title
                        .clone(),
                    1 => RefCell::borrow(&(**book_system.books.get_unchecked(y as usize)))
                        .author
                        .clone(),
                    2 => RefCell::borrow(&(**book_system.books.get_unchecked(y as usize)))
                        .pages
                        .to_string(),
                    _ => RefCell::borrow(&(**book_system.books.get_unchecked(y as usize)))
                        .books
                        .iter()
                        .filter(|x| !(***x).borrow().is_using)
                        .count()
                        .to_string(),
                }
            }
        } else {
            "".to_string()
        }
    );
}

#[inline]
pub fn cell_book2(
    x: i32,
    y: i32,
    ind: usize,
    reader_base: &'static ReaderBase,
    book_system: &'static BookSystem,
) -> String {
    unsafe {
        return format!(
            "{}",
            if y < RefCell::borrow(&(**reader_base.readers.get_unchecked(ind)))
                .books
                .len() as i32
            {
                match x {
                    0 => (*RefCell::borrow(&(**reader_base.readers.get_unchecked(ind)))
                        .books
                        .get_unchecked(y as usize)
                        .upgrade()
                        .unwrap())
                    .borrow()
                    .title
                    .clone(),
                    1 => (*RefCell::borrow(&(**reader_base.readers.get_unchecked(ind)))
                        .books
                        .get_unchecked(y as usize)
                        .upgrade()
                        .unwrap())
                    .borrow()
                    .author
                    .clone(),
                    2 => (*RefCell::borrow(&(**reader_base.readers.get_unchecked(ind)))
                        .books
                        .get_unchecked(y as usize)
                        .upgrade()
                        .unwrap())
                    .borrow()
                    .author
                    .to_string(),
                    _ => get_book_ind(
                        book_system,
                        (*RefCell::borrow(&(**reader_base.readers.get_unchecked(ind)))
                            .books
                            .get_unchecked(y as usize)
                            .upgrade()
                            .unwrap())
                        .as_ptr(),
                    )
                    .to_string(),
                }
            } else {
                "".to_string()
            }
        );
    }
}
