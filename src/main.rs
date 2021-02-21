extern crate fltk;

use booklibrs::{
    actions::{book::*, genres::*, giveaway::*, read::*, tables::*},
    books::{book_sys::BookSystem, genres::Genres},
    change::{input2::Input2, Inputable},
    reading::read_base::ReaderBase,
    restore::caretaker::Caretaker,
    Lang,
};

use fltk::{
    app,
    button::*,
    dialog::alert,
    draw,
    enums::Shortcut,
    frame::Frame,
    image::*,
    input::{Input, SecretInput},
    menu::*,
    table,
    table::Table,
    window::*,
};

use std::{
    cell::RefCell,
    cmp::max,
    error::Error,
    fs::File,
    io::{Read, Write},
    rc::Rc,
};

/// All messages, which used to call functions

#[derive(Clone, Copy)]
enum Message {
    AddReader,
    RemoveReader,
    ChangeName,
    ChangeFamily,
    ChangeFather,
    ChangeAge,
    InfoReader,
    AddBooks,
    RemoveBook,
    RemoveTheBook,
    ChangeTitle,
    ChangeAuthor,
    ChangePages,
    InfoTheBook,
    InfoBook,
    GiveBook,
    GetBook,
    ShowAllBooks,
    ShowGenres,
    AddGenre,
    RemoveGenre,
    CustomizeBookGenre,
    FindByGenre,
    PrevData,
    NextData,
    English,
    Russian,
}

/// Hashing login and password

#[inline]
fn get_hash(str: &String, p: u128, module: u128) -> Vec<u128> {
    let mut ans = vec![0; str.len()];
    let bytes = str.as_bytes();

    unsafe {
        *ans.get_unchecked_mut(0) = *bytes.get_unchecked(0) as u128;

        (1..str.len()).for_each(|i| {
            *ans.get_unchecked_mut(i) = ((ans.get_unchecked(i - 1).overflowing_mul(p))
                .0
                .overflowing_add(*bytes.get_unchecked(i) as u128)
                .0)
                % module;
        });
    }

    ans
}

fn main() -> Result<(), Box<dyn Error>> {
    let reader_base = Rc::new(RefCell::new(ReaderBase::new()));
    let book_system = Rc::new(RefCell::new(BookSystem::new()));
    let genres = Rc::new(RefCell::new(Genres::new()));
    let lang = Lang::new();

    let app = app::App::default().with_scheme(fltk::app::AppScheme::Plastic);
    let (s, r) = app::channel();

    (*reader_base).borrow_mut().load();
    (*book_system)
        .borrow_mut()
        .load(&mut (*reader_base).borrow_mut());

    (*genres).borrow_mut().load();

    let caretaker = Rc::new(RefCell::new(Caretaker::new()));

    let mut admin = File::open("src/utils/admin.bin")?;
    let mut adm = String::new();
    admin.read_to_string(&mut adm)?;

    #[allow(unused_assignments)]
    let mut success = 0; // 0 - no input / 1 - ok / 2 - mistake

    if adm.is_empty() {
        let (s, r) = app::channel();

        loop {
            success = 0;
            let mut password =
                Input2::<Input, SecretInput>::new("New User", "New Login", "New Password");
            password.show();

            (*password.ok).borrow_mut().emit(s, true);

            while app.wait() {
                if let Some(msg) = r.recv() {
                    match msg {
                        true => {
                            password.hide();

                            if let Ok(data) = password.set_input() {
                                if !data.first().unwrap().is_ascii()
                                    || !data.last().unwrap().is_ascii()
                                {
                                    alert(
                                        500,
                                        500,
                                        "Incorrect Password. You must use only English letters. Try again",
                                    );

                                    success = 2;
                                } else {
                                    let hash1 =
                                        get_hash(&data.first().unwrap(), 97, 1e9 as u128 + 7);
                                    let hash2 =
                                        get_hash(&data.last().unwrap(), 101, 1e9 as u128 + 7);

                                    File::create("src/utils/admin.bin").unwrap().write(
                                        format!(
                                            "{}",
                                            hash1
                                                .iter()
                                                .map(|x| *x as u8 as char)
                                                .collect::<String>()
                                                + "\0"
                                                + hash2
                                                    .iter()
                                                    .map(|x| *x as u8 as char)
                                                    .collect::<String>()
                                                    .as_str()
                                        )
                                        .as_bytes(),
                                    )?;

                                    fltk::dialog::message(
                                        500,
                                        500,
                                        "New login and password are saved",
                                    );
                                    success = 1;
                                    break;
                                }
                            }
                        }
                        false => (),
                    }
                }
            }

            if success != 2 {
                password.hide();
                break;
            }
        }
    } else {
        let admin_data = adm.split('\0').collect::<Vec<_>>();
        let (s, r) = app::channel();

        loop {
            success = 0;
            let mut password = Input2::<Input, SecretInput>::new(
                match lang {
                    Lang::English => "Authorization",
                    Lang::Russian => "Авторизация",
                },
                match lang {
                    Lang::English => "Login",
                    Lang::Russian => "Логин",
                },
                match lang {
                    Lang::English => "Password",
                    Lang::Russian => "Пароль",
                },
            );
            password.show();

            (*password.ok).borrow_mut().emit(s, true);

            while app.wait() {
                if let Some(msg) = r.recv() {
                    match msg {
                        true => {
                            let input = password.set_input();
                            password.hide();

                            if let Ok(data) = input {
                                let hash1 = get_hash(&data.first().unwrap(), 97, 1e9 as u128 + 7);
                                let hash2 = get_hash(&data.last().unwrap(), 101, 1e9 as u128 + 7);

                                let rehash1 =
                                    hash1.iter().map(|x| *x as u8 as char).collect::<String>();
                                let rehash2 =
                                    hash2.iter().map(|x| *x as u8 as char).collect::<String>();

                                if format!("{}", rehash1)
                                    == format!("{}", admin_data.first().unwrap())
                                    && format!("{}", rehash2)
                                        == format!("{}", *admin_data.last().unwrap())
                                {
                                    fltk::dialog::message(
                                        500,
                                        500,
                                        match lang {
                                            Lang::English => "Authorization is complete",
                                            Lang::Russian => "Авторизация пройдена",
                                        },
                                    );
                                    success = 1;
                                    break;
                                } else {
                                    success = 2;
                                    alert(
                                        500,
                                        500,
                                        match lang {
                                            Lang::English => "Wrong login or password. Try again",
                                            Lang::Russian => "Неправильный логин или пароль",
                                        },
                                    );
                                }
                            }
                        }
                        false => (),
                    }
                }
            }

            if success != 2 {
                break;
            }
        }
    }

    if success == 0 {
        return Ok(());
    }

    let mut main_window = MenuWindow::default()
        .with_label(match lang {
            Lang::English => "Library System",
            Lang::Russian => "Система Учёта Библиотеки",
        })
        .with_size(1800, 900)
        .center_screen();

    let mut frame = Frame::new(0, 0, 1800, 900, "");
    let mut background = SharedImage::load("src/utils/background.jpg")?;
    frame.draw2(move |f| background.draw(f.x(), f.y(), f.width(), f.height()));

    main_window.set_icon(Some(JpegImage::load("src/utils/icon.jpg")?));

    let mut time = fltk::misc::Clock::new(1680, 10, 100, 100, "");
    time.set_type(fltk::misc::ClockType::Square);

    let mut table = Table::new(10, 120, 1780, 890, "");
    table.set_rows(max(50, (*reader_base).borrow().len() as u32));
    table.set_row_header(true);
    table.set_cols(4);
    table.set_col_header(true);
    table.set_col_width_all(460);
    table.end();

    let mut hello = Frame::new(
        0,
        15,
        1800,
        80,
        match lang {
            Lang::English => "BOOK LIBRARY INTERFACE",
            Lang::Russian => "СИСТЕМА УЧЁТА КНИГ",
        },
    );

    hello.set_label_font(Font::HelveticaBoldItalic);
    hello.set_label_color(Color::from_u32(0x1DE7D8));
    hello.set_label_size(50);

    main_window.end();
    main_window.make_resizable(true);

    let rb = reader_base.clone();
    let bs = book_system.clone();

    table.draw_cell2(move |t, ctx, row, col, x, y, w, h| match ctx {
        table::TableContext::StartPage => draw::set_font(Font::Helvetica, 14),

        table::TableContext::ColHeader => draw_header(
            &format!(
                "{}",
                match col {
                    0 => match lang {
                        Lang::English => "Reader",
                        Lang::Russian => "Читатель",
                    },

                    1 => match lang {
                        Lang::English => "Book",
                        Lang::Russian => "Книга",
                    },

                    2 => match lang {
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

        table::TableContext::RowHeader => draw_header(&format!("{}", row + 1), x, y, w, h),

        table::TableContext::Cell => {
            let pair = unsafe { cell_reader(col, row, &*(*rb).as_ptr(), &*(*bs).as_ptr(), lang) };

            draw_data(
                &format!("{}", pair.0),
                x,
                y,
                w,
                h,
                t.is_selected(row, col),
                pair.1,
            );
        }

        _ => (),
    });

    let mut menu = MenuBar::new(
        0,
        0,
        350 - match lang {
            Lang::English => 0,
            Lang::Russian => 40,
        },
        30,
        "",
    );
    main_window.add(&menu);

    menu.add_emit(
        match lang {
            Lang::English => "&Readers/Add reader\t",
            Lang::Russian => "&Читатели/Добавить читателя\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::AddReader,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Readers/Remove reader\t",
            Lang::Russian => "&Читатели/Удалить читателя\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::RemoveReader,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Readers/Change name\t",
            Lang::Russian => "&Читатели/Изменить имя\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::ChangeName,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Readers/Change second name\t",
            Lang::Russian => "&Читатели/Изменить фамилию\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::ChangeFamily,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Readers/Change middle name\t",
            Lang::Russian => "&Читатели/Изменить отчество\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::ChangeFather,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Readers/Change age\t",
            Lang::Russian => "&Читатели/Изменить возраст\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::ChangeAge,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Readers/Get reader's information\t",
            Lang::Russian => "&Читатели/Получить информацию о читателе\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::InfoReader,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Books/Add books\t",
            Lang::Russian => "&Книги/Добавить книги\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::AddBooks,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Books/Remove book\t",
            Lang::Russian => "&Книги/Удалить книгу\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::RemoveBook,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Books/Remove all specific books\t",
            Lang::Russian => "&Книги/Убрать все схожие книги\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::RemoveTheBook,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Books/Change book's title\t",
            Lang::Russian => "&Книги/Изменить название\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::ChangeTitle,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Books/Change book's author\t",
            Lang::Russian => "&Книги/Изменить автора\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::ChangeAuthor,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Books/Change book's amount of pages\t",
            Lang::Russian => "&Книги/Изменить количество страниц\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::ChangePages,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Books/Get type book's information\t",
            Lang::Russian => "&Книги/Получить информацию о всех схожих книгах\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::InfoTheBook,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Books/Get current book's information\t",
            Lang::Russian => "&Книги/Получить информацию о конкретной книгe\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::InfoBook,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Books/All genres\t",
            Lang::Russian => "&Книги/Все жанры\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::ShowGenres,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Books/Add genre\t",
            Lang::Russian => "&Книги/Добавить жанр\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::AddGenre,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Books/Remove genre\t",
            Lang::Russian => "&Книги/Удалить жанр\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::RemoveGenre,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Books/Customize book genres\t",
            Lang::Russian => "&Книги/Изменить жанры книги\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::CustomizeBookGenre,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Books/Find books by genre\t",
            Lang::Russian => "&Книги/Найти книгу по жанру\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::FindByGenre,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Books/List of all books\t",
            Lang::Russian => "&Книги/Все книги\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::ShowAllBooks,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Giveaway/Give book\t",
            Lang::Russian => "&Выдача/Выдать книгу\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::GiveBook,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Giveaway/Get book\t",
            Lang::Russian => "&Выдача/Вернуть книгу\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::GetBook,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Restore/Restore previous data\t",
            Lang::Russian => "&Откат/Откатить изменения назад\t",
        },
        fltk::enums::Shortcut::Ctrl | 'z',
        MenuFlag::Normal,
        s,
        Message::PrevData,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Restore/Restore next data\t",
            Lang::Russian => "&Откат/Откатить изменения вперед\t",
        },
        fltk::enums::Shortcut::Ctrl | fltk::enums::Shortcut::Shift | 'z',
        MenuFlag::Normal,
        s,
        Message::NextData,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Language/English\t",
            Lang::Russian => "&Язык/Английский\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::English,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Language/Russian\t",
            Lang::Russian => "&Язык/Русский\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::Russian,
    );

    main_window.show();

    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                Message::AddReader => {
                    add_reader(
                        &mut (*reader_base).borrow_mut(),
                        &mut (*book_system).borrow_mut(),
                        &(*genres).borrow(),
                        &mut (*caretaker).borrow_mut(),
                        &app,
                        lang,
                    );

                    table.set_rows(max(50, (*reader_base).borrow().len() as u32));
                    table.redraw();
                }

                Message::RemoveReader => {
                    remove_reader(
                        &mut (*reader_base).borrow_mut(),
                        &mut (*book_system).borrow_mut(),
                        &(*genres).borrow(),
                        &mut (*caretaker).borrow_mut(),
                        &app,
                        lang,
                    );

                    table.set_rows(max(50, (*reader_base).borrow().len() as u32));
                    table.redraw();
                }

                Message::ChangeName => {
                    change_name(
                        &mut (*reader_base).borrow_mut(),
                        &mut (*book_system).borrow_mut(),
                        &(*genres).borrow(),
                        &mut (*caretaker).borrow_mut(),
                        &app,
                        lang,
                    );

                    table.redraw();
                }

                Message::ChangeFamily => {
                    change_family(
                        &mut (*reader_base).borrow_mut(),
                        &mut (*book_system).borrow_mut(),
                        &(*genres).borrow(),
                        &mut (*caretaker).borrow_mut(),
                        &app,
                        lang,
                    );

                    table.redraw();
                }

                Message::ChangeFather => {
                    change_father(
                        &mut (*reader_base).borrow_mut(),
                        &mut (*book_system).borrow_mut(),
                        &(*genres).borrow(),
                        &mut (*caretaker).borrow_mut(),
                        &app,
                        lang,
                    );

                    table.redraw();
                }

                Message::ChangeAge => {
                    change_age(
                        &mut (*reader_base).borrow_mut(),
                        &mut (*book_system).borrow_mut(),
                        &(*genres).borrow(),
                        &mut (*caretaker).borrow_mut(),
                        &app,
                        lang,
                    );

                    table.redraw();
                }

                Message::InfoReader => {
                    reader_info(
                        reader_base.clone(),
                        book_system.clone(),
                        &(*genres).borrow(),
                        &mut (*caretaker).borrow_mut(),
                        &app,
                        lang,
                    );

                    table.redraw();
                }

                Message::AddBooks => {
                    add_books(
                        &mut (*book_system).borrow_mut(),
                        &(*reader_base).borrow(),
                        &(*genres).borrow(),
                        &mut (*caretaker).borrow_mut(),
                        &app,
                        lang,
                    );

                    table.redraw();
                }

                Message::RemoveBook => {
                    remove_book(
                        &mut (*book_system).borrow_mut(),
                        &mut (*reader_base).borrow_mut(),
                        &(*genres).borrow(),
                        &mut (*caretaker).borrow_mut(),
                        &app,
                        lang,
                    );

                    table.redraw();
                }

                Message::RemoveTheBook => {
                    remove_the_book(
                        &mut (*book_system).borrow_mut(),
                        &mut (*reader_base).borrow_mut(),
                        &(*genres).borrow(),
                        &mut (*caretaker).borrow_mut(),
                        &app,
                        lang,
                    );

                    table.redraw();
                }

                Message::ChangeTitle => {
                    change_title(
                        &mut (*book_system).borrow_mut(),
                        &mut (*reader_base).borrow_mut(),
                        &(*genres).borrow(),
                        &mut (*caretaker).borrow_mut(),
                        &app,
                        lang,
                    );

                    table.redraw();
                }

                Message::ChangeAuthor => {
                    change_author(
                        &mut (*book_system).borrow_mut(),
                        &mut (*reader_base).borrow_mut(),
                        &(*genres).borrow(),
                        &mut (*caretaker).borrow_mut(),
                        &app,
                        lang,
                    );

                    table.redraw();
                }

                Message::ChangePages => {
                    change_pages(
                        &mut (*book_system).borrow_mut(),
                        &mut (*reader_base).borrow_mut(),
                        &(*genres).borrow(),
                        &mut (*caretaker).borrow_mut(),
                        &app,
                        lang,
                    );

                    table.redraw();
                }

                Message::InfoTheBook => {
                    the_book_info(
                        book_system.clone(),
                        reader_base.clone(),
                        &(*genres).borrow(),
                        &mut (*caretaker).borrow_mut(),
                        &app,
                        lang,
                    );
                    table.redraw();
                }

                Message::InfoBook => {
                    book_info(&(*book_system).borrow(), &app, lang);
                    table.redraw();
                }

                Message::ShowGenres => {
                    all_genres(genres.clone(), &*(*book_system).borrow(), &app, lang)
                }

                Message::AddGenre => add_genre(
                    &mut (*genres).borrow_mut(),
                    &(*reader_base).borrow(),
                    &(*book_system).borrow(),
                    &mut *(caretaker).borrow_mut(),
                    &app,
                    lang,
                ),

                Message::RemoveGenre => remove_genre(
                    &mut (*genres).borrow_mut(),
                    &(*reader_base).borrow(),
                    &(*book_system).borrow(),
                    &mut *(caretaker).borrow_mut(),
                    &app,
                    lang,
                ),

                Message::CustomizeBookGenre => customize_book_genre(
                    &(*genres).borrow(),
                    &mut (*book_system).borrow_mut(),
                    &(*reader_base).borrow(),
                    &mut *(caretaker).borrow_mut(),
                    &app,
                    lang,
                ),

                Message::FindByGenre => find_by_genre(&*(*book_system).borrow(), &app, lang),

                Message::ShowAllBooks => show_all_books(book_system.clone(), lang),

                Message::GiveBook => {
                    give_book(
                        &mut (*reader_base).borrow_mut(),
                        &mut (*book_system).borrow_mut(),
                        &(*genres).borrow(),
                        &mut *(caretaker).borrow_mut(),
                        &app,
                        lang,
                    );
                    table.redraw();
                }

                Message::GetBook => {
                    get_book(
                        &mut (*reader_base).borrow_mut(),
                        &mut (*book_system).borrow_mut(),
                        &(*genres).borrow(),
                        &mut *(caretaker).borrow_mut(),
                        &app,
                        lang,
                    );

                    table.redraw();
                }

                Message::PrevData => {
                    (*caretaker).borrow_mut().add_memento(
                        &(*reader_base).borrow(),
                        &(*book_system).borrow(),
                        &(*genres).borrow(),
                    );

                    unsafe { (*caretaker).borrow_mut().__ind_minus() }

                    (*caretaker).borrow_mut().get_memento_back(
                        &mut *(reader_base).borrow_mut(),
                        &mut *(book_system).borrow_mut(),
                        &mut *(genres).borrow_mut(),
                    );
                    table.redraw();
                }

                Message::NextData => {
                    (*caretaker).borrow_mut().get_memento_forward(
                        &mut *(reader_base).borrow_mut(),
                        &mut *(book_system).borrow_mut(),
                        &mut *(genres).borrow_mut(),
                    );
                    table.redraw();
                }

                Message::English => {
                    if fltk::dialog::choice(
                        500,
                        500,
                        match lang {
                            Lang::English => "Are you sure you want to change your language? You'll have to restart the program",
                            Lang::Russian => "Вы уверены, что хотите сменить язык? Для этого придётся перезапустить программу"
                        },
                        match lang {
                            Lang::English => "Ok",
                            Lang::Russian => "Ок"
                        },
                        match lang {
                            Lang::English => "Cancel",
                            Lang::Russian => "Отмена"
                        },
                        ""
                    ) == 0 {
                        Lang::change(Lang::English);
                        app.quit()
                    }
                }

                Message::Russian => {
                    if fltk::dialog::choice(
                        500,
                        500,
                        match lang {
                            Lang::English => "Are you sure you want to change your language? You'll have to restart the program",
                            Lang::Russian => "Вы уверены, что хотите сменить язык? Для этого придётся перезапустить программу"
                        },
                        match lang {
                            Lang::English => "Ok",
                            Lang::Russian => "Ок"
                        },
                        match lang {
                            Lang::English => "Cancel",
                            Lang::Russian => "Отмена"
                        },
                        ""
                    ) == 0 {
                        Lang::change(Lang::Russian);
                        app.quit()
                    }
                }
            }
        }

        let len = (*reader_base).borrow().len();

        (0..len).for_each(|i| {
            if table.is_selected(i as i32, 0) {
                reader_info_simple(
                    i,
                    &mut (*reader_base).borrow_mut(),
                    &mut (*book_system).borrow_mut(),
                    &(*genres).borrow(),
                    &mut (*caretaker).borrow_mut(),
                    &app,
                    lang,
                );

                table.unset_selection();
                return;
            }

            if table.is_selected(i as i32, 1) {
                book_info_simple(
                    (*reader_base).borrow().get_book(i),
                    &mut (*book_system).borrow_mut(),
                    &app,
                    lang,
                );

                table.unset_selection();
                return;
            }
        });
    }

    app.run().unwrap();
    Ok(())
}
