extern crate fltk;
use fltk::app::AppScheme;
use fltk::enums::Shortcut;
use fltk::{app, button::*, menu::*, window::*};
use librs::actions::giveaway::{get_book, give_book};
use librs::actions::{book::*, read::*};
use librs::book::BookSystem;
use librs::reader::ReaderBase;

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
}

fn main() {
    let app = app::App::default().with_scheme(AppScheme::Gleam);
    let (s, r) = app::channel::<Message>();
    let mut reader_base = ReaderBase::new();
    let mut book_system = BookSystem::new();

    reader_base.load();
    book_system.load(&mut reader_base);

    let mut main_window = MenuWindow::default()
        .with_label("Library System")
        .with_size(1800, 900)
        .center_screen();

    main_window.end();
    main_window.make_resizable(true);

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
        "&Books/Add existing books.yaml\t",
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
        "&Books/Remove all specific books.yaml\t",
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
            match msg {
                Message::AddReader => add_reader(&mut reader_base, &app),
                Message::RemoveReader => remove_reader(&mut reader_base, &app),
                Message::ChangeName => change_name(&mut reader_base, &app),
                Message::ChangeFamily => change_family(&mut reader_base, &app),
                Message::ChangeFather => change_father(&mut reader_base, &app),
                Message::ChangeAge => change_age(&mut reader_base, &app),
                Message::InfoReader => reader_info(&mut reader_base, &app),

                Message::AddBooks => add_books(&mut book_system, &app),
                Message::RemoveBook => remove_book(&mut book_system, &app),
                Message::AddTheBook => add_book(&mut book_system, &app),
                Message::RemoveTheBook => remove_the_book(&mut book_system, &app),
                Message::ChangeTitle => change_title(&mut book_system, &app),
                Message::ChangeAuthor => change_author(&mut book_system, &app),
                Message::ChangePages => change_pages(&mut book_system, &app),
                Message::InfoTheBook => book_info(&mut book_system, &app),

                Message::GiveBook => give_book(&mut reader_base, &mut book_system, &app),
                Message::GetBook => get_book(&mut reader_base, &mut book_system, &app),
            }
        }
    }

    app.run().unwrap();
}
