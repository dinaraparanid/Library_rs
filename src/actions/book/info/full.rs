extern crate fltk;

use crate::{
    actions::{
        book::{info::simple::*, utils::check_book},
        tables::{cell_book, draw_data, draw_header},
    },
    books::{book::Book, book_sys::BookSystem, genres::Genres},
    change::{input1::Input1, input3::Input3, Inputable},
    reading::read_base::ReaderBase,
    restore::caretaker::Caretaker,
    Lang,
};

use fltk::{
    app,
    app::App,
    dialog::alert,
    draw,
    input::{Input, IntInput},
    prelude::*,
    table,
    table::Table,
    tree::Tree,
    window::SingleWindow,
};

use std::{cell::RefCell, cmp::max, collections::BTreeMap, rc::Rc};

/// Function that gives all information about TheBook:
/// title, author, pages, amount of simple books.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn the_book_info(
    book_system: Rc<RefCell<BookSystem>>,
    reader_base: Rc<RefCell<ReaderBase>>,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) {
    let (s2, r2) = app::channel();
    let mut inp = Input3::<Input, Input, IntInput>::new(
        match lang {
            Lang::English => "Find book",
            Lang::Russian => "Найти книгу",
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

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    inp.hide();

                    if let Ok(the_book) = inp.set_input() {
                        let index;

                        match check_book(&(*book_system).borrow(), &the_book, lang) {
                            Ok(x) => index = x,
                            Err(_) => return,
                        }

                        the_book_info_simple(
                            index,
                            &mut *(*book_system).borrow_mut(),
                            &mut *(*reader_base).borrow_mut(),
                            genres,
                            caretaker,
                            app,
                            lang,
                        )
                    }
                }
                false => (),
            }
            break;
        } else if !inp.shown() {
            return;
        }
    }
}

/// Function that gives all information about simple Book:
/// title, author, pages, and readers (+ current reader).
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn book_info(book_system: &BookSystem, app: &App, lang: Lang) {
    let (s2, r2) = app::channel();
    let mut inp = Input3::<Input, Input, IntInput>::new(
        match lang {
            Lang::English => "Find book",
            Lang::Russian => "Найти книгу",
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

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    inp.hide();

                    if let Ok(the_book) = inp.set_input() {
                        let index;

                        match check_book(book_system, &the_book, lang) {
                            Ok(x) => index = x,
                            Err(_) => return,
                        }

                        let (s, r) = app::channel();
                        let mut inp2 = Input1::<IntInput>::new(
                            match lang {
                                Lang::English => "Number of Book",
                                Lang::Russian => "Номер книги",
                            },
                            match lang {
                                Lang::English => "Number of Book",
                                Lang::Russian => "Номер книги",
                            },
                        );

                        inp2.show();
                        (*inp2.ok).borrow_mut().emit(s, true);

                        while app.wait() {
                            if let Some(msg) = r.recv() {
                                match msg {
                                    true => {
                                        inp2.hide();

                                        if let Ok(bind_v) = inp2.set_input() {
                                            let bind = bind_v
                                                .first()
                                                .unwrap()
                                                .trim()
                                                .parse::<usize>()
                                                .unwrap();

                                            unsafe {
                                                if bind
                                                    > (**book_system.books.get_unchecked(index))
                                                        .borrow()
                                                        .books
                                                        .len()
                                                    || bind == 0
                                                {
                                                    alert(
                                                        500,
                                                        500,
                                                        match lang {
                                                            Lang::English => {
                                                                "Incorrect number of book"
                                                            }
                                                            Lang::Russian => {
                                                                "Некорректный номер книги"
                                                            }
                                                        },
                                                    );
                                                    return;
                                                }

                                                book_info_simple(
                                                    Some(Rc::downgrade(
                                                        (**book_system.books.get_unchecked(index))
                                                            .borrow()
                                                            .books
                                                            .get_unchecked(bind - 1),
                                                    )),
                                                    book_system,
                                                    app,
                                                    lang,
                                                );
                                            }
                                        }
                                    }

                                    false => (),
                                }
                            } else if !inp2.shown() {
                                return;
                            }
                        }
                    }
                }
                false => (),
            }
            break;
        } else if !inp.shown() {
            return;
        }
    }
}

/// Function that shows all information about all existing books:
/// title, author, num of pages and num of available simple books

#[inline]
pub fn show_all_books(
    book_system: Rc<RefCell<BookSystem>>,
    reader_base: &mut ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) {
    let mut wind = SingleWindow::default()
        .with_label(match lang {
            Lang::English => "All Books",
            Lang::Russian => "Все книги",
        })
        .with_size(820, 550)
        .center_screen();

    let mut table = Table::new(10, 10, 800, 540, "");
    table.set_rows(max(20, (*book_system).borrow().books.len() as u32));
    table.set_row_header(true);
    table.set_cols(4);
    table.set_col_header(true);
    table.set_col_width_all(190);
    table.end();

    let bs = book_system.clone();

    table.draw_cell2(move |t, ctx, row, col, x, y, w, h| match ctx {
        fltk::table::TableContext::StartPage => draw::set_font(Font::Helvetica, 14),

        fltk::table::TableContext::ColHeader => draw_header(
            &format!(
                "{}",
                match col {
                    0 => match lang {
                        Lang::English => "Title",
                        Lang::Russian => "Название",
                    },

                    1 => match lang {
                        Lang::English => "Author",
                        Lang::Russian => "Автор",
                    },

                    2 => match lang {
                        Lang::English => "Amount of Pages",
                        Lang::Russian => "Количество страниц",
                    },

                    _ => match lang {
                        Lang::English => "Amount of available books",
                        Lang::Russian => "Количество доступных книг",
                    },
                }
            ),
            x,
            y,
            w,
            h,
        ),

        fltk::table::TableContext::RowHeader => draw_header(&format!("{}", row + 1), x, y, w, h),

        fltk::table::TableContext::Cell => draw_data(
            &format!("{}", cell_book(col, row, &*(*bs).borrow())),
            x,
            y,
            w,
            h,
            t.is_selected(row, col),
            None,
        ),

        _ => (),
    });

    wind.end();
    wind.show();

    while app.wait() {
        let len = (*book_system).borrow().books.len();

        (0..len).for_each(|i| {
            if table.is_selected(i as i32, 0)
                || table.is_selected(i as i32, 1)
                || table.is_selected(i as i32, 2)
                || table.is_selected(i as i32, 3)
            {
                the_book_info_simple(
                    i,
                    &mut (*book_system).borrow_mut(),
                    reader_base,
                    genres,
                    caretaker,
                    app,
                    lang,
                );

                table.unset_selection();
                return;
            }
        })
    }
}

#[inline]
pub fn show_all_authors(book_system: &BookSystem, lang: Lang) {
    let mut wind = SingleWindow::new(
        500,
        500,
        300,
        400,
        match lang {
            Lang::English => "All Books with Genres",
            Lang::Russian => "Все Книги с Жанрами",
        },
    );

    let mut tree = Tree::new(0, 0, 300, 400, "");
    tree.set_root_label(match lang {
        Lang::English => "Authors",
        Lang::Russian => "Авторы",
    });

    let mut authors = BTreeMap::new();

    book_system.iter().for_each(|b| {
        authors
            .entry((**b).borrow().author.clone())
            .or_insert(vec![])
            .push(format!(
                "{}, {} {}",
                (**b).borrow().title,
                (**b).borrow().pages,
                match lang {
                    Lang::English => "pages",
                    Lang::Russian => "страниц",
                }
            ));
    });

    authors.into_iter().for_each(|p| {
        let author = p.0;
        tree.add(author.as_str()).unwrap();

        p.1.into_iter().for_each(|b| {
            tree.add(format!("{}/{}", author, b).as_str()).unwrap();
        });
    });

    wind.end();
    wind.show();
}
