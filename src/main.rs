extern crate fltk;
use fltk::app::AppScheme;
use fltk::dialog::alert;
use fltk::enums::Shortcut;
use fltk::frame::Frame;
use fltk::input::{Input, SecretInput};
use fltk::table::Table;
use fltk::{app, button::*, draw, menu::*, table, window::*};
use librs::actions::{book::*, giveaway::*, read::*, tables::*};
use librs::book::BookSystem;
use librs::change_menu::*;
use librs::reader::ReaderBase;
use std::cmp::max;
use std::fs::File;
use std::io::{Read, Write};

/// All messages, which used to call functions

#[derive(Clone, Copy)]
pub enum Message {
    AddReader,
    RemoveReader,
    ChangeName,
    ChangeFamily,
    ChangeFather,
    ChangeAge,
    InfoReader,
    AddBooks,
    RemoveBook,
    AddTheBook,
    RemoveTheBook,
    ChangeTitle,
    ChangeAuthor,
    ChangePages,
    InfoTheBook,
    GiveBook,
    GetBook,
    ShowAllBooks,
}

/// Hashing login and password

fn get_hash(str: &String, p: u128, module: u128, ans: &mut Vec<u128>) {
    ans.resize(str.len(), 0);
    let bytes = str.as_bytes();

    unsafe {
        *ans.get_unchecked_mut(0) = *bytes.get_unchecked(0) as u128;

        for i in 1..str.len() {
            *ans.get_unchecked_mut(i) = ((ans.get_unchecked(i - 1).overflowing_mul(p))
                .0
                .overflowing_add(*bytes.get_unchecked(i) as u128)
                .0)
                % module;
        }
    }
}

/// I'm **really sorry** about this,
/// but FLTK's realisation requires it :(
static mut READER_BASE: ReaderBase = ReaderBase::new();
static mut BOOK_SYSTEM: BookSystem = BookSystem::new();

fn main() {
    let app = app::App::default().with_scheme(AppScheme::Plastic);
    let (s, r) = app::channel();

    unsafe {
        READER_BASE.load();
        BOOK_SYSTEM.load(&mut READER_BASE);
    }

    let mut admin = File::open("src/admin.bin").unwrap();
    let mut adm = String::new();
    admin.read_to_string(&mut adm).unwrap();
    let mut success = false;

    if adm.is_empty() {
        let (s, r) = app::channel();
        let mut password =
            Input2::<Input, SecretInput>::new("New User", "New Login", "New Password");
        password.show();

        (*password.ok).borrow_mut().emit(s, true);

        while app.wait() {
            if let Some(msg) = r.recv() {
                match msg {
                    true => {
                        let input = password.set_input();
                        password.hide();

                        if let Ok(data) = input {
                            let mut new_password = File::create("src/admin.bin").unwrap();

                            let mut hash1 = Vec::new();
                            let mut hash2 = Vec::new();
                            get_hash(&data.first().unwrap(), 97, 1e9 as u128 + 7, &mut hash1);
                            get_hash(&data.last().unwrap(), 53, 1e9 as u128 + 7, &mut hash2);

                            new_password
                                .write(
                                    format!(
                                        "{}",
                                        hash1.iter().map(|x| *x as u8 as char).collect::<String>()
                                            + "\0"
                                            + hash2
                                                .iter()
                                                .map(|x| *x as u8 as char)
                                                .collect::<String>()
                                                .as_str()
                                    )
                                    .as_bytes(),
                                )
                                .unwrap();

                            fltk::dialog::message(500, 500, "New login and password are saved");
                            success = true;
                            break;
                        }
                    }
                    false => (),
                }
            }
        }
    } else {
        let admin_data = adm.split('\0').collect::<Vec<&str>>();
        let (s, r) = app::channel();
        let mut password = Input2::<Input, SecretInput>::new("Authorization", "Login", "Password");
        password.show();

        (*password.ok).borrow_mut().emit(s, true);

        while app.wait() {
            if let Some(msg) = r.recv() {
                match msg {
                    true => {
                        let input = password.set_input();
                        password.hide();

                        if let Ok(data) = input {
                            let mut hash1 = Vec::new();
                            let mut hash2 = Vec::new();
                            get_hash(&data.first().unwrap(), 97, 1e9 as u128 + 7, &mut hash1);
                            get_hash(&data.last().unwrap(), 53, 1e9 as u128 + 7, &mut hash2);

                            let rehash1 =
                                hash1.iter().map(|x| *x as u8 as char).collect::<String>();
                            let rehash2 =
                                hash2.iter().map(|x| *x as u8 as char).collect::<String>();

                            if format!("{}", rehash1) == format!("{}", admin_data.first().unwrap())
                                && format!("{}", rehash2)
                                    == format!("{}", *admin_data.last().unwrap())
                            {
                                fltk::dialog::message(500, 500, "Everything is Ok");
                                success = true;
                                break;
                            } else {
                                alert(500, 500, "Wrong login or password");
                                println!(
                                    "{} != {} or {} != {}",
                                    rehash1,
                                    admin_data.first().unwrap(),
                                    rehash2,
                                    admin_data.last().unwrap(),
                                );
                                app.quit();
                                return;
                            }
                        }
                    }
                    false => (),
                }
            }
        }
    }

    if !success {
        alert(500, 500, "Nothing was inputted");
        return;
    }

    let mut main_window = MenuWindow::default()
        .with_label("Library System")
        .with_size(1800, 900)
        .center_screen();

    let mut table = Table::new(10, 50, 1780, 890, "");
    table.set_rows(max(50, unsafe { READER_BASE.len() } as u32));
    table.set_row_header(true);
    table.set_cols(4);
    table.set_col_header(true);
    table.set_col_width_all(460);
    table.end();

    let mut hello = Frame::new(0, 5, 1800, 40, "BOOK LIBRARY INTERFACE");
    hello.set_label_font(Font::Symbol);
    hello.set_label_color(Color::DarkBlue);
    hello.set_label_size(30);

    main_window.end();
    main_window.make_resizable(true);

    unsafe {
        table.draw_cell2(|t, ctx, row, col, x, y, w, h| match ctx {
            table::TableContext::StartPage => draw::set_font(Font::Helvetica, 14),

            table::TableContext::ColHeader => draw_header(
                &format!(
                    "{}",
                    match col {
                        0 => "Reader",
                        1 => "Book",
                        2 => "Start Date",
                        _ => "Finish Date",
                    }
                ),
                x,
                y,
                w,
                h,
            ),

            table::TableContext::RowHeader => draw_header(&format!("{}", row + 1), x, y, w, h),

            table::TableContext::Cell => draw_data(
                &format!(
                    "{}",
                    cell_reader(col, row, &mut READER_BASE, &mut BOOK_SYSTEM)
                ),
                x,
                y,
                w,
                h,
                t.is_selected(row, col),
            ),

            _ => (),
        });
    }

    let mut menu = MenuBar::new(0, 0, 200, 30, "");
    main_window.add(&menu);

    menu.add_emit(
        "&Readers/Add reader\t",
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::AddReader,
    );

    menu.add_emit(
        "&Readers/Remove reader\t",
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::RemoveReader,
    );

    menu.add_emit(
        "&Readers/Change reader's name\t",
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::ChangeName,
    );

    menu.add_emit(
        "&Readers/Change reader's second name\t",
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::ChangeFamily,
    );

    menu.add_emit(
        "&Readers/Change reader's middle name\t",
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::ChangeFather,
    );

    menu.add_emit(
        "&Readers/Change reader's age\t",
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::ChangeAge,
    );

    menu.add_emit(
        "&Readers/Get reader's information\t",
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::InfoReader,
    );

    menu.add_emit(
        "&Books/Add existing books\t",
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::AddBooks,
    );

    menu.add_emit(
        "&Books/Remove book\t",
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::RemoveBook,
    );

    menu.add_emit(
        "&Books/Add new book\t",
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::AddTheBook,
    );

    menu.add_emit(
        "&Books/Remove all specific books\t",
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::RemoveTheBook,
    );

    menu.add_emit(
        "&Books/Change book's title\t",
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::ChangeTitle,
    );

    menu.add_emit(
        "&Books/Change book's author\t",
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::ChangeAuthor,
    );

    menu.add_emit(
        "&Books/Change book's pages\t",
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::ChangePages,
    );

    menu.add_emit(
        "&Books/Get book's information\t",
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::InfoTheBook,
    );

    menu.add_emit(
        "&Books/Show list of all books\t",
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::ShowAllBooks,
    );

    menu.add_emit(
        "&Giveaway/Give book to reader\t",
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::GiveBook,
    );

    menu.add_emit(
        "&Giveaway/Get book from reader\t",
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::GetBook,
    );

    main_window.show();

    while app.wait() {
        if let Some(msg) = r.recv() {
            unsafe {
                match msg {
                    Message::AddReader => {
                        add_reader(&mut READER_BASE, &app);
                        table.set_rows(max(50, READER_BASE.len() as u32));
                        table.redraw();
                    }

                    Message::RemoveReader => {
                        remove_reader(&mut READER_BASE, &mut BOOK_SYSTEM, &app);
                        table.set_rows(max(50, READER_BASE.len() as u32));
                        table.redraw();
                    }

                    Message::ChangeName => {
                        change_name(&mut READER_BASE, &mut BOOK_SYSTEM, &app);
                        table.redraw();
                    }

                    Message::ChangeFamily => {
                        change_family(&mut READER_BASE, &mut BOOK_SYSTEM, &app);
                        table.redraw();
                    }

                    Message::ChangeFather => {
                        change_father(&mut READER_BASE, &mut BOOK_SYSTEM, &app);
                        table.redraw();
                    }

                    Message::ChangeAge => {
                        change_age(&mut READER_BASE, &mut BOOK_SYSTEM, &app);
                        table.redraw();
                    }

                    Message::InfoReader => reader_info(&READER_BASE, &mut BOOK_SYSTEM, &app),

                    Message::AddBooks => {
                        add_books(&mut BOOK_SYSTEM, &app);
                        table.redraw();
                    }

                    Message::RemoveBook => {
                        remove_book(&mut BOOK_SYSTEM, &mut READER_BASE, &app);
                        table.redraw();
                    }

                    Message::AddTheBook => {
                        add_book(&mut BOOK_SYSTEM, &app);
                        table.redraw();
                    }

                    Message::RemoveTheBook => {
                        remove_the_book(&mut BOOK_SYSTEM, &mut READER_BASE, &app);
                        table.redraw();
                    }

                    Message::ChangeTitle => {
                        change_title(&mut BOOK_SYSTEM, &mut READER_BASE, &app);
                        table.redraw();
                    }

                    Message::ChangeAuthor => {
                        change_author(&mut BOOK_SYSTEM, &mut READER_BASE, &app);
                        table.redraw();
                    }

                    Message::ChangePages => {
                        change_pages(&mut BOOK_SYSTEM, &mut READER_BASE, &app);
                        table.redraw();
                    }

                    Message::InfoTheBook => book_info(&BOOK_SYSTEM, &app),

                    Message::ShowAllBooks => show_all_books(&BOOK_SYSTEM),

                    Message::GiveBook => {
                        give_book(&mut READER_BASE, &mut BOOK_SYSTEM, &app);
                        table.redraw();
                    }

                    Message::GetBook => {
                        get_book(&mut READER_BASE, &mut BOOK_SYSTEM, &app);
                        table.redraw();
                    }
                }
            }
        }
    }

    app.run().unwrap();
}
