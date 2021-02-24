extern crate fltk;

use crate::{
    actions::{book::book_info_simple, tables::*},
    books::{book::Book, book_sys::BookSystem, genres::Genres},
    change::{input1::Input1, input3::Input3, input4::Input4, Inputable},
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
    input::*,
    menu::{MenuBar, MenuFlag},
    prelude::*,
    table::Table,
    window::SingleWindow,
};

use crate::actions::giveaway::{get_book_known_reader, give_book_known_reader};
use std::{cell::RefCell, cmp::max, num::ParseIntError, rc::Rc};

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
        } else if inp.get_unchecked(2).is_empty() {
            alert(
                500,
                500,
                match lang {
                    Lang::English => "'Age' is empty",
                    Lang::Russian => "'Возраст' пусто",
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
    lang: Lang,
) -> Result<usize, ()> {
    let age;
    let ind;

    unsafe {
        if empty_inp_reader(reader, lang) {
            return Err(());
        }

        match reader.get_unchecked(3).trim().parse::<u8>() {
            Ok(x) => age = x,
            Err(_) => {
                alert(
                    500,
                    500,
                    match lang {
                        Lang::English => "'Age' input error",
                        Lang::Russian => "Ошибка ввода 'Возраста'",
                    },
                );
                return Err(());
            }
        }

        ind = reader_base.find_reader(
            reader.get_unchecked(0),
            reader.get_unchecked(1),
            reader.get_unchecked(2),
            age,
        );
    }

    return match ind {
        None => {
            alert(
                500,
                500,
                match lang {
                    Lang::English => "Reader isn't found",
                    Lang::Russian => "Читатель не найден",
                },
            );
            Err(())
        }

        Some(i) => Ok(i),
    };
}

/// Function that returns index of simple book.
/// Panics if book is not in vec of books.

#[inline]
pub(crate) fn get_book_ind(book_system: &BookSystem, book: *const Book) -> usize {
    if book.is_null() {
        panic!("nullptr in actions/read get_book_ind");
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

/// Removes already known reader

#[inline]
fn remove_reader_simple(
    ind: usize,
    reader_base: &mut ReaderBase,
    book_system: &mut BookSystem,
    genres: &Genres,
    caretaker: &mut Caretaker,
    lang: Lang,
) {
    caretaker.add_memento(reader_base, book_system, genres);

    match reader_base.remove_reader(ind) {
        Ok(_) => {
            fltk::dialog::message(
                500,
                500,
                match lang {
                    Lang::English => "Successfully removed",
                    Lang::Russian => "Успешно удалён",
                },
            );

            reader_base.save();
            book_system.save();
        }

        Err(_) => {
            alert(
                500,
                500,
                match lang {
                    Lang::English => "Reader is not found",
                    Lang::Russian => "Читатель не найден",
                },
            );
            caretaker.pop();
            return;
        }
    }
}

/// Change name of already known reader

#[inline]
fn change_name_simple(
    ind: usize,
    reader_base: &mut ReaderBase,
    book_system: &mut BookSystem,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) -> Option<String> {
    let (s3, r3) = app::channel();
    let mut get_name = Input1::<Input>::new(
        match lang {
            Lang::English => "New Name",
            Lang::Russian => "Новое Имя",
        },
        match lang {
            Lang::English => "New Name",
            Lang::Russian => "Новое Имя",
        },
    );

    caretaker.add_memento(reader_base, book_system, genres);

    get_name.show();
    (*get_name.ok).borrow_mut().emit(s3, true);

    while app.wait() {
        if let Some(mes) = r3.recv() {
            match mes {
                true => {
                    get_name.hide();

                    if let Ok(new_name) = get_name.set_input() {
                        unsafe {
                            return match reader_base
                                .change_name(ind, new_name.get_unchecked(0).clone())
                            {
                                Ok(_) => {
                                    fltk::dialog::message(
                                        500,
                                        500,
                                        match lang {
                                            Lang::English => "Successfully changed",
                                            Lang::Russian => "Успешно изменено",
                                        },
                                    );

                                    reader_base.save();
                                    book_system.save();
                                    Some(new_name.get_unchecked(0).clone())
                                }

                                Err(0) => {
                                    alert(
                                        500,
                                        500,
                                        match lang {
                                            Lang::English => "Reader is not found",
                                            Lang::Russian => "Читатель не найден",
                                        },
                                    );
                                    caretaker.pop();
                                    None
                                }

                                Err(1) => {
                                    alert(500, 500, match lang {
                                        Lang::English => "Reader with same parameters already exists",
                                        Lang::Russian => "Читатель с предложенными параметрами уже существует",
                                    });
                                    caretaker.pop();
                                    None
                                }

                                Err(_) => {
                                    alert(
                                        500,
                                        500,
                                        match lang {
                                            Lang::English => "'New name' is empty",
                                            Lang::Russian => "'Новое имя' пусто",
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
            break;
        } else if !get_name.shown() {
            caretaker.pop();
            return None;
        }
    }

    None
}

/// Change 2-nd name of already known reader

#[inline]
fn change_family_simple(
    ind: usize,
    reader_base: &mut ReaderBase,
    book_system: &mut BookSystem,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) -> Option<String> {
    let (s3, r3) = app::channel();
    let mut get_family = Input1::<Input>::new(
        match lang {
            Lang::English => "'New 2-nd Name",
            Lang::Russian => "Новая Фамилия",
        },
        match lang {
            Lang::English => "'New 2-nd Name",
            Lang::Russian => "Новая Фамилия",
        },
    );

    caretaker.add_memento(reader_base, book_system, genres);

    get_family.show();
    (*get_family.ok).borrow_mut().emit(s3, true);

    while app.wait() {
        if let Some(mes) = r3.recv() {
            match mes {
                true => {
                    get_family.hide();

                    if let Ok(new_family) = get_family.set_input() {
                        unsafe {
                            return match reader_base
                                .change_family(ind, new_family.get_unchecked(0).clone())
                            {
                                Ok(_) => {
                                    fltk::dialog::message(
                                        500,
                                        500,
                                        match lang {
                                            Lang::English => "Successfully changed",
                                            Lang::Russian => "Успешно изменено",
                                        },
                                    );

                                    reader_base.save();
                                    book_system.save();

                                    Some(new_family.get_unchecked(0).clone())
                                }

                                Err(0) => {
                                    alert(
                                        500,
                                        500,
                                        match lang {
                                            Lang::English => "Reader is not found",
                                            Lang::Russian => "Читатель не найден",
                                        },
                                    );
                                    caretaker.pop();
                                    None
                                }

                                Err(1) => {
                                    alert(
                                        500,
                                        500,
                                        match lang {
                                            Lang::English => "Reader with same parameters already exists",
                                            Lang::Russian => "Читатель с предложенными параметрами уже существует",
                                        },
                                    );
                                    caretaker.pop();
                                    None
                                }

                                Err(_) => {
                                    alert(
                                        500,
                                        500,
                                        match lang {
                                            Lang::English => "'New 2-nd Name' is empty",
                                            Lang::Russian => "'Новая Фамилия' пусто",
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
            break;
        } else if !get_family.shown() {
            caretaker.pop();
            return None;
        }
    }

    None
}

/// Change middle name of already known reader

#[inline]
fn change_father_simple(
    ind: usize,
    reader_base: &mut ReaderBase,
    book_system: &mut BookSystem,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) -> Option<String> {
    let (s3, r3) = app::channel();
    let mut get_father = Input1::<Input>::new(
        match lang {
            Lang::English => "New Middle Name",
            Lang::Russian => "Новое Отчество",
        },
        match lang {
            Lang::English => "New Middle Name",
            Lang::Russian => "Новое Отчество",
        },
    );

    caretaker.add_memento(reader_base, book_system, genres);

    get_father.show();
    (*get_father.ok).borrow_mut().emit(s3, true);

    while app.wait() {
        if let Some(mes) = r3.recv() {
            match mes {
                true => {
                    get_father.hide();

                    if let Ok(new_father) = get_father.set_input() {
                        unsafe {
                            return match reader_base
                                .change_father(ind, new_father.get_unchecked(0).clone())
                            {
                                Ok(_) => {
                                    fltk::dialog::message(
                                        500,
                                        500,
                                        match lang {
                                            Lang::English => "Successfully changed",
                                            Lang::Russian => "Успешно изменено",
                                        },
                                    );

                                    reader_base.save();
                                    book_system.save();
                                    Some(new_father.get_unchecked(0).clone())
                                }

                                Err(0) => {
                                    alert(
                                        500,
                                        500,
                                        match lang {
                                            Lang::English => "Reader is not found",
                                            Lang::Russian => "Читатель не найден",
                                        },
                                    );
                                    caretaker.pop();
                                    None
                                }

                                Err(1) => {
                                    alert(
                                        500,
                                        500,
                                        match lang {
                                            Lang::English => "Reader with same parameters already exists",
                                            Lang::Russian => "Читатель с предложенными параметрами уже существует",
                                        },
                                    );
                                    caretaker.pop();
                                    None
                                }

                                Err(_) => {
                                    alert(
                                        500,
                                        500,
                                        match lang {
                                            Lang::English => "'New Mid. Name' is empty",
                                            Lang::Russian => "'Новое Отчество' пусто",
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
            break;
        } else if !get_father.shown() {
            caretaker.pop();
            return None;
        }
    }

    None
}

/// Changes age of already known reader

#[inline]
fn change_age_simple(
    ind: usize,
    reader_base: &mut ReaderBase,
    book_system: &mut BookSystem,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) -> Option<String> {
    let (s3, r3) = app::channel();
    let mut get_age = Input1::<IntInput>::new(
        match lang {
            Lang::English => "New Age",
            Lang::Russian => "Новый возраст",
        },
        match lang {
            Lang::English => "New Age",
            Lang::Russian => "Новый возраст",
        },
    );

    caretaker.add_memento(reader_base, book_system, genres);

    get_age.show();
    (*get_age.ok).borrow_mut().emit(s3, true);

    while app.wait() {
        if let Some(mes) = r3.recv() {
            match mes {
                true => {
                    get_age.hide();

                    if let Ok(new_age) = get_age.set_input() {
                        if new_age.first().unwrap().is_empty() {
                            alert(
                                500,
                                500,
                                match lang {
                                    Lang::English => "'New Age' is empty",
                                    Lang::Russian => "'Новый возраст' пусто",
                                },
                            );
                            caretaker.pop();
                            return None;
                        }

                        unsafe {
                            return match reader_base
                                .change_age(ind, new_age.get_unchecked(0).clone())
                            {
                                Ok(_) => {
                                    fltk::dialog::message(
                                        500,
                                        500,
                                        match lang {
                                            Lang::English => "Successfully changed",
                                            Lang::Russian => "Успешно изменено",
                                        },
                                    );

                                    reader_base.save();
                                    book_system.save();
                                    Some(new_age.get_unchecked(0).clone())
                                }

                                Err(0) => {
                                    alert(
                                        500,
                                        500,
                                        match lang {
                                            Lang::English => "'New Age' input error",
                                            Lang::Russian => "Ошибка ввода 'Нового возраста'",
                                        },
                                    );
                                    caretaker.pop();
                                    None
                                }

                                Err(_) => {
                                    alert(
                                        500,
                                        500,
                                        match lang {
                                            Lang::English => "Reader with same parameters already exists",
                                            Lang::Russian => "Читатель с предложенными параметрами уже существует",
                                        }
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
            break;
        } else if !get_age.shown() {
            caretaker.pop();
            return None;
        }
    }

    None
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
                (*reader_base.readers.get_unchecked(ind)).borrow().age
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
                                new_age
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

/// Function that adds reader.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn add_reader(
    reader_base: &mut ReaderBase,
    book_system: &mut BookSystem,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) {
    let (s2, r2) = app::channel();

    let mut inp = Input4::<Input, Input, Input, IntInput>::new(
        match lang {
            Lang::English => "Add Reader",
            Lang::Russian => "Добавить читателя",
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
            Lang::English => "Age",
            Lang::Russian => "Возраст",
        },
    );

    caretaker.add_memento(reader_base, book_system, genres);

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    inp.hide();

                    if let Ok(reader) = inp.set_input() {
                        if empty_inp_reader(&reader, lang) {
                            caretaker.pop();
                            return;
                        }

                        match reader.last().unwrap().trim().parse::<u8>() {
                            Ok(a) => unsafe {
                                match reader_base.add_reader(
                                    reader.get_unchecked(0).clone(),
                                    reader.get_unchecked(1).clone(),
                                    reader.get_unchecked(2).clone(),
                                    a,
                                ) {
                                    Ok(_) => {
                                        fltk::dialog::message(
                                            500,
                                            500,
                                            match lang {
                                                Lang::English => "Successfully added",
                                                Lang::Russian => "Успешно добавлено",
                                            },
                                        );
                                        reader_base.save();
                                    }

                                    Err(_) => {
                                        alert(
                                            500,
                                            500,
                                            match lang {
                                                Lang::English => "Reader with same parameters already exists",
                                                Lang::Russian => "Читатель с предложенными парамтрами уже существует",
                                            }
                                        );
                                        caretaker.pop();
                                        return;
                                    }
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
                false => (),
            }
            break;
        } else if !inp.shown() {
            caretaker.pop();
            return;
        }
    }
}

/// Function that removes reader.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn remove_reader(
    reader_base: &mut ReaderBase,
    book_system: &mut BookSystem,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) {
    let (s2, r2) = app::channel();

    let mut inp = Input4::<Input, Input, Input, IntInput>::new(
        match lang {
            Lang::English => "Remove Reader",
            Lang::Russian => "Удалить читателя",
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
            Lang::English => "Age",
            Lang::Russian => "Возраст",
        },
    );

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    let rem_reader_params = inp.set_input();
                    inp.hide();

                    if let Ok(reader) = rem_reader_params {
                        let rind;

                        match check_reader(reader_base, &reader, lang) {
                            Ok(x) => rind = x,
                            Err(_) => return,
                        }

                        remove_reader_simple(
                            rind,
                            reader_base,
                            book_system,
                            genres,
                            caretaker,
                            lang,
                        );
                    }
                }
                false => (),
            }
            break;
        } else if !inp.shown() {
            break;
        }
    }
}

/// Function that changes reader's name.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn change_name(
    reader_base: &mut ReaderBase,
    book_system: &mut BookSystem,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) {
    let (s2, r2) = app::channel();

    let mut inp = Input4::<Input, Input, Input, IntInput>::new(
        match lang {
            Lang::English => "Change Name",
            Lang::Russian => "Изменить имя",
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
            Lang::English => "Age",
            Lang::Russian => "Возраст",
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
                        let rind;

                        match check_reader(reader_base, &reader, lang) {
                            Ok(x) => rind = x,
                            Err(_) => return,
                        }

                        change_name_simple(
                            rind,
                            reader_base,
                            book_system,
                            genres,
                            caretaker,
                            app,
                            lang,
                        );
                    }
                }
                false => (),
            }
            break;
        } else if !inp.shown() {
            break;
        }
    }
}

/// Function that changes reader's second name.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn change_family(
    reader_base: &mut ReaderBase,
    book_system: &mut BookSystem,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) {
    let (s2, r2) = app::channel();

    let mut inp = Input4::<Input, Input, Input, IntInput>::new(
        match lang {
            Lang::English => "Change 2-nd Name",
            Lang::Russian => "Изменить фамилию",
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
            Lang::English => "Age",
            Lang::Russian => "Возраст",
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
                        let rind;

                        match check_reader(reader_base, &reader, lang) {
                            Ok(x) => rind = x,
                            Err(_) => return,
                        }

                        change_family_simple(
                            rind,
                            reader_base,
                            book_system,
                            genres,
                            caretaker,
                            app,
                            lang,
                        );
                    }
                }
                false => (),
            }
            break;
        } else if !inp.shown() {
            break;
        }
    }
}

/// Function that changes reader's middle name.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn change_father(
    reader_base: &mut ReaderBase,
    book_system: &mut BookSystem,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) {
    let (s2, r2) = app::channel();

    let mut inp = Input4::<Input, Input, Input, IntInput>::new(
        match lang {
            Lang::English => "Change Middle Name",
            Lang::Russian => "Изменить Отчество",
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
            Lang::English => "Age",
            Lang::Russian => "Возраст",
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
                        let rind;

                        match check_reader(reader_base, &reader, lang) {
                            Ok(x) => rind = x,
                            Err(_) => return,
                        }

                        change_father_simple(
                            rind,
                            reader_base,
                            book_system,
                            genres,
                            caretaker,
                            app,
                            lang,
                        );
                    }
                }
                false => (),
            }
            break;
        } else if !inp.shown() {
            break;
        }
    }
}

/// Function that changes reader's age.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn change_age(
    reader_base: &mut ReaderBase,
    book_system: &mut BookSystem,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) {
    let (s2, r2) = app::channel();

    let mut inp = Input4::<Input, Input, Input, IntInput>::new(
        match lang {
            Lang::English => "Change Age",
            Lang::Russian => "Изменить Возраст",
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
            Lang::English => "Age",
            Lang::Russian => "Возраст",
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
                        let rind;

                        match check_reader(reader_base, &reader, lang) {
                            Ok(x) => rind = x,
                            Err(_) => return,
                        }

                        change_age_simple(
                            rind,
                            reader_base,
                            book_system,
                            genres,
                            caretaker,
                            app,
                            lang,
                        );
                    }
                }
                false => (),
            }
            break;
        } else if !inp.shown() {
            break;
        }
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
    let mut inp = Input4::<Input, Input, Input, IntInput>::new(
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
            Lang::English => "Age",
            Lang::Russian => "Возраст",
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
                        match reader.last().unwrap().trim().parse::<u8>() {
                            Ok(x) => unsafe {
                                let find = (*reader_base).borrow().find_reader(
                                    reader.get_unchecked(0),
                                    reader.get_unchecked(1),
                                    reader.get_unchecked(2),
                                    x,
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
