extern crate fltk;

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
    table::Table,
    window::SingleWindow,
};

use crate::{
    actions::{
        book::info::book_info_simple,
        giveaway::{get_book_known_reader, give_book_known_reader},
        read::{add_rem::remove_reader_simple, change::*},
        tables::*,
    },
    books::{book_sys::BookSystem, date::Date, genres::Genres},
    change::{input4::Input4, Inputable},
    reading::read_base::ReaderBase,
    restore::caretaker::Caretaker,
    Lang,
};

use std::{cell::RefCell, cmp::max, rc::Rc};

/// Messages for info menu

#[derive(Clone, Copy)]
enum MessageReader {
    ChangeName,
    ChangeFamily,
    ChangeFather,
    ChangeAge,
    GiveBook,
    GetBook,
    RemoveThis,
}

/// Function that gives information
/// about already known reader

pub fn reader_info_simple(
    ind: usize,
    reader_base: &mut ReaderBase,
    book_system: &mut BookSystem,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) {
    let mut wind;
    let mut table1;
    let mut table2;
    let mut name_frame;
    let mut family_frame;
    let mut father_frame;
    let mut age_frame;
    let mut reading_frame;

    unsafe {
        wind = SingleWindow::new(
            800,
            100,
            570,
            600,
            format!(
                "{} {} {}",
                (*reader_base.readers.get_unchecked(ind)).borrow().name,
                (*reader_base.readers.get_unchecked(ind)).borrow().family,
                (*reader_base.readers.get_unchecked(ind)).borrow().father,
            )
            .as_str(),
        )
        .center_screen();

        table1 = VGrid::new(0, 0, 650, 100, "");
        table1.set_params(6, 1, 1);

        name_frame = Frame::new(
            10,
            50,
            100,
            30,
            format!(
                "{}: {}",
                match lang {
                    Lang::English => "First Name",
                    Lang::Russian => "Имя",
                },
                (*reader_base.readers.get_unchecked(ind)).borrow().name
            )
            .as_str(),
        );

        family_frame = Frame::new(
            30,
            50,
            100,
            30,
            format!(
                "{}: {}",
                match lang {
                    Lang::English => "Second Name",
                    Lang::Russian => "Фамилия",
                },
                (*reader_base.readers.get_unchecked(ind)).borrow().family
            )
            .as_str(),
        );

        father_frame = Frame::new(
            50,
            50,
            100,
            30,
            format!(
                "{}: {}",
                match lang {
                    Lang::English => "Middle Name",
                    Lang::Russian => "Отчество",
                },
                (*reader_base.readers.get_unchecked(ind)).borrow().father
            )
            .as_str(),
        );

        age_frame = Frame::new(
            70,
            50,
            100,
            30,
            format!(
                "{}: {}",
                match lang {
                    Lang::English => "Age",
                    Lang::Russian => "Возраст",
                },
                (*reader_base.readers.get_unchecked(ind)).borrow().age()
            )
            .as_str(),
        );

        reading_frame = Frame::new(
            70,
            50,
            100,
            30,
            format!(
                "{}: {}",
                match lang {
                    Lang::English => "Reading now",
                    Lang::Russian => "Читается сейчас",
                },
                if (**reader_base.readers.get_unchecked(ind))
                    .borrow()
                    .reading
                    .is_some()
                {
                    (*(**reader_base.readers.get_unchecked(ind))
                        .borrow()
                        .reading
                        .as_ref()
                        .unwrap()
                        .upgrade()
                        .unwrap())
                    .borrow()
                    .to_string(book_system)
                } else {
                    match lang {
                        Lang::English => "None",
                        Lang::Russian => "Ничего",
                    }
                    .to_string()
                }
            )
            .as_str(),
        );

        table1.add(&name_frame);
        table1.add(&family_frame);
        table1.add(&father_frame);
        table1.add(&age_frame);
        table1.add(&reading_frame);

        table1.add(&Frame::new(
            90,
            50,
            100,
            30,
            format!(
                "{}:",
                match lang {
                    Lang::English => "Books read by reader",
                    Lang::Russian => "Прочитанные книги",
                }
            )
            .as_str(),
        ));

        table1.auto_layout();

        table2 = Table::new(0, 127, 570, 600, "");
        table2.set_rows(max(
            30,
            (**reader_base.readers.get_unchecked(ind))
                .borrow()
                .books
                .len() as u32,
        ));
        table2.set_row_header(true);
        table2.set_cols(4);
        table2.set_col_header(true);
        table2.set_col_width_all(130);
        table2.end();
    }

    wind.end();

    let mut menu = MenuBar::new(
        0,
        0,
        240 + match lang {
            Lang::English => 0,
            Lang::Russian => 40,
        },
        30,
        "",
    );
    wind.add(&menu);

    let (s, r) = app::channel();

    menu.add_emit(
        match lang {
            Lang::English => "&Change/Change name\t",
            Lang::Russian => "&Изменить/Изменить имя\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageReader::ChangeName,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Change/Change 2-nd name\t",
            Lang::Russian => "&Изменить/Изменить фамилию\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageReader::ChangeFamily,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Change/Change mid name\t",
            Lang::Russian => "&Изменить/Изменить отчество\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageReader::ChangeFather,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Change/Change age\t",
            Lang::Russian => "&Изменить/Изменить возраст\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageReader::ChangeAge,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Giveaway/Give book\t",
            Lang::Russian => "&Выдача/Выдать книгу\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageReader::GiveBook,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Giveaway/Get book\t",
            Lang::Russian => "&Выдача/Вернуть книгу\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageReader::GetBook,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Remove reader\t",
            Lang::Russian => "&Удалить читателя\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageReader::RemoveThis,
    );

    wind.show();

    let base_ptr = reader_base as *mut ReaderBase;
    let sys_ptr = book_system as *mut BookSystem;

    table2.draw_cell2(move |t, ctx, row, col, x, y, w, h| match ctx {
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
                        Lang::English => "Number of book",
                        Lang::Russian => "Номер книги",
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
            &format!(
                "{}",
                cell_book2(
                    col,
                    row,
                    ind,
                    unsafe { base_ptr.as_ref().unwrap() },
                    unsafe { sys_ptr.as_ref().unwrap() }
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

    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                MessageReader::ChangeName => {
                    if let Some(new_name) = change_name_simple(
                        ind,
                        reader_base,
                        book_system,
                        genres,
                        caretaker,
                        app,
                        lang,
                    ) {
                        name_frame.set_label(
                            format!(
                                "{}: {}",
                                match lang {
                                    Lang::English => "First Name",
                                    Lang::Russian => "Имя",
                                },
                                new_name
                            )
                            .as_str(),
                        );
                        name_frame.redraw();
                    }
                }

                MessageReader::ChangeFamily => {
                    if let Some(new_family) = change_family_simple(
                        ind,
                        reader_base,
                        book_system,
                        genres,
                        caretaker,
                        app,
                        lang,
                    ) {
                        family_frame.set_label(
                            format!(
                                "{}: {}",
                                match lang {
                                    Lang::English => "Second Name",
                                    Lang::Russian => "Фамилия",
                                },
                                new_family
                            )
                            .as_str(),
                        );
                        family_frame.redraw();
                    }
                }

                MessageReader::ChangeFather => {
                    if let Some(new_father) = change_father_simple(
                        ind,
                        reader_base,
                        book_system,
                        genres,
                        caretaker,
                        app,
                        lang,
                    ) {
                        father_frame.set_label(
                            format!(
                                "{}: {}",
                                match lang {
                                    Lang::English => "Middle Name",
                                    Lang::Russian => "Отчество",
                                },
                                new_father
                            )
                            .as_str(),
                        );
                        father_frame.redraw();
                    }
                }

                MessageReader::ChangeAge => {
                    if let Some(new_age) = change_age_simple(
                        ind,
                        reader_base,
                        book_system,
                        genres,
                        caretaker,
                        app,
                        lang,
                    ) {
                        age_frame.set_label(
                            format!(
                                "{}: {}",
                                match lang {
                                    Lang::English => "Age",
                                    Lang::Russian => "Возраст",
                                },
                                new_age,
                            )
                            .as_str(),
                        );
                        age_frame.redraw();
                    }
                }

                MessageReader::GiveBook => {
                    if let Some(book) = give_book_known_reader(
                        ind,
                        reader_base,
                        book_system,
                        genres,
                        caretaker,
                        app,
                        lang,
                    ) {
                        reading_frame.set_label(
                            format!(
                                "{}: {}",
                                match lang {
                                    Lang::English => "Reading now",
                                    Lang::Russian => "Читается сейчас",
                                },
                                book
                            )
                            .as_str(),
                        );
                        reading_frame.redraw();
                    }
                    table2.redraw();
                }

                MessageReader::GetBook => {
                    if let Some(_) = get_book_known_reader(
                        ind,
                        reader_base,
                        book_system,
                        genres,
                        caretaker,
                        app,
                        lang,
                    ) {
                        reading_frame.set_label(
                            format!(
                                "{}: {}",
                                match lang {
                                    Lang::English => "Reading now",
                                    Lang::Russian => "Читается сейчас",
                                },
                                match lang {
                                    Lang::English => "None",
                                    Lang::Russian => "Ничего",
                                },
                            )
                            .as_str(),
                        );
                        reading_frame.redraw();
                    }
                    table2.redraw();
                }

                MessageReader::RemoveThis => {
                    remove_reader_simple(ind, reader_base, book_system, genres, caretaker, lang);
                    return;
                }
            }
        }

        if !wind.shown() {
            return;
        }

        let len = unsafe {
            (**reader_base.readers.get_unchecked(ind))
                .borrow()
                .books
                .len()
        };

        (0..len).for_each(|i| {
            if table2.is_selected(i as i32, 0)
                || table2.is_selected(i as i32, 1)
                || table2.is_selected(i as i32, 2)
                || table2.is_selected(i as i32, 3)
            {
                unsafe {
                    book_info_simple(
                        Some(
                            (**reader_base.readers.get_unchecked(ind))
                                .borrow()
                                .books
                                .get_unchecked(i)
                                .clone(),
                        ),
                        book_system,
                        app,
                        lang,
                    );
                }

                table2.unset_selection();
                return;
            }
        });
    }
}

/// Function that gives info about reader.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn reader_info(
    reader_base: Rc<RefCell<ReaderBase>>,
    book_system: Rc<RefCell<BookSystem>>,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) {
    let (s2, r2) = app::channel();
    let mut inp = Input4::<Input, Input, Input, Input>::new(
        match lang {
            Lang::English => "Find Reader",
            Lang::Russian => "Поиск Читателя",
        },
        match lang {
            Lang::English => "First Name",
            Lang::Russian => "Имя",
        },
        match lang {
            Lang::English => "Second Names",
            Lang::Russian => "Фамилия",
        },
        match lang {
            Lang::English => "Middle Name",
            Lang::Russian => "Отчество",
        },
        match lang {
            Lang::English => "Birth Date (D/M/Y)",
            Lang::Russian => "Дата Рождения (Д/М/Г)",
        },
    );

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    inp.hide();

                    if let Ok(reader) = inp.set_input() {
                        let it = reader.last().unwrap().trim().split('/').collect::<Vec<_>>();

                        if it.len() != 3 {
                            alert(
                                500,
                                500,
                                match lang {
                                    Lang::English => "Incorrect 'Birth Date' input",
                                    Lang::Russian => "Некорректный ввод 'Даты Рождения'",
                                },
                            );
                            caretaker.pop();
                            return;
                        }

                        let mut it = it.into_iter();

                        unsafe {
                            match it.next().unwrap().trim().parse::<u8>() {
                                Ok(day) => match it.next().unwrap().trim().parse::<u8>() {
                                    Ok(month) => match it.next().unwrap().trim().parse::<u16>() {
                                        Ok(year) => match Date::new(day, month, year) {
                                            Ok(date) => {
                                                let find = (*reader_base).borrow().find_reader(
                                                    reader.get_unchecked(0),
                                                    reader.get_unchecked(1),
                                                    reader.get_unchecked(2),
                                                    date,
                                                );

                                                match find {
                                                    None => alert(
                                                        500,
                                                        500,
                                                        match lang {
                                                            Lang::English => "Reader isn't found",
                                                            Lang::Russian => "Читатель не найден",
                                                        },
                                                    ),

                                                    Some(ind) => reader_info_simple(
                                                        ind,
                                                        &mut *(*reader_base).borrow_mut(),
                                                        &mut *(*book_system).borrow_mut(),
                                                        genres,
                                                        caretaker,
                                                        app,
                                                        lang,
                                                    ),
                                                }
                                            }

                                            Err(_) => {
                                                alert(
                                                    500,
                                                    500,
                                                    match lang {
                                                        Lang::English => "Incorrect date",
                                                        Lang::Russian => "Некорректная дата",
                                                    },
                                                );
                                                caretaker.pop();
                                                return;
                                            }
                                        },

                                        Err(_) => {
                                            alert(
                                                500,
                                                500,
                                                match lang {
                                                    Lang::English => "'Age' input error",
                                                    Lang::Russian => "Ошибка ввода 'Возраста'",
                                                },
                                            );
                                            caretaker.pop();
                                            return;
                                        }
                                    },

                                    Err(_) => {
                                        alert(
                                            500,
                                            500,
                                            match lang {
                                                Lang::English => "'Age' input error",
                                                Lang::Russian => "Ошибка ввода 'Возраста'",
                                            },
                                        );
                                        caretaker.pop();
                                        return;
                                    }
                                },

                                Err(_) => {
                                    alert(
                                        500,
                                        500,
                                        match lang {
                                            Lang::English => "'Age' input error",
                                            Lang::Russian => "Ошибка ввода 'Возраста'",
                                        },
                                    );
                                    caretaker.pop();
                                    return;
                                }
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
