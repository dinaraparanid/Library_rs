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

/// Changing title of already known book

#[inline]
pub(crate) fn change_title_simple(
    ind: usize,
    book_system: &mut BookSystem,
    reader_base: &mut ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) {
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
            match msg {
                true => {
                    get_title.hide();

                    if let Ok(mut new_title) = get_title.set_input() {
                        unsafe {
                            if new_title.get_unchecked(0).is_empty() {
                                alert(
                                    500,
                                    500,
                                    match lang {
                                        Lang::English => "'New title' is empty",
                                        Lang::Russian => "'Новое название' пусто",
                                    },
                                );
                                caretaker.pop();
                                return;
                            } else {
                                match book_system.change_title(ind, new_title.pop().unwrap()) {
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
                                        return;
                                    }

                                    Err(_) => {
                                        alert(500, 500, match lang {
											Lang::English => "Book with same parameters already exists",
											Lang::Russian => "Книга с предложенными параметрами уже сущетвует",
										});
                                        caretaker.pop();
                                        return;
                                    }
                                }
                            }
                        }
                    }
                }

                false => (),
            }
        } else if !get_title.shown() {
            caretaker.pop();
            return;
        }
    }
}

/// Changing author of already known book

#[inline]
pub(crate) fn change_author_simple(
    ind: usize,
    book_system: &mut BookSystem,
    reader_base: &mut ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) {
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
            match msg {
                true => {
                    get_author.hide();

                    if let Ok(mut new_author) = get_author.set_input() {
                        unsafe {
                            if new_author.get_unchecked(0).is_empty() {
                                alert(
                                    500,
                                    500,
                                    match lang {
                                        Lang::English => "'New author' is empty",
                                        Lang::Russian => "'Новый автор' пусто",
                                    },
                                );
                                caretaker.pop();
                                return;
                            } else {
                                match book_system.change_author(ind, new_author.pop().unwrap()) {
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
                                        return;
                                    }

                                    Err(_) => {
                                        alert(500, 500, match lang {
											Lang::English => "Book with same parameters already exists",
											Lang::Russian => "Книга с предложенными параметрами уже существует",
										});
                                        caretaker.pop();
                                        return;
                                    }
                                }
                            }
                        }
                    }
                }

                false => (),
            }
        } else if !get_author.shown() {
            caretaker.pop();
            return;
        }
    }
}

/// Changing amount of pages of already known book

#[inline]
pub(crate) fn change_pages_simple(
    ind: usize,
    book_system: &mut BookSystem,
    reader_base: &mut ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) {
    let (s3, r3) = app::channel();
    let mut get_pages = Input1::<IntInput>::new(
        match lang {
            Lang::English => "New Amount of Pages",
            Lang::Russian => "Новое Количество страниц",
        },
        match lang {
            Lang::English => "New Amount of Pages",
            Lang::Russian => "Новое Количество Страниц",
        },
    );

    caretaker.add_memento(reader_base, book_system, genres);

    get_pages.show();
    (*get_pages.ok).borrow_mut().emit(s3, true);

    while app.wait() {
        if let Some(msg) = r3.recv() {
            match msg {
                true => {
                    get_pages.hide();

                    if let Ok(mut new_pages) = get_pages.set_input() {
                        unsafe {
                            if new_pages.get_unchecked(0).is_empty() {
                                alert(
                                    500,
                                    500,
                                    match lang {
                                        Lang::English => "'New amount of pages' is empty",
                                        Lang::Russian => "'Новое количество страниц' пусто",
                                    },
                                );
                                caretaker.pop();
                                return;
                            } else {
                                match book_system.change_pages(ind, new_pages.pop().unwrap()) {
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
                                        return;
                                    }

                                    Err(0) => {
                                        alert(500, 500, match lang {
											Lang::English => "'New amount of pages' input error",
											Lang::Russian => "Некорректный ввод для 'Нового количества странциц'",
										});
                                        caretaker.pop();
                                        return;
                                    }

                                    Err(_) => {
                                        alert(500, 500, match lang {
											Lang::English => "Book with same parameters already exists",
											Lang::Russian => "Книга с предложенными параметрами уже существует",
										});
                                        caretaker.pop();
                                        return;
                                    }
                                }
                            }
                        }
                    }
                }

                false => (),
            }
        } else if !get_pages.shown() {
            caretaker.pop();
            return;
        }
    }
}

#[inline]
pub(crate) fn change_location_simple(
    t_ind: usize,
    s_ind: usize,
    book_system: &mut BookSystem,
    reader_base: &mut ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) {
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
            match msg {
                true => {
                    get_loc.hide();

                    if let Ok(mut new_loc) = get_loc.set_input() {
                        unsafe {
                            if new_loc.get_unchecked(0).is_empty() {
                                alert(
                                    500,
                                    500,
                                    match lang {
                                        Lang::English => "'New cabinet' is empty",
                                        Lang::Russian => "'Новый шкаф' пусто",
                                    },
                                );
                                caretaker.pop();
                                return;
                            } else if new_loc.get_unchecked(1).is_empty() {
                                alert(
                                    500,
                                    500,
                                    match lang {
                                        Lang::English => "'New shelf' is empty",
                                        Lang::Russian => "'Новая полка' пусто",
                                    },
                                );
                                caretaker.pop();
                                return;
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
                                        reader_base.save();
                                        return;
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
                                        caretaker.pop();
                                        return;
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
                                        caretaker.pop();
                                        return;
                                    }

                                    Err(2) => {
                                        alert(
                                            500,
                                            500,
                                            match lang {
                                                Lang::English => "Book is not found",
                                                Lang::Russian => "Книга не найдена",
                                            },
                                        );
                                        caretaker.pop();
                                        return;
                                    }

                                    Err(_) => {
                                        alert(
                                            500,
                                            500,
                                            match lang {
                                                Lang::English => "Book's number is incorrect",
                                                Lang::Russian => "Номер книги некорректен",
                                            },
                                        );
                                        caretaker.pop();
                                        return;
                                    }
                                }
                            }
                        }
                    }
                }

                false => (),
            }
        } else if !get_loc.shown() {
            caretaker.pop();
            return;
        }
    }
}
