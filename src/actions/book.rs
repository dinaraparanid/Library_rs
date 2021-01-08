extern crate fltk;
use crate::actions::tables::*;
use crate::books::book_sys::BookSystem;
use crate::change::input1::Input1;
use crate::change::input3::Input3;
use crate::change::Inputable;
use crate::reading::read_base::ReaderBase;
use fltk::app::{channel, App};
use fltk::dialog::alert;
use fltk::frame::Frame;
use fltk::group::VGrid;
use fltk::input::*;
use fltk::prelude::*;
use fltk::table::*;
use fltk::window::SingleWindow;
use fltk::{app, draw};
use std::cmp::max;
use std::num::ParseIntError;

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
            alert(500, 500, "Pages are empty");
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
                alert(500, 500, "Pages input error");
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

/// Function that add simple books.
/// If number of books to add plus
/// number of existing books
/// is more than **usize::MAX**,
/// than you will receive message about it.
/// If you have mistakes in input,
/// program will let you know

pub fn add_books(book_system: &mut BookSystem, app: &App) {
    let (s2, r2) = app::channel();
    let mut inp = Input3::<Input, Input, IntInput>::new("Add Books", "Title", "Author", "Pages");

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

                        let (s3, r3) = app::channel();
                        let mut get_amount =
                            Input1::<IntInput>::new("Books amount", "Amount of books to add");

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
                                                        fltk::dialog::message(
                                                            500,
                                                            500,
                                                            "Successfully added",
                                                        );
                                                        book_system.save();
                                                    }

                                                    Err(_) => alert(500, 500, "Too much books"),
                                                },

                                                Err(_) => {
                                                    alert(500, 500, "Amount of books input error");
                                                    println!(
                                                        "{:?}",
                                                        amount
                                                            .last()
                                                            .unwrap()
                                                            .trim()
                                                            .parse::<usize>()
                                                    )
                                                }
                                            }
                                        }
                                    }

                                    false => (),
                                }
                            } else if !get_amount.shown() {
                                break;
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

/// Function that removes simple books.
/// It takes index of book, so be careful.
/// If you have mistakes in input,
/// program will let you know

pub fn remove_book(book_system: &mut BookSystem, reader_base: &mut ReaderBase, app: &App) {
    let (s2, r2) = app::channel();
    let mut inp = Input3::<Input, Input, IntInput>::new("Remove Book", "Title", "Author", "Pages");

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
                                                Ok(x) => {
                                                    match book_system.remove_one_book(index, x) {
                                                        Ok(_) => {
                                                            fltk::dialog::message(
                                                                500,
                                                                500,
                                                                "Successfully removed",
                                                            );
                                                            book_system.save();
                                                            reader_base.save();
                                                        }

                                                        Err(_) => alert(
                                                            500,
                                                            500,
                                                            "Incorrect number of book",
                                                        ),
                                                    }
                                                }

                                                Err(_) => {
                                                    alert(500, 500, "Book's number input error");
                                                    println!(
                                                        "{:?}",
                                                        ind.last().unwrap().trim().parse::<usize>()
                                                    )
                                                }
                                            }
                                        }
                                    }

                                    false => (),
                                }
                            } else if !get_ind.shown() {
                                break;
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

/// Function that add new book and **ONE** simple book.
/// If you have mistakes in input,
/// program will let you know

pub fn add_book(book_system: &mut BookSystem, app: &App) {
    let (s2, r2) = app::channel();
    let mut inp = Input3::<Input, Input, IntInput>::new("Add New Book", "Title", "Author", "Pages");

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

pub fn remove_the_book(book_system: &mut BookSystem, reader_base: &mut ReaderBase, app: &App) {
    let (s2, r2) = app::channel();
    let mut inp = Input3::<Input, Input, IntInput>::new("Remove Books", "Title", "Author", "Pages");

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

                        match book_system.remove_book(index) {
                            Ok(_) => {
                                fltk::dialog::message(500, 500, "Successfully removed");
                                book_system.save();
                                reader_base.save();
                            }

                            Err(_) => alert(500, 500, "Wrong book's number"),
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

/// Function that changes title
/// of all simple books and TheBook.
/// If you have mistakes in input,
/// program will let you know

pub fn change_title(book_system: &mut BookSystem, reader_base: &mut ReaderBase, app: &App) {
    let (s2, r2) = app::channel();
    let mut inp = Input3::<Input, Input, IntInput>::new("Change Title", "Title", "Author", "Pages");

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
                                                    return;
                                                }

                                                match book_system.change_title(
                                                    index,
                                                    new_title.get_unchecked(0).clone(),
                                                ) {
                                                    Ok(_) => {
                                                        fltk::dialog::message(
                                                            500,
                                                            500,
                                                            "Successfully changed",
                                                        );
                                                        book_system.save();
                                                        reader_base.save();
                                                    }

                                                    Err(_) => alert(
                                                        500,
                                                        500,
                                                        "Book with same parameters already exists",
                                                    ),
                                                }
                                            }
                                        }
                                    }

                                    false => (),
                                }
                            } else if !get_title.shown() {
                                break;
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

/// Function that changes author
/// of all simple books and TheBook.
/// If you have mistakes in input,
/// program will let you know

pub fn change_author(book_system: &mut BookSystem, reader_base: &mut ReaderBase, app: &App) {
    let (s2, r2) = app::channel();
    let mut inp =
        Input3::<Input, Input, IntInput>::new("Change Author", "Title", "Author", "Pages");

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
                                                    alert(500, 500, "New author is empty");
                                                    return;
                                                }

                                                match book_system.change_author(
                                                    index,
                                                    new_author.get_unchecked(0).clone(),
                                                ) {
                                                    Ok(_) => {
                                                        fltk::dialog::message(
                                                            500,
                                                            500,
                                                            "Successfully changed",
                                                        );
                                                        book_system.save();
                                                        reader_base.save();
                                                    }

                                                    Err(_) => alert(
                                                        500,
                                                        500,
                                                        "Book with same parameters already exists",
                                                    ),
                                                }
                                            }
                                        }
                                    }

                                    false => (),
                                }
                            } else if !get_author.shown() {
                                break;
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

/// Function that changes pages
/// of all simple books and TheBook.
/// If you have mistakes in input,
/// program will let you know

pub fn change_pages(book_system: &mut BookSystem, reader_base: &mut ReaderBase, app: &App) {
    let (s2, r2) = app::channel();
    let mut inp = Input3::<Input, Input, IntInput>::new("Change Pages", "Title", "Author", "Pages");

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

                        let (s3, r3) = app::channel();
                        let mut get_pages =
                            Input1::<IntInput>::new("New Amount of Pages", "New Amount of Pages");

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

                                                match book_system.change_pages(
                                                    index,
                                                    new_pages.get_unchecked(0).clone(),
                                                ) {
                                                    Ok(_) => {
                                                        fltk::dialog::message(
                                                            500,
                                                            500,
                                                            "Successfully changed",
                                                        );
                                                        book_system.save();
                                                        reader_base.save();
                                                    }

                                                    Err(0) => alert(
                                                        500,
                                                        500,
                                                        "New amount of pages input error",
                                                    ),

                                                    Err(_) => alert(
                                                        500,
                                                        500,
                                                        "Book with same parameters already exists",
                                                    ),
                                                }
                                            }
                                        }
                                    }

                                    false => (),
                                }
                            } else if !get_pages.shown() {
                                break;
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

/// Function that gives all information about TheBook:
/// title, author, pages, amount of simple books.
/// If you have mistakes in input,
/// program will let you know

pub fn book_info(book_system: &BookSystem, app: &App) {
    let (s2, r2) = app::channel();
    let mut inp = Input3::<Input, Input, IntInput>::new("Find Book", "Title", "Author", "Pages");

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    let book_params = inp.set_input();
                    inp.hide();

                    if let Ok(the_book) = book_params {
                        match the_book.last().unwrap().trim().parse::<u16>() {
                            Ok(x) => unsafe {
                                let ind = book_system.find_book(
                                    the_book.get_unchecked(0),
                                    the_book.get_unchecked(1),
                                    x,
                                );

                                if ind.is_none() {
                                    alert(500, 500, "Book isn't found");
                                    return;
                                }

                                let mut wind = SingleWindow::new(
                                    800,
                                    500,
                                    300,
                                    100,
                                    format!(
                                        "{} {}",
                                        the_book.get_unchecked(0),
                                        the_book.get_unchecked(1)
                                    )
                                    .as_str(),
                                );

                                let mut table = VGrid::new(0, 0, 300, 100, "");
                                table.set_params(4, 1, 1);

                                table.add(&Frame::new(
                                    10,
                                    50,
                                    100,
                                    30,
                                    format!("Title: {}", the_book.get_unchecked(0)).as_str(),
                                ));

                                table.add(&Frame::new(
                                    30,
                                    50,
                                    100,
                                    30,
                                    format!("Author: {}", the_book.get_unchecked(0)).as_str(),
                                ));

                                table.add(&Frame::new(
                                    50,
                                    50,
                                    100,
                                    30,
                                    format!("Pages: {}", x).as_str(),
                                ));

                                table.add(&Frame::new(
                                    70,
                                    50,
                                    100,
                                    30,
                                    format!(
                                        "Amount of books: {}",
                                        (*book_system.books.get_unchecked(ind.unwrap()))
                                            .borrow()
                                            .books
                                            .len()
                                    )
                                    .as_str(),
                                ));

                                table.auto_layout();

                                wind.end();
                                wind.show();
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

/// Function that shows all information about all existing books:
/// title, author, num of pages and num of available simple books

#[inline]
pub fn show_all_books(book_system: &'static BookSystem) {
    let mut wind = SingleWindow::default()
        .with_label("All Books")
        .with_size(820, 550)
        .center_screen();

    let mut table = Table::new(10, 10, 800, 540, "");
    table.set_rows(max(20, book_system.books.len() as u32));
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
            &format!("{}", cell_book(col, row, book_system)),
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
