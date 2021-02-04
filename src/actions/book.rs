extern crate fltk;

use crate::{
    actions::{read::get_book_ind, tables::*},
    books::{book::Book, book_sys::BookSystem, genres::Genres},
    change::{input1::Input1, input3::Input3, Inputable},
    reading::read_base::ReaderBase,
    restore::caretaker::Caretaker,
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
    table,
    table::Table,
    window::SingleWindow,
};

use std::{
    borrow::Borrow,
    cell::RefCell,
    cmp::max,
    num::ParseIntError,
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

/// Function that checks if input was empty

#[inline]
pub(crate) fn empty_inp_book(inp: &Vec<String>) -> bool {
    unsafe {
        return if inp.get_unchecked(0).is_empty() {
            alert(500, 500, "Title is empty");
            true
        } else if inp.get_unchecked(1).is_empty() {
            alert(500, 500, "Author is empty");
            true
        } else if inp.get_unchecked(2).is_empty() {
            alert(500, 500, "Amount of Pages are empty");
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
pub(crate) fn check_book(book_system: &BookSystem, books: &Vec<String>) -> Result<usize, ()> {
    let pages;
    let ind;

    unsafe {
        if empty_inp_book(books) {
            return Err(());
        }

        match books.get_unchecked(2).trim().parse::<u16>() {
            Ok(x) => pages = x,
            Err(_) => {
                alert(500, 500, "Amount of Pages input error");
                return Err(());
            }
        }

        ind = book_system.find_book(books.get_unchecked(0), books.get_unchecked(1), pages);
    }

    return match ind {
        Some(i) => Ok(i),

        None => {
            alert(500, 500, "Book isn't found");
            Err(())
        }
    };
}

/// Changing title of already known book

#[inline]
fn change_title_simple(
    ind: usize,
    book_system: &mut BookSystem,
    reader_base: &mut ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
) {
    let (s3, r3) = app::channel();
    let mut get_title = Input1::<Input>::new("New Title", "New Title");

    get_title.show();
    (*get_title.ok).borrow_mut().emit(s3, true);

    while app.wait() {
        if let Some(msg) = r3.recv() {
            match msg {
                true => {
                    let title_param = get_title.set_input();
                    get_title.hide();

                    if let Ok(new_title) = title_param {
                        unsafe {
                            if new_title.get_unchecked(0).is_empty() {
                                alert(500, 500, "New title is empty");
                            }

                            match book_system.change_title(ind, new_title.get_unchecked(0).clone())
                            {
                                Ok(_) => {
                                    fltk::dialog::message(500, 500, "Successfully changed");
                                    book_system.save();
                                    reader_base.save();
                                    caretaker.add_memento(reader_base, book_system, genres);
                                }

                                Err(_) => {
                                    alert(500, 500, "Book with same parameters already exists")
                                }
                            }
                        }
                    }
                }

                false => (),
            }
        } else if !get_title.shown() {
            return;
        }
    }
}

/// Changing author of already known book

#[inline]
fn change_author_simple(
    ind: usize,
    book_system: &mut BookSystem,
    reader_base: &mut ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
) {
    let (s3, r3) = app::channel();
    let mut get_author = Input1::<Input>::new("New Author", "New Author");

    get_author.show();
    (*get_author.ok).borrow_mut().emit(s3, true);

    while app.wait() {
        if let Some(msg) = r3.recv() {
            match msg {
                true => {
                    let author_param = get_author.set_input();
                    get_author.hide();

                    if let Ok(new_author) = author_param {
                        unsafe {
                            if new_author.get_unchecked(0).is_empty() {
                                alert(500, 500, "New title is empty");
                            }

                            match book_system
                                .change_author(ind, new_author.get_unchecked(0).clone())
                            {
                                Ok(_) => {
                                    fltk::dialog::message(500, 500, "Successfully changed");
                                    book_system.save();
                                    reader_base.save();
                                    caretaker.add_memento(reader_base, book_system, genres);
                                }

                                Err(_) => {
                                    alert(500, 500, "Book with same parameters already exists")
                                }
                            }
                        }
                    }
                }

                false => (),
            }
        } else if !get_author.shown() {
            return;
        }
    }
}

/// Changing amount of pages of already known book

#[inline]
fn change_pages_simple(
    ind: usize,
    book_system: &mut BookSystem,
    reader_base: &mut ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
) {
    let (s3, r3) = app::channel();
    let mut get_pages = Input1::<IntInput>::new("New Amount of Pages", "New Amount of Pages");

    get_pages.show();
    (*get_pages.ok).borrow_mut().emit(s3, true);

    while app.wait() {
        if let Some(msg) = r3.recv() {
            match msg {
                true => {
                    let pages_param = get_pages.set_input();
                    get_pages.hide();

                    if let Ok(new_pages) = pages_param {
                        unsafe {
                            if new_pages.get_unchecked(0).is_empty() {
                                alert(500, 500, "New amount of pages is empty");
                                return;
                            }

                            match book_system.change_pages(ind, new_pages.get_unchecked(0).clone())
                            {
                                Ok(_) => {
                                    fltk::dialog::message(500, 500, "Successfully changed");
                                    book_system.save();
                                    reader_base.save();
                                    caretaker.add_memento(reader_base, book_system, genres);
                                }

                                Err(0) => alert(500, 500, "New amount of pages input error"),

                                Err(_) => {
                                    alert(500, 500, "Book with same parameters already exists")
                                }
                            }
                        }
                    }
                }

                false => (),
            }
        } else if !get_pages.shown() {
            return;
        }
    }
}

/// Removes already known the book

#[inline]
fn remove_the_book_simple(
    ind: usize,
    book_system: &mut BookSystem,
    reader_base: &mut ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
) {
    match book_system.remove_book(ind) {
        Ok(_) => {
            fltk::dialog::message(500, 500, "Successfully removed");
            book_system.save();
            reader_base.save();
            caretaker.add_memento(reader_base, book_system, genres);
        }

        Err(_) => alert(500, 500, "Wrong book's number"),
    }
}

/// Adds simple books to already known the book

#[inline]
fn add_books_simple(
    ind: usize,
    book_system: &mut BookSystem,
    reader_base: &ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
) {
    let (s3, r3) = app::channel();
    let mut get_amount = Input1::<IntInput>::new("Books amount", "Amount of books to add");

    get_amount.show();
    (*get_amount.ok).borrow_mut().emit(s3, true);

    while app.wait() {
        if let Some(msg) = r3.recv() {
            match msg {
                true => {
                    let amount_param = get_amount.set_input();
                    get_amount.hide();

                    if let Ok(amount) = amount_param {
                        match amount.first().unwrap().trim().parse::<usize>() {
                            Ok(x) => match book_system.add_books(ind, x) {
                                Ok(_) => {
                                    fltk::dialog::message(500, 500, "Successfully added");
                                    book_system.save();
                                    caretaker.add_memento(reader_base, book_system, genres);
                                }

                                Err(_) => alert(500, 500, "Too much books"),
                            },

                            Err(_) => {
                                alert(500, 500, "Amount of books input error");
                                println!("{:?}", amount.last().unwrap().trim().parse::<usize>())
                            }
                        }
                    }
                }

                false => (),
            }
        } else if !get_amount.shown() {
            return;
        }
    }
}

/// Removes one simple book from known the book

#[inline]
fn remove_book_simple(
    index: usize,
    book_system: &mut BookSystem,
    reader_base: &mut ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
) {
    let (s3, r3) = app::channel();
    let mut get_ind = Input1::<IntInput>::new("Book's number", "Book's number");
    get_ind.show();
    (*get_ind.ok).borrow_mut().emit(s3, true);

    while app.wait() {
        if let Some(msg) = r3.recv() {
            match msg {
                true => {
                    let ind_param = get_ind.set_input();
                    get_ind.hide();

                    if let Ok(ind) = ind_param {
                        match ind.first().unwrap().trim().parse::<usize>() {
                            Ok(x) => match book_system.remove_one_book(index, x) {
                                Ok(_) => {
                                    fltk::dialog::message(500, 500, "Successfully removed");
                                    book_system.save();
                                    reader_base.save();
                                    caretaker.add_memento(reader_base, book_system, genres);
                                }

                                Err(_) => alert(500, 500, "Incorrect number of book"),
                            },

                            Err(_) => {
                                alert(500, 500, "Book's number input error");
                                println!("{:?}", ind.last().unwrap().trim().parse::<usize>())
                            }
                        }
                    }
                }

                false => (),
            }
        } else if !get_ind.shown() {
            return;
        }
    }
}

/// Removes one known simple book from known the book

#[inline]
fn remove_book_simple2(
    index: usize,
    s_index: usize,
    book_system: &mut BookSystem,
    reader_base: &mut ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
) {
    unsafe {
        book_system.remove_one_book_unchecked(index, s_index);
    }
    fltk::dialog::message(500, 500, "Successfully removed");
    book_system.save();
    reader_base.save();
    caretaker.add_memento(reader_base, book_system, genres);
}

/// Function that gives information
/// about already known simple book

pub fn book_info_simple(book: Option<Weak<RefCell<Book>>>, book_system: &BookSystem, app: &App) {
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
                    (*b.upgrade().unwrap()).borrow().title,
                    (*b.upgrade().unwrap()).borrow().author,
                    (*b.upgrade().unwrap()).borrow().pages,
                )
                .as_str(),
            )
            .center_screen();

            let mut table1 = VGrid::new(0, 0, 848, 100, "");
            table1.set_params(6, 1, 1);

            table1.add(&Frame::new(
                10,
                50,
                100,
                30,
                format!("Title: {}", (*b.upgrade().unwrap()).borrow().title).as_str(),
            ));

            table1.add(&Frame::new(
                30,
                50,
                100,
                30,
                format!("Author: {}", (*b.upgrade().unwrap()).borrow().author).as_str(),
            ));

            table1.add(&Frame::new(
                50,
                50,
                100,
                30,
                format!(
                    "Amount of Pages: {}",
                    (*b.upgrade().unwrap()).borrow().pages,
                )
                .as_str(),
            ));

            table1.add(&Frame::new(
                50,
                50,
                100,
                30,
                format!(
                    "Order Number: {}",
                    get_book_ind(book_system, b.upgrade().unwrap().as_ptr()),
                )
                .as_str(),
            ));

            table1.add(&Frame::new(
                70,
                50,
                100,
                30,
                format!(
                    "Now is Read By : {}",
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
                            .age
                            .to_string()
                            .as_str()
                    } else {
                        "None".to_string()
                    }
                )
                .as_str(),
            ));

            table1.add(&Frame::new(
                90,
                50,
                100,
                30,
                format!("All Readers:").as_str(),
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
                            0 => "Name",
                            1 => "2-nd Name",
                            2 => "Middle Name",
                            3 => "Age",
                            4 => "Start",
                            _ => "Finish",
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
                "Title: {}",
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
                "Author: {}",
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
                "Amount of Pages: {}",
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
                "Amount of books: {}",
                (**book_system.books.get_unchecked(ind))
                    .borrow()
                    .books
                    .len()
            )
            .as_str(),
        ));

        table.add(&Frame::new(90, 50, 100, 30, format!("Genres:").as_str()));

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
                let gen = cell_genre(row, &b);
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
        "&Change/Change title\t",
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageTheBook::ChangeTitle,
    );

    menu.add_emit(
        "&Change/Change author\t",
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageTheBook::ChangeAuthor,
    );

    menu.add_emit(
        "&Change/Change amount of pages\t",
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageTheBook::ChangePages,
    );

    menu.add_emit(
        "&Remove/Remove all books\t",
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageTheBook::RemoveThis,
    );

    menu.add_emit(
        "&Remove/Remove one book\t",
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageTheBook::RemoveSimple,
    );

    menu.add_emit(
        "&Add book\t",
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageTheBook::AddSimple,
    );

    wind.show();

    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                MessageTheBook::ChangeAuthor => {
                    change_author_simple(ind, book_system, reader_base, genres, caretaker, app)
                }

                MessageTheBook::ChangeTitle => {
                    change_title_simple(ind, book_system, reader_base, genres, caretaker, app)
                }

                MessageTheBook::ChangePages => {
                    change_pages_simple(ind, book_system, reader_base, genres, caretaker, app)
                }

                MessageTheBook::RemoveSimple => {
                    remove_book_simple(ind, book_system, reader_base, genres, caretaker, app)
                }

                MessageTheBook::AddSimple => {
                    add_books_simple(ind, book_system, reader_base, genres, caretaker, app)
                }

                MessageTheBook::RemoveThis => {
                    remove_the_book_simple(ind, book_system, reader_base, genres, caretaker)
                }
            }
            return;
        } else if !wind.shown() {
            return;
        }
    }
}

/// Function that add simple books.
/// If number of books to add plus
/// number of existing books
/// is more than **usize::MAX**,
/// than you will receive message about it.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn add_books(
    book_system: &mut BookSystem,
    reader_base: &ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
) {
    let (s2, r2) = app::channel();
    let mut inp =
        Input3::<Input, Input, IntInput>::new("Add Books", "Title", "Author", "Amount of Pages");
    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    let new_books_params = inp.set_input();
                    inp.hide();

                    if let Ok(books) = new_books_params {
                        let ind;

                        match check_book(book_system, &books) {
                            Ok(x) => ind = x,
                            Err(_) => return,
                        }

                        add_books_simple(ind, book_system, reader_base, genres, caretaker, app);
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

/// Function that removes simple books.
/// It takes index of book, so be careful.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn remove_book(
    book_system: &mut BookSystem,
    reader_base: &mut ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
) {
    let (s2, r2) = app::channel();
    let mut inp =
        Input3::<Input, Input, IntInput>::new("Remove Book", "Title", "Author", "Amount of Pages");

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    let rem_book_params = inp.set_input();
                    inp.hide();

                    if let Ok(book) = rem_book_params {
                        let index;

                        match check_book(book_system, &book) {
                            Ok(x) => index = x,
                            Err(_) => return,
                        }

                        remove_book_simple(index, book_system, reader_base, genres, caretaker, app);
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

/// Function that add new book and **ONE** simple book.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn add_book(
    book_system: &mut BookSystem,
    reader_base: &ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
) {
    let (s2, r2) = app::channel();
    let mut inp =
        Input3::<Input, Input, IntInput>::new("Add New Book", "Title", "Author", "Amount of Pages");

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    let new_book_params = inp.set_input();
                    inp.hide();

                    if let Ok(the_book) = new_book_params {
                        if empty_inp_book(&the_book) {
                            return;
                        }

                        match the_book.last().unwrap().trim().parse::<u16>() {
                            Ok(x) => unsafe {
                                match book_system.add_book(
                                    the_book.get_unchecked(0).clone(),
                                    the_book.get_unchecked(1).clone(),
                                    x,
                                ) {
                                    Ok(_) => {
                                        fltk::dialog::message(500, 500, "Successfully added");
                                        book_system.save();
                                        caretaker.add_memento(reader_base, book_system, genres);
                                    }

                                    Err(_) => {
                                        alert(500, 500, "Book with same parameters already exists")
                                    }
                                }
                            },

                            Err(_) => {
                                alert(500, 500, "Pages input error");
                                println!("{:?}", the_book.last().unwrap().trim().parse::<u16>())
                            }
                        }
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

/// Function that removes all simple books and TheBook.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn remove_the_book(
    book_system: &mut BookSystem,
    reader_base: &mut ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
) {
    let (s2, r2) = app::channel();
    let mut inp =
        Input3::<Input, Input, IntInput>::new("Remove Books", "Title", "Author", "Amount of Pages");

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    let rem_book_params = inp.set_input();
                    inp.hide();

                    if let Ok(the_book) = rem_book_params {
                        let index;

                        match check_book(book_system, &the_book) {
                            Ok(x) => index = x,
                            Err(_) => return,
                        }

                        remove_the_book_simple(index, book_system, reader_base, genres, caretaker);
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

/// Function that changes title
/// of all simple books and TheBook.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn change_title(
    book_system: &mut BookSystem,
    reader_base: &mut ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
) {
    let (s2, r2) = app::channel();
    let mut inp =
        Input3::<Input, Input, IntInput>::new("Change Title", "Title", "Author", "Amount of Pages");

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    let book_params = inp.set_input();
                    inp.hide();

                    if let Ok(book) = book_params {
                        let index;

                        match check_book(book_system, &book) {
                            Ok(x) => index = x,
                            Err(_) => return,
                        }

                        change_title_simple(
                            index,
                            book_system,
                            reader_base,
                            genres,
                            caretaker,
                            app,
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

/// Function that changes author
/// of all simple books and TheBook.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn change_author(
    book_system: &mut BookSystem,
    reader_base: &mut ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
) {
    let (s2, r2) = app::channel();
    let mut inp = Input3::<Input, Input, IntInput>::new(
        "Change Author",
        "Title",
        "Author",
        "Amount of Pages",
    );

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    let book_params = inp.set_input();
                    inp.hide();

                    if let Ok(book) = book_params {
                        let index;

                        match check_book(book_system, &book) {
                            Ok(x) => index = x,
                            Err(_) => return,
                        }

                        change_author_simple(
                            index,
                            book_system,
                            reader_base,
                            genres,
                            caretaker,
                            app,
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

/// Function that changes pages
/// of all simple books and TheBook.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn change_pages(
    book_system: &mut BookSystem,
    reader_base: &mut ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
) {
    let (s2, r2) = app::channel();
    let mut inp =
        Input3::<Input, Input, IntInput>::new("Change Pages", "Title", "Author", "Amount of Pages");

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    let book_params = inp.set_input();
                    inp.hide();

                    if let Ok(book) = book_params {
                        let index;

                        match check_book(book_system, &book) {
                            Ok(x) => index = x,
                            Err(_) => return,
                        }

                        change_pages_simple(
                            index,
                            book_system,
                            reader_base,
                            genres,
                            caretaker,
                            app,
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
) {
    let (s2, r2) = app::channel();
    let mut inp =
        Input3::<Input, Input, IntInput>::new("Find Book", "Title", "Author", "Amount of Pages");

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    let book_params = inp.set_input();
                    inp.hide();

                    if let Ok(the_book) = book_params {
                        let index;

                        match check_book(&(*book_system).borrow(), &the_book) {
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
pub fn book_info(book_system: &BookSystem, app: &App) {
    let (s2, r2) = app::channel();
    let mut inp =
        Input3::<Input, Input, IntInput>::new("Find Book", "Title", "Author", "Amount of Pages");

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    let book_params = inp.set_input();
                    inp.hide();

                    if let Ok(the_book) = book_params {
                        let index;

                        match check_book(book_system, &the_book) {
                            Ok(x) => index = x,
                            Err(_) => return,
                        }

                        let (s, r) = app::channel();
                        let mut inp2 = Input1::<IntInput>::new("Number", "Number of Book");

                        inp2.show();
                        (*inp2.ok).borrow_mut().emit(s, true);

                        while app.wait() {
                            if let Some(msg) = r.recv() {
                                match msg {
                                    true => {
                                        let get_bind = inp2.set_input();
                                        inp2.hide();

                                        if let Ok(bind_v) = get_bind {
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
                                                {
                                                    alert(500, 500, "Incorrect number of book");
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
pub fn show_all_books(book_system: Rc<RefCell<BookSystem>>) {
    let mut wind = SingleWindow::default()
        .with_label("All Books")
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
                    0 => "Title",
                    1 => "Author",
                    2 => "Pages",
                    _ => "Amount of available books",
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
