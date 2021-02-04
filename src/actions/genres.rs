extern crate fltk;

use crate::{
    actions::{
        book::check_book,
        tables::{cell_book3, cell_genre2, draw_data},
    },
    books::{book_sys::BookSystem, genres::Genres},
    change::{input1::Input1, input3::Input3, Inputable},
    reading::read_base::ReaderBase,
    restore::caretaker::Caretaker,
};

use fltk::{
    app,
    app::App,
    browser::CheckBrowser,
    dialog::alert,
    draw,
    input::{Input, IntInput},
    menu::Choice,
    prelude::*,
    table,
    table::Table,
    window::SingleWindow,
};

use std::{borrow::Borrow, cell::RefCell, cmp::max, collections::HashSet, rc::Rc};

/// Function that adds new genre.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn add_genre(
    genres: &mut Genres,
    reader_base: &ReaderBase,
    book_system: &BookSystem,
    caretaker: &mut Caretaker,
    app: &App,
) {
    let (s2, r2) = app::channel();
    let mut inp = Input1::<Input>::new("Add Genre", "New Genre");

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    let genre_params = inp.set_input();
                    inp.hide();

                    if let Ok(genre) = genre_params {
                        if genre.first().unwrap().is_empty() {
                            alert(500, 500, "New genre is empty");
                        } else {
                            genres.add(genre.first().unwrap().clone());
                            fltk::dialog::message(500, 500, "Successfully added");
                            genres.save();
                            caretaker.add_memento(reader_base, book_system, genres);
                        }
                    }
                }
                false => (),
            }
            return;
        } else if !inp.shown() {
            return;
        }
    }
}

/// Function that removes genre.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn remove_genre(
    genres: &mut Genres,
    reader_base: &ReaderBase,
    book_system: &BookSystem,
    caretaker: &mut Caretaker,
    app: &App,
) {
    let (s2, r2) = app::channel();
    let mut inp = Input1::<Input>::new("Remove Genre", "Genre");

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    let genre_params = inp.set_input();
                    inp.hide();

                    if let Ok(genre) = genre_params {
                        if genre.first().unwrap().is_empty() {
                            alert(500, 500, "Genre is empty");
                        } else {
                            genres.remove(genre.first().unwrap());
                            fltk::dialog::message(500, 500, "Successfully removed");
                            genres.save();
                            caretaker.add_memento(reader_base, book_system, genres);
                        }
                    }
                }
                false => (),
            }
            return;
        } else if !inp.shown() {
            return;
        }
    }
}

/// Function that changes title
/// of all simple books and TheBook.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn customize_book_genre(
    genres: &Genres,
    book_system: &mut BookSystem,
    reader_base: &ReaderBase,
    caretaker: &mut Caretaker,
    app: &App,
) {
    let (s2, r2) = app::channel();
    let mut inp =
        Input3::<Input, Input, IntInput>::new("Add Genre", "Title", "Author", "Amount of Pages");

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

                        let mut wind = SingleWindow::new(
                            500,
                            100,
                            300,
                            50 * genres.genres.len() as i32,
                            "Select Genres",
                        );

                        let mut genre_choice =
                            CheckBrowser::new(0, 0, 300, 50 * genres.genres.len() as i32, "");

                        for g in &genres.genres {
                            unsafe {
                                genre_choice.add(
                                    g.as_str(),
                                    if let Some(gen) = &(**book_system.books.get_unchecked(index))
                                        .borrow_mut()
                                        .genres
                                    {
                                        if gen.contains(g) {
                                            true
                                        } else {
                                            false
                                        }
                                    } else {
                                        false
                                    },
                                );
                            }
                        }

                        wind.end();
                        wind.show();

                        while app.wait() {
                            for i in 0..genres.genres.len() {
                                if genre_choice.checked(i as i32 + 1) {
                                    unsafe {
                                        if (**book_system.books.get_unchecked(index))
                                            .borrow_mut()
                                            .genres
                                            .is_some()
                                        {
                                            (**book_system.books.get_unchecked(index))
                                                .borrow_mut()
                                                .genres
                                                .as_mut()
                                                .unwrap()
                                                .insert(
                                                    genres
                                                        .genres
                                                        .iter()
                                                        .skip(i)
                                                        .next()
                                                        .unwrap()
                                                        .clone(),
                                                );
                                        } else {
                                            (**book_system.books.get_unchecked(index))
                                                .borrow_mut()
                                                .genres = Some(HashSet::new());

                                            (**book_system.books.get_unchecked(index))
                                                .borrow_mut()
                                                .genres
                                                .as_mut()
                                                .unwrap()
                                                .insert(
                                                    genres
                                                        .genres
                                                        .iter()
                                                        .skip(i)
                                                        .next()
                                                        .unwrap()
                                                        .clone(),
                                                );
                                        }
                                    }

                                    book_system.save();
                                } else {
                                    unsafe {
                                        if (**book_system.books.get_unchecked(index))
                                            .borrow_mut()
                                            .genres
                                            .is_some()
                                        {
                                            (**book_system.books.get_unchecked(index))
                                                .borrow_mut()
                                                .genres
                                                .as_mut()
                                                .unwrap()
                                                .remove(
                                                    genres.genres.iter().skip(i).next().unwrap(),
                                                );

                                            if (**book_system.books.get_unchecked(index))
                                                .borrow()
                                                .genres
                                                .as_ref()
                                                .unwrap()
                                                .len()
                                                == 0
                                            {
                                                (**book_system.books.get_unchecked(index))
                                                    .borrow_mut()
                                                    .genres = None;
                                            }
                                        }
                                    }
                                }
                                book_system.save();
                                caretaker.add_memento(reader_base, book_system, genres);
                            }

                            if !wind.shown() {
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

#[inline]
pub(crate) fn find_by_genre_simple(genre: &String, book_system: &BookSystem) {
    let mut wind = SingleWindow::new(500, 500, 300, 400, "Books with spec genre");
    let mut book_table = Table::new(0, 0, 300, 400, "");

    let mut find = vec![];

    for x in &book_system.books {
        if (**x).borrow().genres.is_some()
            && (**x)
                .borrow()
                .genres
                .as_ref()
                .unwrap()
                .contains(genre.as_str())
        {
            find.push((
                (**x).borrow().title.clone(),
                (**x).borrow().author.clone(),
                (**x).borrow().pages.clone(),
            ))
        }
    }

    book_table.set_rows(max(20, find.len() as u32));

    book_table.set_cols(1);
    book_table.set_col_width_all(300);
    book_table.end();

    book_table.draw_cell2(move |t, ctx, row, col, x, y, w, h| match ctx {
        table::TableContext::StartPage => draw::set_font(Font::Helvetica, 14),

        table::TableContext::Cell => {
            let gen = cell_book3(row, &find);
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
    wind.show();
}

/// Function that shows
/// all books with specific genre

#[inline]
pub fn find_by_genre(book_system: &BookSystem, app: &App) {
    let (s, r) = app::channel();
    let mut inp = Input1::<Input>::new("Input Genre", "Genre");

    inp.show();
    (*inp.ok).borrow_mut().emit(s, true);

    while app.wait() {
        if let Some(message) = r.recv() {
            match message {
                true => {
                    let genre_params = inp.set_input();
                    inp.hide();

                    if let Ok(genre) = genre_params {
                        find_by_genre_simple(genre.first().unwrap(), book_system);
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

/// Function that shows
/// all books with specific genre

#[inline]
pub fn all_genres(genres: Rc<RefCell<Genres>>, book_system: &BookSystem, app: &App) {
    let mut wind = SingleWindow::new(500, 500, 300, 400, "All Genres");

    let mut tab = Table::new(0, 0, 300, 400, "");
    tab.set_rows(max(20, (*genres).borrow().genres.len() as u32));
    tab.set_cols(1);
    tab.set_col_width_all(300);
    tab.end();

    let gen = genres.clone();

    tab.draw_cell2(move |t, ctx, row, col, x, y, w, h| match ctx {
        table::TableContext::StartPage => draw::set_font(Font::Helvetica, 14),

        table::TableContext::Cell => {
            let gen = cell_genre2(row, &*(*gen).borrow());
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
    wind.show();

    while app.wait() {
        if !wind.shown() {
            return;
        }

        let len = (*genres).borrow().genres.len();
        for ind in 0..len {
            if tab.is_selected(ind as i32, 0) {
                find_by_genre_simple(
                    (*genres).borrow().genres.iter().skip(ind).next().unwrap(),
                    book_system,
                );
                tab.unset_selection();
                break;
            }
        }
    }
}
