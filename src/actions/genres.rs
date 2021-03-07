extern crate fltk;

use crate::{
    actions::{
        book::utils::check_book,
        tables::{cell_book3, draw_data},
    },
    books::{book_sys::BookSystem, genres::Genres},
    change::{input1::Input1, input3::Input3, Inputable},
    reading::read_base::ReaderBase,
    restore::caretaker::Caretaker,
    Lang,
};

use fltk::{
    app,
    app::App,
    browser::CheckBrowser,
    dialog::alert,
    draw,
    input::{Input, IntInput},
    prelude::*,
    table,
    table::Table,
    tree::{Tree, TreeItem},
    window::SingleWindow,
};

use std::{cmp::max, collections::HashSet};

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
    lang: Lang,
) {
    let (s2, r2) = app::channel();
    let mut inp = Input1::<Input>::new(
        match lang {
            Lang::English => "Add Genre",
            Lang::Russian => "Добавить жанр",
        },
        match lang {
            Lang::English => "New Genre",
            Lang::Russian => "Новый жанр",
        },
    );

    caretaker.add_memento(reader_base, book_system, genres);

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            if message {
                inp.hide();

                if let Ok(genre) = inp.set_input() {
                    if genre.first().unwrap().is_empty() {
                        alert(
                            500,
                            500,
                            match lang {
                                Lang::English => "'New genre' is empty",
                                Lang::Russian => "'Новый жанр' пусто",
                            },
                        );
                        caretaker.pop().unwrap();
                        return;
                    } else {
                        genres.add(genre.first().unwrap().clone());
                        fltk::dialog::message(
                            500,
                            500,
                            match lang {
                                Lang::English => "Successfully added",
                                Lang::Russian => "Успешно добавлено",
                            },
                        );
                        genres.save();
                    }
                }
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
    book_system: &mut BookSystem,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) {
    let (s2, r2) = app::channel();
    let mut inp = Input1::<Input>::new(
        match lang {
            Lang::English => "Remove Genre",
            Lang::Russian => "Удалить жанр",
        },
        match lang {
            Lang::English => "Genre's title",
            Lang::Russian => "Название жанра",
        },
    );

    caretaker.add_memento(reader_base, book_system, genres);

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            if message {
                inp.hide();

                if let Ok(genre) = inp.set_input() {
                    if genre.first().unwrap().is_empty() {
                        alert(
                            500,
                            500,
                            match lang {
                                Lang::English => "'Genre's title' is empty",
                                Lang::Russian => "'Название жанра' пусто",
                            },
                        );
                        caretaker.pop().unwrap();
                        return;
                    } else {
                        genres.remove(genre.first().unwrap());

                        book_system.iter_mut().for_each(|b| {
                            if {
                                let check = (**b).borrow().genres.is_some();
                                check
                            } {
                                (**b)
                                    .borrow_mut()
                                    .genres
                                    .as_mut()
                                    .unwrap()
                                    .remove(genre.first().unwrap());

                                if {
                                    let check = (**b).borrow().genres.as_ref().unwrap().is_empty();
                                    check
                                } {
                                    (**b).borrow_mut().genres = None
                                }
                            }
                        });

                        fltk::dialog::message(
                            500,
                            500,
                            match lang {
                                Lang::English => "Successfully removed",
                                Lang::Russian => "Успешно удалено",
                            },
                        );
                        genres.save();
                    }
                }
            }
            return;
        } else if !inp.shown() {
            return;
        }
    }
}

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

/// Function that changes genres
/// of all simple books and TheBook.
/// You can choose as much
/// genres as you need

#[inline]
pub fn customize_book_genre(
    genres: &Genres,
    book_system: &mut BookSystem,
    reader_base: &ReaderBase,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) {
    let (s2, r2) = app::channel();
    let mut inp = Input3::<Input, Input, IntInput>::new(
        match lang {
            Lang::English => "Customize book's genres",
            Lang::Russian => "Изменить жанры книги",
        },
        match lang {
            Lang::English => "Title",
            Lang::Russian => "Название",
        },
        match lang {
            Lang::English => "Author",
            Lang::Russian => "Автор",
        },
        match lang {
            Lang::English => "Amount of Pages",
            Lang::Russian => "Кол-во страниц",
        },
    );

    caretaker.add_memento(reader_base, book_system, genres);

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            if message {
                inp.hide();

                if let Ok(book) = inp.set_input() {
                    if let Ok(index) = check_book(book_system, &book, lang) {
                        customize_book_genre_simple(
                            index,
                            genres,
                            book_system,
                            caretaker,
                            app,
                            lang,
                        )
                    }
                }
            }
            break;
        } else if !inp.shown() {
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

/// **DEPRECATED**
///
/// Used before.
/// Consider **using all_genres() instead**,
/// it's more representable
///
/// Function that shows
/// all books with specific genre

#[allow(dead_code)]
#[deprecated(note = "Used before. Consider using all_genres() instead, it's more representable")]
fn find_by_genre(book_system: &BookSystem, app: &App, lang: Lang) {
    let (s, r) = app::channel();
    let mut inp = Input1::<Input>::new(
        match lang {
            Lang::English => "Input Genre",
            Lang::Russian => "Ввод Жанра",
        },
        match lang {
            Lang::English => "Input Genre",
            Lang::Russian => "Введите Жанр",
        },
    );

    inp.show();
    (*inp.ok).borrow_mut().emit(s, true);

    while app.wait() {
        if let Some(message) = r.recv() {
            if message {
                inp.hide();

                if let Ok(genre) = inp.set_input() {
                    let mut wind = SingleWindow::new(
                        500,
                        500,
                        300,
                        400,
                        match lang {
                            Lang::English => "Books with spec genre",
                            Lang::Russian => "Книги с искомым жанром",
                        },
                    );

                    let mut book_table = Table::new(0, 0, 300, 400, "");
                    let mut find = vec![];

                    book_system.iter().for_each(|x| {
                        if (**x).borrow().genres.is_some()
                            && (**x)
                                .borrow()
                                .genres
                                .as_ref()
                                .unwrap()
                                .contains(genre.first().unwrap().as_str())
                        {
                            find.push((
                                (**x).borrow().title.clone(),
                                (**x).borrow().author.clone(),
                                (**x).borrow().pages.clone(),
                            ))
                        }
                    });

                    book_table.set_rows(max(20, find.len() as u32));

                    book_table.set_cols(1);
                    book_table.set_col_width_all(300);
                    book_table.end();

                    book_table.draw_cell2(move |t, ctx, row, col, x, y, w, h| match ctx {
                        table::TableContext::StartPage => draw::set_font(Font::Helvetica, 14),

                        table::TableContext::Cell => {
                            let gen = cell_book3(row, &find, lang);
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
            }
            break;
        } else if !inp.shown() {
            break;
        }
    }
}

/// Function that sorts
/// all books by genres

#[inline]
pub fn all_genres(
    genres: &Genres,
    book_system: &BookSystem,
    app: &App,
    lang: Lang,
) -> Option<TreeItem> {
    let mut wind = SingleWindow::new(
        500,
        500,
        300,
        400,
        match lang {
            Lang::English => "All Books with Genres",
            Lang::Russian => "Все Книги с Жанрами",
        },
    );

    let mut tree = Tree::new(0, 0, 300, 400, "");
    tree.set_root_label(match lang {
        Lang::English => "Genres",
        Lang::Russian => "Жанры",
    });

    genres.iter().for_each(|g| {
        tree.add(g.as_str()).unwrap();
        find_by_genre_simple(g, book_system)
            .into_iter()
            .for_each(|b| {
                tree.add(
                    format!(
                        "{}/{} {} {} {}",
                        g,
                        b.0,
                        b.1,
                        b.2,
                        match lang {
                            Lang::English => "pages",
                            Lang::Russian => "страниц",
                        }
                    )
                    .as_str(),
                )
                .unwrap();
            })
    });

    let no_genre = book_system
        .iter()
        .filter(|b| (***b).borrow().genres.is_none())
        .map(|b| {
            format!(
                "{} {} {} {}",
                (**b).borrow().title,
                (**b).borrow().author,
                (**b).borrow().pages,
                match lang {
                    Lang::English => "pages",
                    Lang::Russian => "страниц",
                }
            )
        })
        .collect::<Vec<_>>();

    if !no_genre.is_empty() {
        tree.add(match lang {
            Lang::English => "Without Genres",
            Lang::Russian => "Без Жанров",
        })
        .unwrap();

        no_genre.into_iter().for_each(|b| {
            tree.add(
                format!(
                    "{}/{}",
                    match lang {
                        Lang::English => "Without Genres",
                        Lang::Russian => "Без Жанров",
                    },
                    b
                )
                .as_str(),
            )
            .unwrap();
        });
    }

    wind.end();
    wind.show();

    while app.wait() {
        if let Some(item) = tree.set_item_clicked() {
            if !item.has_children() {
                wind.hide();
                return Some(item);
            } else {
                continue;
            }
        } else if !wind.shown() {
            wind.hide();
            return None;
        }
    }

    wind.hide();
    None
}
