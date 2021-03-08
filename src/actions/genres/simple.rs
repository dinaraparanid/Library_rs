extern crate fltk;

use fltk::{app::App, browser::CheckBrowser, prelude::*, window::SingleWindow};

use crate::{
    books::{book_sys::BookSystem, genres::Genres},
    restore::caretaker::Caretaker,
    Lang,
};

use std::collections::HashSet;

/// Function that changes genres
/// of all simple books and TheBook.
/// TheBook is taking by index.
/// You can choose as much
/// genres as you need

#[inline]
pub(crate) fn customize_book_genre_simple(
    index: usize,
    genres: &Genres,
    book_system: &mut BookSystem,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) {
    let mut wind = SingleWindow::new(
        500,
        100,
        300,
        50 * genres.len() as i32,
        match lang {
            Lang::English => "Select Genres",
            Lang::Russian => "Выбрать жанры",
        },
    );

    let mut genre_choice = CheckBrowser::new(0, 0, 300, 50 * genres.len() as i32 + 30, "");

    genres.iter().for_each(|g| {
        genre_choice.add(
            g.as_str(),
            if let Some(gen) = unsafe {
                &(**book_system.books.get_unchecked(index))
                    .borrow_mut()
                    .genres
            } {
                if gen.contains(g) {
                    true
                } else {
                    false
                }
            } else {
                false
            },
        );
    });

    wind.end();
    wind.show();

    while app.wait() {
        (0..genres.len()).for_each(|i| {
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
                            .insert(genres.iter().skip(i).next().unwrap().clone());
                    } else {
                        (**book_system.books.get_unchecked(index))
                            .borrow_mut()
                            .genres = Some(HashSet::new());

                        (**book_system.books.get_unchecked(index))
                            .borrow_mut()
                            .genres
                            .as_mut()
                            .unwrap()
                            .insert(genres.iter().skip(i).next().unwrap().clone());
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
                            .remove(genres.iter().skip(i).next().unwrap());

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
        });

        if !wind.shown() {
            caretaker.pop().unwrap();
            return;
        }
    }
}

/// Function that searches
/// all book with specific genre.
/// Returns params of those books

#[inline]
pub fn find_by_genre_simple(
    genre: &String,
    book_system: &BookSystem,
) -> Vec<(String, String, u16)> {
    let mut find = vec![];

    book_system.iter().for_each(|x| {
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
    });

    find
}
