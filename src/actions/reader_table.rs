extern crate fltk;
use crate::reader::ReaderBase;
use fltk::draw;
use fltk::prelude::*;
use fltk::table::Table;

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

/// Function that returns String with data.
/// If column is 0, it' ll return reader's params,
/// if column is 1, it' ll return books's params (or none)
/// if column is 2, it' ll return start date's params (or none)
/// if column is 2, it' ll return finish date's params (or none)

#[inline]
pub fn cell(x: i32, y: i32, reader_base: &mut ReaderBase) -> String {
    unsafe {
        return if y < reader_base.readers.len() as i32 {
            if x == 0 {
                format!(
                    "{} {} {}, {} years old",
                    (*reader_base.readers.get_unchecked(y as usize))
                        .borrow()
                        .name,
                    (*reader_base.readers.get_unchecked(y as usize))
                        .borrow()
                        .family,
                    (*reader_base.readers.get_unchecked(y as usize))
                        .borrow()
                        .father,
                    (*reader_base.readers.get_unchecked(y as usize))
                        .borrow()
                        .age
                )
            } else if (*reader_base.readers.get_unchecked(y as usize))
                .borrow()
                .reading
                .is_some()
            {
                match x {
                    1 => format!(
                        "'{}' {}, {} pages",
                        (*(*reader_base.readers.get_unchecked(y as usize))
                            .borrow()
                            .reading
                            .as_ref()
                            .unwrap()
                            .upgrade()
                            .unwrap())
                        .borrow()
                        .title,
                        (*(*reader_base.readers.get_unchecked(y as usize))
                            .borrow()
                            .reading
                            .as_ref()
                            .unwrap()
                            .upgrade()
                            .unwrap())
                        .borrow()
                        .author,
                        (*(*reader_base.readers.get_unchecked(y as usize))
                            .borrow()
                            .reading
                            .as_ref()
                            .unwrap()
                            .upgrade()
                            .unwrap())
                        .borrow()
                        .pages,
                    ),

                    2 => format!(
                        "{}/{}/{}",
                        ((*(*reader_base.readers.get_unchecked(y as usize))
                            .borrow()
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
                        ((*(*reader_base.readers.get_unchecked(y as usize))
                            .borrow()
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
                        ((*(*reader_base.readers.get_unchecked(y as usize))
                            .borrow()
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
                        ((*(*reader_base.readers.get_unchecked(y as usize))
                            .borrow()
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
                        ((*(*reader_base.readers.get_unchecked(y as usize))
                            .borrow()
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
                        ((*(*reader_base.readers.get_unchecked(y as usize))
                            .borrow()
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
