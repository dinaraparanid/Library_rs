use crate::book::BookSystem;
use crate::change_menu::*;
use crate::reader::ReaderBase;
use fltk::app;
use fltk::app::{channel, App};
use fltk::dialog::alert;
use fltk::draw::capture_window;
use fltk::frame::Frame;
use fltk::group::VGrid;
use fltk::prelude::*;
use fltk::window::SingleWindow;
use std::num::ParseIntError;

/// Function that add simple books.
/// If number of books to add plus
/// number of existing books
/// is more than **usize::MAX**,
/// than you will receive message about it.
/// If you have mistakes in input,
/// program will let you know

pub fn add_books(book_system: &mut BookSystem, app: &App) {
    let (s2, r2) = app::channel();
    let mut inp = Input3::new("Add Books", "Title", "Author", "Pages");

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    let new_books_params = inp.set_input();
                    inp.hide();

                    if let Ok(books) = new_books_params {
                        let (s3, r3) = app::channel();
                        let mut get_amount = Input1::new("Books amount", "Amount of books to add");
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
                                                Ok(x) => {
                                                    match books
                                                        .last()
                                                        .unwrap()
                                                        .trim()
                                                        .parse::<u16>()
                                                    {
                                                        Ok(a) => unsafe {
                                                            match book_system.add_books(
                                                                books.get_unchecked(0).clone(),
                                                                books.get_unchecked(1).clone(),
                                                                a,
                                                                x,
                                                            ) {
                                                                Ok(_) => {
                                                                    fltk::dialog::message(
                                                                        500,
                                                                        500,
                                                                        "Successfully added",
                                                                    );
                                                                    book_system.save();
                                                                }

                                                                Err(0) => alert(
                                                                    500,
                                                                    500,
                                                                    "Too much books",
                                                                ),

                                                                Err(_) => alert(
                                                                    500,
                                                                    500,
                                                                    "The Book is not found",
                                                                ),
                                                            }
                                                        },

                                                        Err(_) => {
                                                            alert(500, 500, "Pages input error");
                                                            println!(
                                                                "{:?}",
                                                                books
                                                                    .last()
                                                                    .unwrap()
                                                                    .trim()
                                                                    .parse::<u16>()
                                                            )
                                                        }
                                                    }
                                                }

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
    let mut inp = Input3::new("Remove Book", "Title", "Author", "Pages");

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    let rem_book_params = inp.set_input();
                    inp.hide();

                    if let Ok(book) = rem_book_params {
                        let (s3, r3) = app::channel();
                        let mut get_ind = Input1::new("Book's number", "Book's number");
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
                                                    match book.last().unwrap().trim().parse::<u16>()
                                                    {
                                                        Ok(a) => unsafe {
                                                            match book_system.remove_one_book(
                                                                book.get_unchecked(0),
                                                                book.get_unchecked(1),
                                                                a,
                                                                x,
                                                            ) {
                                                                Ok(_) => {
                                                                    fltk::dialog::message(
                                                                        500,
                                                                        500,
                                                                        "Successfully removed",
                                                                    );
                                                                    book_system.save();
                                                                    reader_base.save();
                                                                }

                                                                Err(0) => alert(
                                                                    500,
                                                                    500,
                                                                    "The Book is not found",
                                                                ),

                                                                Err(_) => {
                                                                    alert(
                                                                        500,
                                                                        500,
                                                                        "Book's number input error",
                                                                    );
                                                                    println!(
                                                                        "{:?}",
                                                                        ind.last()
                                                                            .unwrap()
                                                                            .trim()
                                                                            .parse::<usize>()
                                                                    );
                                                                }
                                                            }
                                                        },

                                                        Err(_) => {
                                                            alert(500, 500, "Pages input error");
                                                            println!(
                                                                "{:?}",
                                                                book.last()
                                                                    .unwrap()
                                                                    .trim()
                                                                    .parse::<u16>()
                                                            )
                                                        }
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
    let mut inp = Input3::new("Add New Book", "Title", "Author", "Pages");

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    let new_book_params = inp.set_input();
                    inp.hide();

                    if let Ok(the_book) = new_book_params {
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
    let mut inp = Input3::new("Remove Books", "Title", "Author", "Pages");

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    let rem_book_params = inp.set_input();
                    inp.hide();

                    if let Ok(the_book) = rem_book_params {
                        match the_book.last().unwrap().trim().parse::<u16>() {
                            Ok(x) => unsafe {
                                match book_system.remove_book(
                                    the_book.get_unchecked(0),
                                    the_book.get_unchecked(1),
                                    x,
                                ) {
                                    Ok(_) => {
                                        fltk::dialog::message(500, 500, "Successfully removed");
                                        book_system.save();
                                        reader_base.save();
                                    }

                                    Err(_) => alert(500, 500, "The Book isn't found"),
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

/// Function that changes title
/// of all simple books and TheBook.
/// If you have mistakes in input,
/// program will let you know

pub fn change_title(book_system: &mut BookSystem, reader_base: &mut ReaderBase, app: &App) {
    let (s2, r2) = app::channel();
    let mut inp = Input3::new("Change Title", "Title", "Author", "Pages");

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    let book_params = inp.set_input();
                    inp.hide();

                    if let Ok(book) = book_params {
                        let (s3, r3) = app::channel();
                        let mut get_title = Input1::new("New Title", "New Title");
                        get_title.show();
                        (*get_title.ok).borrow_mut().emit(s3, true);

                        while app.wait() {
                            if let Some(msg) = r3.recv() {
                                match msg {
                                    true => {
                                        let title_param = get_title.set_input();
                                        get_title.hide();

                                        if let Ok(new_title) = title_param {
                                            match book.last().unwrap().trim().parse::<u16>() {
                                                Ok(a) => unsafe {
                                                    match book_system.change_title(
                                                        book.get_unchecked(0),
                                                        book.get_unchecked(1),
                                                        a,
                                                        new_title.first().unwrap().clone(),
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

                                                        Err(0) => alert(500, 500, "The Book isn't found"),
                                                        Err(_) => alert(500, 500, "Book with same parameters already exists")
                                                    }
                                                },

                                                Err(_) => {
                                                    alert(500, 500, "Pages input error");
                                                    println!(
                                                        "{:?}",
                                                        book.last().unwrap().trim().parse::<u16>()
                                                    )
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
    let mut inp = Input3::new("Change Title", "Title", "Author", "Pages");

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    let book_params = inp.set_input();
                    inp.hide();

                    if let Ok(book) = book_params {
                        let (s3, r3) = app::channel();
                        let mut get_author = Input1::new("New Author", "New Author");
                        get_author.show();
                        (*get_author.ok).borrow_mut().emit(s3, true);

                        while app.wait() {
                            if let Some(msg) = r3.recv() {
                                match msg {
                                    true => {
                                        let author_param = get_author.set_input();
                                        get_author.hide();

                                        if let Ok(new_author) = author_param {
                                            match book.last().unwrap().trim().parse::<u16>() {
                                                Ok(a) => unsafe {
                                                    match book_system.change_author(
                                                        book.get_unchecked(0),
                                                        book.get_unchecked(1),
                                                        a,
                                                        new_author.first().unwrap().clone(),
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

                                                        Err(0) => alert(500, 500, "The Book isn't found"),
                                                        Err(_) => alert(500, 500, "Book with same parameters already exists")
                                                    }
                                                },

                                                Err(_) => {
                                                    alert(500, 500, "Pages input error");
                                                    println!(
                                                        "{:?}",
                                                        book.last().unwrap().trim().parse::<u16>()
                                                    )
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
    let mut inp = Input3::new("Change Title", "Title", "Author", "Pages");

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    let book_params = inp.set_input();
                    inp.hide();

                    if let Ok(book) = book_params {
                        let (s3, r3) = app::channel();
                        let mut get_pages =
                            Input1::new("New Amount of Pages", "New Amount of Pages");
                        get_pages.show();
                        (*get_pages.ok).borrow_mut().emit(s3, true);

                        while app.wait() {
                            if let Some(msg) = r3.recv() {
                                match msg {
                                    true => {
                                        let pages_param = get_pages.set_input();
                                        get_pages.hide();

                                        if let Ok(new_pages) = pages_param {
                                            match book.last().unwrap().trim().parse::<u16>() {
                                                Ok(a) => unsafe {
                                                    match book_system.change_pages(
                                                        book.get_unchecked(0),
                                                        book.get_unchecked(1),
                                                        a,
                                                        new_pages.first().unwrap().clone(),
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

                                                        Err(0) => alert(500, 500, "New amount of pages input error"),
                                                        Err(1) => alert(500, 500, "The Book isn't found"),
                                                        Err(_) => alert(500, 500, "Book with same parameters already exists")
                                                    }
                                                },

                                                Err(_) => {
                                                    alert(500, 500, "Pages input error");
                                                    println!(
                                                        "{:?}",
                                                        book.last().unwrap().trim().parse::<u16>()
                                                    )
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

pub fn book_info(book_system: &mut BookSystem, app: &App) {
    let (s2, r2) = app::channel();
    let mut inp = Input3::new("Find Book", "Title", "Author", "Pages");

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
                                        (*book_system.books.get_unchecked(ind))
                                            .borrow_mut()
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
