extern crate fltk;
use booklibrs::{
    actions::{
        book::*,
        genres::{add_genre, customize_book_genre, remove_genre},
        giveaway::*,
        read::*,
        tables::*,
    },
    books::{book_sys::BookSystem, genres::Genres},
    change::{input2::Input2, Inputable},
    reading::read_base::ReaderBase,
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
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
use std::{
    cmp::max,
    fs::File,
    io::{Read, Write},
};

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
    InfoBook,
    GiveBook,
    GetBook,
    ShowAllBooks,
    AddGenre,
    RemoveGenre,
    CustomizeBookGenre,
}

/// Hashing login and password

#[inline]
fn get_hash(str: &String, p: u128, module: u128) -> Vec<u128> {
    let mut ans = vec![0; str.len()];
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

    ans
}

fn main() -> Result<(), Box<dyn Error>> {
    let reader_base = Rc::new(RefCell::new(ReaderBase::new()));
    let book_system = Rc::new(RefCell::new(BookSystem::new()));
    let mut genres = Genres::new();

    let app = app::App::default().with_scheme(fltk::app::AppScheme::Plastic);
    let (s, r) = app::channel();

    (*reader_base).borrow_mut().load();
    (*book_system)
        .borrow_mut()
        .load(&mut (*reader_base).borrow_mut());

    genres.load();

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
                            let input = password.set_input();
                            password.hide();

                            if let Ok(data) = input {
                                let mut new_password = File::create("src/utils/admin.bin").unwrap();
                                let hash1 = get_hash(&data.first().unwrap(), 97, 1e9 as u128 + 7);
                                let hash2 = get_hash(&data.last().unwrap(), 101, 1e9 as u128 + 7);

                                new_password.write(
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
                                )?;

                                fltk::dialog::message(500, 500, "New login and password are saved");
                                success = 1;
                                break;
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
        let admin_data = adm.split('\0').collect::<Vec<&str>>();
        let (s, r) = app::channel();

        loop {
            success = 0;
            let mut password =
                Input2::<Input, SecretInput>::new("Authorization", "Login", "Password");
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
                                    fltk::dialog::message(500, 500, "Everything is Ok");
                                    success = 1;
                                    break;
                                } else {
                                    success = 2;
                                    alert(500, 500, "Wrong login or password. Try again");
                                    println!(
                                        "{} != {} or {} != {}",
                                        rehash1,
                                        admin_data.first().unwrap(),
                                        rehash2,
                                        admin_data.last().unwrap(),
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
        .with_label("Library System")
        .with_size(1800, 900)
        .center_screen();

    let mut frame = Frame::new(0, 0, 1800, 900, "");
    let mut background = SharedImage::load("src/utils/background.jpg")?;
    background.scale(main_window.width(), main_window.height(), true, true);
    frame.draw2(move |f| {
        background.scale(f.width(), f.height(), true, true);
        background.draw(f.x(), f.y(), f.width(), f.height());
    });

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

    let mut hello = Frame::new(0, 15, 1800, 80, "BOOK LIBRARY INTERFACE");
    hello.set_label_font(Font::Symbol);
    hello.set_label_color(Color::DarkBlue);
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

        table::TableContext::Cell => {
            let pair = cell_reader(col, row, &mut (*rb).borrow_mut(), &mut (*bs).borrow_mut());

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
        "&Books/Get type book's information\t",
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::InfoTheBook,
    );

    menu.add_emit(
        "&Books/Get current book's information\t",
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::InfoBook,
    );

    menu.add_emit(
        "&Books/Add genre\t",
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::AddGenre,
    );

    menu.add_emit(
        "&Books/Remove genre\t",
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::RemoveGenre,
    );

    menu.add_emit(
        "&Books/Customize book genres\t",
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        Message::CustomizeBookGenre,
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

    let rb = reader_base.clone();
    let bs = book_system.clone();

    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                Message::AddReader => {
                    add_reader(&mut (*reader_base).borrow_mut(), &app);
                    table.set_rows(max(50, (*reader_base).borrow().len() as u32));
                    table.redraw();
                }

                Message::RemoveReader => {
                    remove_reader(
                        &mut (*reader_base).borrow_mut(),
                        &mut (*book_system).borrow_mut(),
                        &app,
                    );
                    table.set_rows(max(50, (*reader_base).borrow().len() as u32));
                    table.redraw();
                }

                Message::ChangeName => {
                    change_name(
                        &mut (*reader_base).borrow_mut(),
                        &mut (*book_system).borrow_mut(),
                        &app,
                    );
                    table.redraw();
                }

                Message::ChangeFamily => {
                    change_family(
                        &mut (*reader_base).borrow_mut(),
                        &mut (*book_system).borrow_mut(),
                        &app,
                    );
                    table.redraw();
                }

                Message::ChangeFather => {
                    change_father(
                        &mut (*reader_base).borrow_mut(),
                        &mut (*book_system).borrow_mut(),
                        &app,
                    );
                    table.redraw();
                }

                Message::ChangeAge => {
                    change_age(
                        &mut (*reader_base).borrow_mut(),
                        &mut (*book_system).borrow_mut(),
                        &app,
                    );
                    table.redraw();
                }

                Message::InfoReader => {
                    reader_info(rb.clone(), bs.clone(), &app);
                    table.redraw();
                }

                Message::AddBooks => {
                    add_books(&mut (*book_system).borrow_mut(), &app);
                    table.redraw();
                }

                Message::RemoveBook => {
                    remove_book(
                        &mut (*book_system).borrow_mut(),
                        &mut (*reader_base).borrow_mut(),
                        &app,
                    );
                    table.redraw();
                }

                Message::AddTheBook => {
                    add_book(&mut (*book_system).borrow_mut(), &app);
                    table.redraw();
                }

                Message::RemoveTheBook => {
                    remove_the_book(
                        &mut (*book_system).borrow_mut(),
                        &mut (*reader_base).borrow_mut(),
                        &app,
                    );
                    table.redraw();
                }

                Message::ChangeTitle => {
                    change_title(
                        &mut (*book_system).borrow_mut(),
                        &mut (*reader_base).borrow_mut(),
                        &app,
                    );
                    table.redraw();
                }

                Message::ChangeAuthor => {
                    change_author(
                        &mut (*book_system).borrow_mut(),
                        &mut (*reader_base).borrow_mut(),
                        &app,
                    );
                    table.redraw();
                }

                Message::ChangePages => {
                    change_pages(
                        &mut (*book_system).borrow_mut(),
                        &mut (*reader_base).borrow_mut(),
                        &app,
                    );
                    table.redraw();
                }

                Message::InfoTheBook => {
                    the_book_info(bs.clone(), rb.clone(), &app);
                    table.redraw();
                }

                Message::InfoBook => {
                    book_info(&(*book_system).borrow(), &app);
                    table.redraw();
                }

                Message::AddGenre => add_genre(&mut genres, &app),

                Message::RemoveGenre => remove_genre(&mut genres, &app),

                Message::CustomizeBookGenre => {
                    customize_book_genre(&genres, &mut (*book_system).borrow_mut(), &app)
                }

                Message::ShowAllBooks => show_all_books(bs.clone()),

                Message::GiveBook => {
                    give_book(
                        &mut (*reader_base).borrow_mut(),
                        &mut (*book_system).borrow_mut(),
                        &app,
                    );
                    table.redraw();
                }

                Message::GetBook => {
                    get_book(
                        &mut (*reader_base).borrow_mut(),
                        &mut (*book_system).borrow_mut(),
                        &app,
                    );
                    table.redraw();
                }
            }
        }

        let len = (*reader_base).borrow().len();

        for i in 0..len {
            if table.is_selected(i as i32, 0) {
                reader_info_simple(
                    i,
                    &mut (*reader_base).borrow_mut(),
                    &mut (*book_system).borrow_mut(),
                    &app,
                );
                table.unset_selection();
                break;
            }

            if table.is_selected(i as i32, 1) {
                book_info_simple(
                    (*reader_base).borrow().get_book(i),
                    &mut (*book_system).borrow_mut(),
                    &app,
                );
                table.unset_selection();
                break;
            }
        }
    }

    app.run().unwrap();
    Ok(())
}
