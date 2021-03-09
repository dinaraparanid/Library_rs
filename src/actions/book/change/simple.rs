extern crate fltk;

use crate::{
    books::{book_sys::BookSystem, genres::Genres},
    change::{input1::Input1, input2::Input2, Inputable},
    reading::read_base::ReaderBase,
    restore::caretaker::Caretaker,
    Lang,
};

use fltk::{
    app,
    app::App,
    dialog::alert,
    input::{Input, IntInput},
    prelude::*,
};

/// Changing title
/// of already known the book
/// (and all simple books)

#[inline]
pub(crate) fn change_title_simple(
    ind: usize,
    book_system: &mut BookSystem,
    reader_base: &mut ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) -> Option<String> {
    let (s3, r3) = app::channel();
    let mut get_title = Input1::<Input>::new(
        match lang {
            Lang::English => "New Title",
            Lang::Russian => "Новое название",
        },
        match lang {
            Lang::English => "New Title",
            Lang::Russian => "Новое название",
        },
    );

    caretaker.add_memento(reader_base, book_system, genres);

    get_title.show();
    (*get_title.ok).borrow_mut().emit(s3, true);

    while app.wait() {
        if let Some(msg) = r3.recv() {
            if msg {
                get_title.hide();

                if let Ok(mut new_title) = get_title.set_input() {
                    unsafe {
                        return if new_title.get_unchecked(0).is_empty() {
                            alert(
                                500,
                                500,
                                match lang {
                                    Lang::English => "'New title' is empty",
                                    Lang::Russian => "'Новое название' пусто",
                                },
                            );
                            caretaker.pop().unwrap();
                            None
                        } else {
                            match book_system.change_title(ind, new_title.first().unwrap().clone())
                            {
                                Ok(_) => {
                                    fltk::dialog::message(
                                        500,
                                        500,
                                        match lang {
                                            Lang::English => "Successfully changed",
                                            Lang::Russian => "Успешно изменено",
                                        },
                                    );

                                    book_system.save();
                                    reader_base.save();
                                    Some(new_title.pop().unwrap())
                                }

                                Err(_) => {
                                    alert(
                                        500,
                                        500,
                                        match lang {
                                            Lang::English => {
                                                "Book with same parameters already exists"
                                            }
                                            Lang::Russian => {
                                                "Книга с предложенными параметрами уже сущетвует"
                                            }
                                        },
                                    );
                                    caretaker.pop().unwrap();
                                    None
                                }
                            }
                        };
                    }
                }
            }
            break;
        } else if !get_title.shown() {
            caretaker.pop().unwrap();
            return None;
        }
    }

    None
}

/// Changing author
/// of already known book
/// (and all simple books)

#[inline]
pub(crate) fn change_author_simple(
    ind: usize,
    book_system: &mut BookSystem,
    reader_base: &mut ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) -> Option<String> {
    let (s3, r3) = app::channel();
    let mut get_author = Input1::<Input>::new(
        match lang {
            Lang::English => "New Author",
            Lang::Russian => "Новый автор",
        },
        match lang {
            Lang::English => "New Author",
            Lang::Russian => "Новый автор",
        },
    );

    caretaker.add_memento(reader_base, book_system, genres);

    get_author.show();
    (*get_author.ok).borrow_mut().emit(s3, true);

    while app.wait() {
        if let Some(msg) = r3.recv() {
            if msg {
                get_author.hide();

                if let Ok(mut new_author) = get_author.set_input() {
                    unsafe {
                        return if new_author.get_unchecked(0).is_empty() {
                            alert(
                                500,
                                500,
                                match lang {
                                    Lang::English => "'New author' is empty",
                                    Lang::Russian => "'Новый автор' пусто",
                                },
                            );
                            caretaker.pop().unwrap();
                            None
                        } else {
                            match book_system
                                .change_author(ind, new_author.first().unwrap().clone())
                            {
                                Ok(_) => {
                                    fltk::dialog::message(
                                        500,
                                        500,
                                        match lang {
                                            Lang::English => "Successfully changed",
                                            Lang::Russian => "Успешно изменено",
                                        },
                                    );

                                    book_system.save();
                                    reader_base.save();
                                    Some(new_author.pop().unwrap())
                                }

                                Err(_) => {
                                    alert(
                                        500,
                                        500,
                                        match lang {
                                            Lang::English => {
                                                "Book with same parameters already exists"
                                            }
                                            Lang::Russian => {
                                                "Книга с предложенными параметрами уже существует"
                                            }
                                        },
                                    );
                                    caretaker.pop().unwrap();
                                    None
                                }
                            }
                        };
                    }
                }
            }
            break;
        } else if !get_author.shown() {
            caretaker.pop().unwrap();
            return None;
        }
    }

    None
}

/// Changing amount of pages
/// of already known book
/// (and all simple books)

#[inline]
pub(crate) fn change_pages_simple(
    ind: usize,
    book_system: &mut BookSystem,
    reader_base: &mut ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) -> Option<String> {
    let (s3, r3) = app::channel();
    let mut get_pages = Input1::<IntInput>::new(
        match lang {
            Lang::English => "New Amount of Pages",
            Lang::Russian => "Новое Кол-во страниц",
        },
        match lang {
            Lang::English => "New Amount of Pages",
            Lang::Russian => "Новое Кол-во Страниц",
        },
    );

    caretaker.add_memento(reader_base, book_system, genres);

    get_pages.show();
    (*get_pages.ok).borrow_mut().emit(s3, true);

    while app.wait() {
        if let Some(msg) = r3.recv() {
            if msg {
                get_pages.hide();

                if let Ok(mut new_pages) = get_pages.set_input() {
                    unsafe {
                        return if new_pages.get_unchecked(0).is_empty() {
                            alert(
                                500,
                                500,
                                match lang {
                                    Lang::English => "'New amount of pages' is empty",
                                    Lang::Russian => "'Новое Кол-во страниц' пусто",
                                },
                            );
                            caretaker.pop().unwrap();
                            None
                        } else {
                            match book_system.change_pages(ind, new_pages.first().unwrap().clone())
                            {
                                Ok(_) => {
                                    fltk::dialog::message(
                                        500,
                                        500,
                                        match lang {
                                            Lang::English => "Successfully changed",
                                            Lang::Russian => "Успешно изменено",
                                        },
                                    );

                                    book_system.save();
                                    reader_base.save();
                                    Some(new_pages.pop().unwrap())
                                }

                                Err(0) => {
                                    alert(
                                        500,
                                        500,
                                        match lang {
                                            Lang::English => "'New amount of pages' input error",
                                            Lang::Russian => {
                                                "Некорректный ввод для 'Нового количества странциц'"
                                            }
                                        },
                                    );
                                    caretaker.pop().unwrap();
                                    None
                                }

                                Err(_) => {
                                    alert(
                                        500,
                                        500,
                                        match lang {
                                            Lang::English => {
                                                "Book with same parameters already exists"
                                            }
                                            Lang::Russian => {
                                                "Книга с предложенными параметрами уже существует"
                                            }
                                        },
                                    );
                                    caretaker.pop().unwrap();
                                    None
                                }
                            }
                        };
                    }
                }
            }
        } else if !get_pages.shown() {
            caretaker.pop();
            return None;
        }
    }

    None
}

/// Changing location
/// (cabinet and shelf)
/// of already known book
/// (and all simple books)

#[inline]
pub(crate) fn change_location_simple(
    t_ind: usize,
    s_ind: usize,
    book_system: &mut BookSystem,
    reader_base: &ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) -> Option<(u16, u8)> {
    let (s3, r3) = app::channel();
    let mut get_loc = Input2::<IntInput, IntInput>::new(
        match lang {
            Lang::English => "New Location",
            Lang::Russian => "Перемещение Книги",
        },
        match lang {
            Lang::English => "New Cabinet",
            Lang::Russian => "Новый Шкаф",
        },
        match lang {
            Lang::English => "New Shelf",
            Lang::Russian => "Новая Полка",
        },
    );

    caretaker.add_memento(reader_base, book_system, genres);

    get_loc.show();
    (*get_loc.ok).borrow_mut().emit(s3, true);

    while app.wait() {
        if let Some(msg) = r3.recv() {
            if msg {
                get_loc.hide();

                if let Ok(mut new_loc) = get_loc.set_input() {
                    unsafe {
                        return if new_loc.get_unchecked(0).is_empty() {
                            alert(
                                500,
                                500,
                                match lang {
                                    Lang::English => "'New cabinet' is empty",
                                    Lang::Russian => "'Новый шкаф' пусто",
                                },
                            );
                            caretaker.pop().unwrap();
                            None
                        } else if new_loc.get_unchecked(1).is_empty() {
                            alert(
                                500,
                                500,
                                match lang {
                                    Lang::English => "'New shelf' is empty",
                                    Lang::Russian => "'Новая полка' пусто",
                                },
                            );
                            caretaker.pop().unwrap();
                            None
                        } else {
                            match book_system.change_location(
                                t_ind,
                                s_ind,
                                new_loc.swap_remove(0),
                                new_loc.pop().unwrap(),
                            ) {
                                Ok(_) => {
                                    fltk::dialog::message(
                                        500,
                                        500,
                                        match lang {
                                            Lang::English => "Successfully changed",
                                            Lang::Russian => "Успешно изменено",
                                        },
                                    );

                                    book_system.save();

                                    Some((
                                        (**(**book_system.books.get_unchecked(t_ind))
                                            .borrow()
                                            .books
                                            .get_unchecked(s_ind - 1))
                                        .borrow()
                                        .cabinet,
                                        (**(**book_system.books.get_unchecked(t_ind))
                                            .borrow()
                                            .books
                                            .get_unchecked(s_ind - 1))
                                        .borrow()
                                        .shelf,
                                    ))
                                }

                                Err(0) => {
                                    alert(
                                        500,
                                        500,
                                        match lang {
                                            Lang::English => "'New cabinet' input error",
                                            Lang::Russian => "Ошибка ввода 'Нового шкафа'",
                                        },
                                    );
                                    caretaker.pop().unwrap();
                                    None
                                }

                                Err(1) => {
                                    alert(
                                        500,
                                        500,
                                        match lang {
                                            Lang::English => "'New shelf' input error",
                                            Lang::Russian => "Ошибка ввода 'Новой полки'",
                                        },
                                    );
                                    caretaker.pop().unwrap();
                                    None
                                }

                                Err(_) => {
                                    alert(
                                        500,
                                        500,
                                        match lang {
                                            Lang::English => "Book's number is incorrect",
                                            Lang::Russian => "Номер книги не корректен",
                                        },
                                    );
                                    caretaker.pop().unwrap();
                                    None
                                }
                            }
                        };
                    }
                }
            }
            break;
        } else if !get_loc.shown() {
            caretaker.pop().unwrap();
            return None;
        }
    }
    None
}
