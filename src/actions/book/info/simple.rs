extern crate fltk;

use crate::{
    actions::{
        book::{add_rem::simple::*, change::simple::*},
        genres::customize_book_genre,
        read::utils::get_book_ind,
        tables::*,
    },
    books::{book::Book, book_sys::BookSystem, genres::Genres},
    change::{input1::Input1, Inputable},
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
    input::IntInput,
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

/// Messages for info_the_book
/// menu for The Book

#[derive(Clone, Copy)]
enum MessageTheBook {
    ChangeTitle,
    ChangeAuthor,
    ChangePages,
    CustomizeBookGenre,
    Info,
    RemoveThis,
    RemoveSimple,
    AddSimple,
}

/// Function that gives information
/// about already known simple book
/// by a smart pointer

pub fn book_info_simple(
    book: Option<Weak<RefCell<Book>>>,
    book_system: Rc<RefCell<BookSystem>>,
    reader_base: &ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) {
    if let Some(book) = book {
        let (t_ind, s_ind) = {
            let t_ind = (*book_system)
                .borrow()
                .find_book(
                    &(*book.upgrade().unwrap()).borrow().title(),
                    &(*book.upgrade().unwrap()).borrow().author(),
                    (*book.upgrade().unwrap()).borrow().pages(),
                )
                .unwrap();

            (t_ind, unsafe {
                (*book_system)
                    .borrow()
                    .books
                    .get_unchecked(t_ind)
                    .borrow()
                    .books
                    .iter()
                    .position(|b| b.as_ptr() == book.upgrade().unwrap().as_ptr())
                    .unwrap()
            })
        };

        let mut wind = SingleWindow::new(
            800,
            100,
            848,
            600,
            format!(
                "{} {} {}",
                *unsafe {
                    &(**(*book_system).borrow().books.get_unchecked(t_ind))
                        .borrow()
                        .title
                },
                *unsafe {
                    &(**(*book_system).borrow().books.get_unchecked(t_ind))
                        .borrow()
                        .author
                },
                *unsafe {
                    &(**(*book_system).borrow().books.get_unchecked(t_ind))
                        .borrow()
                        .pages
                },
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
                *unsafe {
                    &(**(*book_system).borrow().books.get_unchecked(t_ind))
                        .borrow()
                        .title
                }
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
                *unsafe {
                    &(**(*book_system).borrow().books.get_unchecked(t_ind))
                        .borrow()
                        .author
                }
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
                    Lang::Russian => "Кол-во страниц",
                },
                *unsafe {
                    &(**(*book_system).borrow().books.get_unchecked(t_ind))
                        .borrow()
                        .pages
                },
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
                get_book_ind(&*(*book_system).borrow(), unsafe {
                    (**(*book_system).borrow().books.get_unchecked(t_ind))
                        .borrow()
                        .books
                        .get_unchecked(s_ind)
                        .as_ptr()
                }),
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
                if unsafe {
                    (**(**(*book_system).borrow().books.get_unchecked(t_ind))
                        .borrow()
                        .books
                        .get_unchecked(s_ind))
                    .borrow()
                    .is_using
                } {
                    unsafe {
                        (**(**(*book_system).borrow().books.get_unchecked(t_ind))
                            .borrow()
                            .books
                            .get_unchecked(s_ind))
                        .borrow()
                        .readers
                        .last()
                        .unwrap()
                        .0
                        .upgrade()
                        .unwrap()
                        .borrow()
                        .name
                        .clone()
                            + " "
                            + (*(**(**(*book_system).borrow().books.get_unchecked(t_ind))
                                .borrow()
                                .books
                                .get_unchecked(s_ind))
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
                            + (*(**(**(*book_system).borrow().books.get_unchecked(t_ind))
                                .borrow()
                                .books
                                .get_unchecked(s_ind))
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
                            + (*(**(**(*book_system).borrow().books.get_unchecked(t_ind))
                                .borrow()
                                .books
                                .get_unchecked(s_ind))
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
                    }
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
                unsafe {
                    (**(**(*book_system).borrow().books.get_unchecked(t_ind))
                        .borrow()
                        .books
                        .get_unchecked(s_ind))
                    .borrow()
                    .cabinet
                },
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
                unsafe {
                    (**(**(*book_system).borrow().books.get_unchecked(t_ind))
                        .borrow()
                        .books
                        .get_unchecked(s_ind))
                    .borrow()
                    .shelf
                }
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

        table2.set_rows(max(30, unsafe {
            (**(**(*book_system).borrow().books.get_unchecked(t_ind))
                .borrow()
                .books
                .get_unchecked(s_ind))
            .borrow()
            .readers
            .len() as u32
        }));

        table2.set_row_header(true);
        table2.set_cols(6);
        table2.set_col_header(true);
        table2.set_col_width_all(130);
        table2.end();

        wind.end();
        wind.show();

        let bs = book_system.clone();

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
                            Lang::Russian => "Срок Сдачи",
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
                &format!(
                    "{}",
                    cell_reader2(
                        col,
                        row,
                        Rc::downgrade(&unsafe {
                            (*(**(*bs).borrow().books.get_unchecked(t_ind))
                                .borrow()
                                .books
                                .get_unchecked(s_ind))
                            .clone()
                        }),
                    )
                ),
                x,
                y,
                w,
                h,
                t.is_selected(row, col),
                None,
            ),

            _ => (),
        });

        let mut menu = MenuBar::new(
            0,
            0,
            130 + match lang {
                Lang::English => 0,
                Lang::Russian => 50,
            },
            30,
            "",
        );

        wind.add(&menu);

        let (s, r) = app::channel();

        menu.add_emit(
            match lang {
                Lang::English => "&Change location\t",
                Lang::Russian => "&Изменить расположение\t",
            },
            Shortcut::empty(),
            MenuFlag::Normal,
            s,
            true,
        );

        wind.show();

        while app.wait() {
            if let Some(msg) = r.recv() {
                if msg {
                    change_location_simple(
                        t_ind,
                        s_ind,
                        &mut *(*book_system).borrow_mut(),
                        reader_base,
                        genres,
                        caretaker,
                        app,
                        lang,
                    )
                }
            } else if !wind.shown() {
                return;
            }
        }
    }
}

/// Gets information about
/// simple book by index of TheBook

#[inline]
pub(crate) fn book_info_simple2(
    index: usize,
    book_system: Rc<RefCell<BookSystem>>,
    reader_base: &ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) {
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
            if msg {
                inp2.hide();

                if let Ok(bind_v) = inp2.set_input() {
                    let bind = bind_v.first().unwrap().trim().parse::<usize>().unwrap();

                    if bind
                        > unsafe {
                            (**(*book_system).borrow().books.get_unchecked(index))
                                .borrow()
                                .books
                                .len()
                        }
                        || bind == 0
                    {
                        alert(
                            500,
                            500,
                            match lang {
                                Lang::English => "Incorrect number of book",
                                Lang::Russian => "Некорректный номер книги",
                            },
                        );
                        return;
                    }

                    book_info_simple(
                        Some(Rc::downgrade(unsafe {
                            (**(*book_system).borrow().books.get_unchecked(index))
                                .borrow()
                                .books
                                .get_unchecked(bind - 1)
                        })),
                        book_system.clone(),
                        reader_base,
                        genres,
                        caretaker,
                        app,
                        lang,
                    );
                }
            }
            break;
        } else if !inp2.shown() {
            return;
        }
    }
}

/// Function that returns info
/// of already known the book

pub fn the_book_info_simple(
    ind: usize,
    book_system: Rc<RefCell<BookSystem>>,
    reader_base: &mut ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) {
    let mut wind = SingleWindow::new(
        800,
        100,
        520,
        600,
        format!(
            "{} {}",
            *unsafe {
                &(**(*book_system).borrow().books.get_unchecked(ind))
                    .borrow()
                    .title
            },
            *unsafe {
                &(**(*book_system).borrow().books.get_unchecked(ind))
                    .borrow()
                    .author
            }
        )
        .as_str(),
    );

    let mut table = VGrid::new(0, 30, 520, 180, "");
    table.set_params(5, 1, 1);

    let mut title_frame = Frame::new(
        30,
        50,
        420,
        30,
        format!(
            "{}: {}",
            match lang {
                Lang::English => "Title",
                Lang::Russian => "Название",
            },
            *unsafe {
                &(**(*book_system).borrow().books.get_unchecked(ind))
                    .borrow()
                    .title
            },
        )
        .as_str(),
    );

    let mut author_frame = Frame::new(
        50,
        50,
        420,
        30,
        format!(
            "{}: {}",
            match lang {
                Lang::English => "Author",
                Lang::Russian => "Автор",
            },
            *unsafe {
                &(**(*book_system).borrow().books.get_unchecked(ind))
                    .borrow()
                    .author
            }
        )
        .as_str(),
    );

    let mut pages_frame = Frame::new(
        70,
        50,
        420,
        30,
        format!(
            "{}: {}",
            match lang {
                Lang::English => "Amount of Pages",
                Lang::Russian => "Кол-во страниц",
            },
            *unsafe {
                &(**(*book_system).borrow().books.get_unchecked(ind))
                    .borrow()
                    .pages
            }
        )
        .as_str(),
    );

    let mut amount_frame = Frame::new(
        90,
        50,
        320,
        30,
        format!(
            "{}: {}",
            match lang {
                Lang::English => "Amount of books",
                Lang::Russian => "Кол-во книг",
            },
            unsafe {
                (**(*book_system).borrow().books.get_unchecked(ind))
                    .borrow()
                    .books
                    .len()
            }
        )
        .as_str(),
    );

    table.add(&title_frame);
    table.add(&author_frame);
    table.add(&pages_frame);
    table.add(&amount_frame);

    table.add(&Frame::new(
        90,
        50,
        320,
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

    let mut genre_table = Table::new(0, 200, 520, 380, "");

    genre_table.set_rows(
        if let Some(g) = unsafe {
            &(**(*book_system).borrow().books.get_unchecked(ind))
                .borrow()
                .genres
        } {
            max(20, g.len() as u32)
        } else {
            20
        },
    );

    genre_table.set_cols(1);
    genre_table.set_col_width_all(500);
    genre_table.end();

    let b = unsafe { (*book_system).borrow().books.get_unchecked(ind).clone() };

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

    let mut menu = MenuBar::new(
        0,
        0,
        420 + match lang {
            Lang::English => 0,
            Lang::Russian => 90,
        },
        30,
        "",
    );
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
            Lang::Russian => "&Изменить/Изменить Кол-во страниц\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageTheBook::ChangePages,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Change/Customize book genres\t",
            Lang::Russian => "&Изменить/Изменить жанры книги\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageTheBook::CustomizeBookGenre,
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
            Lang::English => "&Add books",
            Lang::Russian => "&Добавить книги",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageTheBook::AddSimple,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Get current book's information",
            Lang::Russian => "&Информация о конкретной книгe",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageTheBook::Info,
    );

    wind.show();

    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                MessageTheBook::ChangeTitle => {
                    if let Some(new_title) = change_title_simple(
                        ind,
                        &mut *(*book_system).borrow_mut(),
                        reader_base,
                        genres,
                        caretaker,
                        app,
                        lang,
                    ) {
                        title_frame.set_label(
                            format!(
                                "{}: {}",
                                match lang {
                                    Lang::English => "Title",
                                    Lang::Russian => "Название",
                                },
                                new_title
                            )
                            .as_str(),
                        );
                        title_frame.redraw();
                    }
                }

                MessageTheBook::ChangeAuthor => {
                    if let Some(new_author) = change_author_simple(
                        ind,
                        &mut *(*book_system).borrow_mut(),
                        reader_base,
                        genres,
                        caretaker,
                        app,
                        lang,
                    ) {
                        author_frame.set_label(
                            format!(
                                "{}: {}",
                                match lang {
                                    Lang::English => "Author",
                                    Lang::Russian => "Автор",
                                },
                                new_author
                            )
                            .as_str(),
                        );
                        author_frame.redraw();
                    }
                }

                MessageTheBook::ChangePages => {
                    if let Some(new_pages) = change_pages_simple(
                        ind,
                        &mut *(*book_system).borrow_mut(),
                        reader_base,
                        genres,
                        caretaker,
                        app,
                        lang,
                    ) {
                        pages_frame.set_label(
                            format!(
                                "{}: {}",
                                match lang {
                                    Lang::English => "Amount of pages",
                                    Lang::Russian => "Кол-во страниц",
                                },
                                new_pages
                            )
                            .as_str(),
                        );
                        pages_frame.redraw();
                    }
                }

                MessageTheBook::RemoveSimple => {
                    if remove_book_simple(
                        ind,
                        &mut *(*book_system).borrow_mut(),
                        reader_base,
                        genres,
                        caretaker,
                        app,
                        lang,
                    ) {
                        amount_frame.set_label(
                            format!(
                                "{}: {}",
                                match lang {
                                    Lang::English => "Amount of pages",
                                    Lang::Russian => "Кол-во страниц",
                                },
                                unsafe {
                                    (**(*book_system).borrow().books.get_unchecked(ind))
                                        .borrow()
                                        .books
                                        .len()
                                }
                            )
                            .as_str(),
                        );
                        amount_frame.redraw();
                    }
                }

                MessageTheBook::AddSimple => {
                    if add_books_simple(
                        ind,
                        &mut *(*book_system).borrow_mut(),
                        reader_base,
                        genres,
                        caretaker,
                        app,
                        lang,
                    ) {
                        amount_frame.set_label(
                            format!(
                                "{}: {}",
                                match lang {
                                    Lang::English => "Amount of books",
                                    Lang::Russian => "Кол-во книг",
                                },
                                unsafe {
                                    (**(*book_system).borrow().books.get_unchecked(ind))
                                        .borrow()
                                        .books
                                        .len()
                                }
                            )
                            .as_str(),
                        );
                        amount_frame.redraw();
                    }
                }

                MessageTheBook::RemoveThis => {
                    remove_the_book_simple(
                        ind,
                        &mut *(*book_system).borrow_mut(),
                        reader_base,
                        genres,
                        caretaker,
                        lang,
                    );
                    return;
                }

                MessageTheBook::CustomizeBookGenre => {
                    customize_book_genre(
                        genres,
                        &mut *(*book_system).borrow_mut(),
                        reader_base,
                        caretaker,
                        app,
                        lang,
                    );
                    genre_table.redraw();
                }

                MessageTheBook::Info => book_info_simple2(
                    ind,
                    book_system.clone(),
                    reader_base,
                    genres,
                    caretaker,
                    app,
                    lang,
                ),
            }
        } else if !wind.shown() {
            return;
        }
    }
}
