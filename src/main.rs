extern crate fltk;
use fltk::app::AppScheme;
use fltk::enums::Shortcut;
use fltk::frame::Frame;
use fltk::table::Table;
use fltk::{app, button::*, draw, menu::*, table, window::*};
use librs::actions::{book::*, giveaway::*, read::*, reader_table::*};
use librs::book::BookSystem;
use librs::reader::ReaderBase;
use std::cmp::max;

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
}

/// I'm **really sorry** about this,
/// but FLTK's realisation requires it :(
static mut READER_BASE: ReaderBase = ReaderBase::new();

fn main() {
    let app = app::App::default().with_scheme(AppScheme::Plastic);
    let (s, r) = app::channel::<Message>();
    let mut book_system = BookSystem::new();

    unsafe {
        READER_BASE.load();
        book_system.load(&mut READER_BASE);
    }

    let mut main_window = MenuWindow::default()
        .with_label("Library System")
        .with_size(1800, 900)
        .center_screen();

    let mut table = Table::new(10, 50, 1780, 890, "");
    table.set_rows(max(50, unsafe { READER_BASE.len() } as u32));
    table.set_row_header(true);
    table.set_row_resize(true);
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
                &format!("{}", cell(col, row, &mut READER_BASE)),
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
                        remove_reader(&mut READER_BASE, &mut book_system, &app);
                        table.set_rows(max(50, READER_BASE.len() as u32));
                        table.redraw();
                    }

                    Message::ChangeName => {
                        change_name(&mut READER_BASE, &mut book_system, &app);
                        table.redraw();
                    }

                    Message::ChangeFamily => {
                        change_family(&mut READER_BASE, &mut book_system, &app);
                        table.redraw();
                    }

                    Message::ChangeFather => {
                        change_father(&mut READER_BASE, &mut book_system, &app);
                        table.redraw();
                    }

                    Message::ChangeAge => {
                        change_age(&mut READER_BASE, &mut book_system, &app);
                        table.redraw();
                    }

                    Message::InfoReader => reader_info(&mut READER_BASE, &app),

                    Message::AddBooks => {
                        add_books(&mut book_system, &app);
                        table.redraw();
                    }

                    Message::RemoveBook => {
                        remove_book(&mut book_system, &mut READER_BASE, &app);
                        table.redraw();
                    }

                    Message::AddTheBook => {
                        add_book(&mut book_system, &app);
                        table.redraw();
                    }

                    Message::RemoveTheBook => {
                        remove_the_book(&mut book_system, &mut READER_BASE, &app);
                        table.redraw();
                    }

                    Message::ChangeTitle => {
                        change_title(&mut book_system, &mut READER_BASE, &app);
                        table.redraw();
                    }

                    Message::ChangeAuthor => {
                        change_author(&mut book_system, &mut READER_BASE, &app);
                        table.redraw();
                    }

                    Message::ChangePages => {
                        change_pages(&mut book_system, &mut READER_BASE, &app);
                        table.redraw();
                    }

                    Message::InfoTheBook => book_info(&mut book_system, &app),

                    Message::GiveBook => {
                        give_book(&mut READER_BASE, &mut book_system, &app);
                        table.redraw();
                    }

                    Message::GetBook => {
                        get_book(&mut READER_BASE, &mut book_system, &app);
                        table.redraw();
                    }
                }
            }
        }
    }

    app.run().unwrap();
}
