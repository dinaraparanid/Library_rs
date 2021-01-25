extern crate fltk;
use self::fltk::browser::CheckBrowser;
use crate::{
    actions::book::check_book,
    books::{book_sys::BookSystem, genres::Genres},
    change::{input1::Input1, input3::Input3, Inputable},
};
use fltk::{
    app,
    app::App,
    dialog::alert,
    input::{Input, IntInput},
    menu::Choice,
    prelude::*,
    window::SingleWindow,
};
use std::collections::HashSet;

/// Function that adds new genre.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn add_genre(genres: &mut Genres, app: &App) {
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
pub fn remove_genre(genres: &mut Genres, app: &App) {
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
pub fn customize_book_genre(genres: &Genres, book_system: &mut BookSystem, app: &App) {
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
