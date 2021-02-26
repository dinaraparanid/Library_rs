extern crate fltk;

use crate::{
    actions::{
        book::{add_rem::*, change::*, utils::check_book},
        read::utils::get_book_ind,
        tables::*,
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
    frame::Frame,
    group::VGrid,
    input::{Input, IntInput},
    menu::{MenuBar, MenuFlag},
    prelude::*,
    table,
    table::Table,
    window::SingleWindow,
};

use std::{
    cell::RefCell,
    cmp::max,
    rc::{Rc, Weak},
};

/// Messages for info menu for The Book

#[derive(Clone, Copy)]
enum MessageTheBook {
    ChangeTitle,
    ChangeAuthor,
    ChangePages,
    RemoveThis,
    RemoveSimple,
    AddSimple,
}

/// Function that gives information
/// about already known simple book

pub fn book_info_simple(
    book: Option<Weak<RefCell<Book>>>,
    book_system: &BookSystem,
    app: &App,
    lang: Lang,
) {
    match book {
        None => return,
        Some(b) => {
            let mut wind = SingleWindow::new(
                800,
                100,
                848,
                600,
                format!(
                    "{} {} {}",
                    (*b.upgrade().unwrap()).borrow().title(),
                    (*b.upgrade().unwrap()).borrow().author(),
                    (*b.upgrade().unwrap()).borrow().pages(),
                )
                .as_str(),
            )
            .center_screen();

            let mut table1 = VGrid::new(0, 0, 908, 170, "");
            table1.set_params(6, 1, 1);

            table1.add(&Frame::new(
                10,
                50,
                100,
                30,
                format!(
                    "{}: {}",
                    match lang {
                        Lang::English => "Title",
                        Lang::Russian => "Название",
                    },
                    (*b.upgrade().unwrap()).borrow().title()
                )
                .as_str(),
            ));

            table1.add(&Frame::new(
                30,
                50,
                100,
                30,
                format!(
                    "{}: {}",
                    match lang {
                        Lang::English => "Author",
                        Lang::Russian => "Автор",
                    },
                    (*b.upgrade().unwrap()).borrow().author()
                )
                .as_str(),
            ));

            table1.add(&Frame::new(
                50,
                50,
                100,
                30,
                format!(
                    "{}: {}",
                    match lang {
                        Lang::English => "Amount of Pages",
                        Lang::Russian => "Количество страниц",
                    },
                    (*b.upgrade().unwrap()).borrow().pages(),
                )
                .as_str(),
            ));

            table1.add(&Frame::new(
                70,
                50,
                100,
                30,
                format!(
                    "{}: {}",
                    match lang {
                        Lang::English => "Order Number",
                        Lang::Russian => "Порядковый номер",
                    },
                    get_book_ind(book_system, b.upgrade().unwrap().as_ptr()),
                )
                .as_str(),
            ));

            table1.add(&Frame::new(
                90,
                50,
                100,
                30,
                format!(
                    "{}: {}",
                    match lang {
                        Lang::English => "Now is Read By",
                        Lang::Russian => "В данный момент читается",
                    },
                    if (*b.upgrade().unwrap()).borrow().is_using {
                        (*(*b.upgrade().unwrap())
                            .borrow()
                            .readers
                            .last()
                            .unwrap()
                            .0
                            .upgrade()
                            .unwrap())
                        .borrow()
                        .name
                        .clone()
                            + " "
                            + (*(*b.upgrade().unwrap())
                                .borrow()
                                .readers
                                .last()
                                .unwrap()
                                .0
                                .upgrade()
                                .unwrap())
                            .borrow()
                            .family
                            .as_str()
                            + " "
                            + (*(*b.upgrade().unwrap())
                                .borrow()
                                .readers
                                .last()
                                .unwrap()
                                .0
                                .upgrade()
                                .unwrap())
                            .borrow()
                            .father
                            .as_str()
                            + " "
                            + (*(*b.upgrade().unwrap())
                                .borrow()
                                .readers
                                .last()
                                .unwrap()
                                .0
                                .upgrade()
                                .unwrap())
                            .borrow()
                            .age()
                            .to_string()
                            .as_str()
                    } else {
                        match lang {
                            Lang::English => "None",
                            Lang::Russian => "Никем",
                        }
                        .to_string()
                    }
                )
                .as_str(),
            ));

            table1.add(&Frame::new(
                110,
                50,
                100,
                30,
                format!(
                    "{}: {}",
                    match lang {
                        Lang::English => "Cabinet",
                        Lang::Russian => "Шкаф",
                    },
                    (*b.upgrade().unwrap()).borrow().cabinet,
                )
                .as_str(),
            ));

            table1.add(&Frame::new(
                130,
                50,
                100,
                30,
                format!(
                    "{}: {}",
                    match lang {
                        Lang::English => "Shelf",
                        Lang::Russian => "Полка",
                    },
                    (*b.upgrade().unwrap()).borrow().shelf,
                )
                .as_str(),
            ));

            table1.add(&Frame::new(
                150,
                50,
                100,
                30,
                format!(
                    "{}:",
                    match lang {
                        Lang::English => "All Readers",
                        Lang::Russian => "Все читатели",
                    }
                )
                .as_str(),
            ));

            table1.auto_layout();

            let mut table2 = Table::new(0, 127, 848, 600, "");

            table2.set_rows(max(
                30,
                (*b.upgrade().unwrap()).borrow().readers.len() as u32,
            ));

            table2.set_row_header(true);
            table2.set_cols(6);
            table2.set_col_header(true);
            table2.set_col_width_all(130);
            table2.end();

            wind.end();
            wind.show();

            table2.draw_cell2(move |t, ctx, row, col, x, y, w, h| match ctx {
                fltk::table::TableContext::StartPage => draw::set_font(Font::Helvetica, 14),

                fltk::table::TableContext::ColHeader => draw_header(
                    &format!(
                        "{}",
                        match col {
                            0 => match lang {
                                Lang::English => "1-st Name",
                                Lang::Russian => "Имя",
                            },

                            1 => match lang {
                                Lang::English => "2-nd Name",
                                Lang::Russian => "Фамилия",
                            },

                            2 => match lang {
                                Lang::English => "Middle Name",
                                Lang::Russian => "Отчество",
                            },

                            3 => match lang {
                                Lang::English => "Age",
                                Lang::Russian => "Возраст",
                            },

                            4 => match lang {
                                Lang::English => "Start Date",
                                Lang::Russian => "Дата начала",
                            },

                            _ => match lang {
                                Lang::English => "Finish Date",
                                Lang::Russian => "Дедлайн",
                            },
                        }
                    ),
                    x,
                    y,
                    w,
                    h,
                ),

                fltk::table::TableContext::RowHeader => {
                    draw_header(&format!("{}", row + 1), x, y, w, h)
                }

                fltk::table::TableContext::Cell => draw_data(
                    &format!("{}", cell_reader2(col, row, b.clone(),)),
                    x,
                    y,
                    w,
                    h,
                    t.is_selected(row, col),
                    None,
                ),

                _ => (),
            });

            while app.wait() {
                if !wind.shown() {
                    return;
                }
            }
        }
    }
}

/// Function that returns info
/// of already known the book

pub fn the_book_info_simple(
    ind: usize,
    book_system: &mut BookSystem,
    reader_base: &mut ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) {
    let mut wind;

    unsafe {
        wind = SingleWindow::new(
            800,
            100,
            300,
            600,
            format!(
                "{} {}",
                (**book_system.books.get_unchecked(ind)).borrow().title,
                (**book_system.books.get_unchecked(ind)).borrow().author
            )
            .as_str(),
        );

        let mut table = VGrid::new(0, 30, 300, 180, "");
        table.set_params(5, 1, 1);

        table.add(&Frame::new(
            30,
            50,
            200,
            30,
            format!(
                "{}: {}",
                match lang {
                    Lang::English => "Title",
                    Lang::Russian => "Название",
                },
                (**book_system.books.get_unchecked(ind)).borrow().title,
            )
            .as_str(),
        ));

        table.add(&Frame::new(
            50,
            50,
            200,
            30,
            format!(
                "{}: {}",
                match lang {
                    Lang::English => "Author",
                    Lang::Russian => "Автор",
                },
                (**book_system.books.get_unchecked(ind)).borrow().author
            )
            .as_str(),
        ));

        table.add(&Frame::new(
            70,
            50,
            200,
            30,
            format!(
                "{}: {}",
                match lang {
                    Lang::English => "Amount of Pages",
                    Lang::Russian => "Количество страниц",
                },
                (**book_system.books.get_unchecked(ind)).borrow().pages
            )
            .as_str(),
        ));

        table.add(&Frame::new(
            90,
            50,
            100,
            30,
            format!(
                "{}: {}",
                match lang {
                    Lang::English => "Amount of books",
                    Lang::Russian => "Количество книг",
                },
                (**book_system.books.get_unchecked(ind))
                    .borrow()
                    .books
                    .len()
            )
            .as_str(),
        ));

        table.add(&Frame::new(
            90,
            50,
            100,
            30,
            format!(
                "{}:",
                match lang {
                    Lang::English => "Genres",
                    Lang::Russian => "Жанры",
                },
            )
            .as_str(),
        ));

        table.auto_layout();

        let mut genre_table = Table::new(0, 200, 280, 380, "");

        genre_table.set_rows(
            if let Some(g) = &(**book_system.books.get_unchecked(ind)).borrow().genres {
                max(20, g.len() as u32)
            } else {
                20
            },
        );

        genre_table.set_cols(1);
        genre_table.set_col_width_all(280);
        genre_table.end();

        let b = book_system.books.get_unchecked(ind).clone();

        genre_table.draw_cell2(move |t, ctx, row, col, x, y, w, h| match ctx {
            table::TableContext::StartPage => draw::set_font(Font::Helvetica, 14),

            table::TableContext::Cell => {
                let gen = cell_genre(row, &b, lang);
                draw_data(
                    &format!("{}", gen),
                    x,
                    y,
                    w,
                    h,
                    t.is_selected(row, col),
                    None,
                );
            }

            _ => (),
        });

        wind.end();
    }

    let mut menu = MenuBar::new(0, 0, 210, 30, "");
    wind.add(&menu);

    let (s, r) = app::channel();

    menu.add_emit(
        match lang {
            Lang::English => "&Change/Change title\t",
            Lang::Russian => "&Изменить/Изменить название\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageTheBook::ChangeTitle,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Change/Change author\t",
            Lang::Russian => "&Изменить/Изменить автора\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageTheBook::ChangeAuthor,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Change/Change amount of pages\t",
            Lang::Russian => "&Изменить/Изменить количество страниц\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageTheBook::ChangePages,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Remove/Remove all books\t",
            Lang::Russian => "&Удалить/Удалить все книги\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageTheBook::RemoveThis,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Remove/Remove one book\t",
            Lang::Russian => "&Удалить/Удалить одну книгу\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageTheBook::RemoveSimple,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Add books\t",
            Lang::Russian => "&Добавить книги\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageTheBook::AddSimple,
    );

    wind.show();

    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                MessageTheBook::ChangeAuthor => change_author_simple(
                    ind,
                    book_system,
                    reader_base,
                    genres,
                    caretaker,
                    app,
                    lang,
                ),

                MessageTheBook::ChangeTitle => {
                    change_title_simple(ind, book_system, reader_base, genres, caretaker, app, lang)
                }

                MessageTheBook::ChangePages => {
                    change_pages_simple(ind, book_system, reader_base, genres, caretaker, app, lang)
                }

                MessageTheBook::RemoveSimple => {
                    remove_book_simple(ind, book_system, reader_base, genres, caretaker, app, lang)
                }

                MessageTheBook::AddSimple => {
                    add_books_simple(ind, book_system, reader_base, genres, caretaker, app, lang)
                }

                MessageTheBook::RemoveThis => {
                    remove_the_book_simple(ind, book_system, reader_base, genres, caretaker, lang)
                }
            }
            return;
        } else if !wind.shown() {
            return;
        }
    }
}

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
pub fn show_all_books(book_system: Rc<RefCell<BookSystem>>, lang: Lang) {
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
            &format!("{}", cell_book(col, row, &*(*book_system).borrow())),
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
}
