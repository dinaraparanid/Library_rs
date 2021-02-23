extern crate fltk;

use crate::{
    actions::{read::get_book_ind, tables::*},
    books::{book::Book, book_sys::BookSystem, genres::Genres},
    change::{input1::Input1, input2::Input2, input3::Input3, Inputable},
    reading::read_base::ReaderBase,
    restore::caretaker::Caretaker,
    Lang,
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
pub(crate) fn empty_inp_book(inp: &Vec<String>, lang: Lang) -> bool {
    unsafe {
        return if inp.get_unchecked(0).is_empty() {
            alert(
                500,
                500,
                match lang {
                    Lang::English => "'Title' is empty",
                    Lang::Russian => "'Название' пусто",
                },
            );
            true
        } else if inp.get_unchecked(1).is_empty() {
            alert(
                500,
                500,
                match lang {
                    Lang::English => "'Author' is empty",
                    Lang::Russian => "'Автор' пусто",
                },
            );
            true
        } else if inp.get_unchecked(2).is_empty() {
            alert(
                500,
                500,
                match lang {
                    Lang::English => "'Amount of pages' is empty",
                    Lang::Russian => "'Количество страниц' пусто",
                },
            );
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
pub(crate) fn check_book(
    book_system: &BookSystem,
    books: &Vec<String>,
    lang: Lang,
) -> Result<usize, ()> {
    unsafe {
        if empty_inp_book(books, lang) {
            return Err(());
        }

        return match books.get_unchecked(2).trim().parse::<u16>() {
            Ok(x) => match book_system.find_book(books.get_unchecked(0), books.get_unchecked(1), x)
            {
                Some(i) => Ok(i),

                None => {
                    alert(
                        500,
                        500,
                        match lang {
                            Lang::English => "Book isn't found",
                            Lang::Russian => "Книга не найдена",
                        },
                    );
                    Err(())
                }
            },

            Err(_) => {
                alert(
                    500,
                    500,
                    match lang {
                        Lang::English => "Amount of Pages input error",
                        Lang::Russian => "Ошибка ввода количества страниц",
                    },
                );
                Err(())
            }
        };
    }
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
fn change_author_simple(
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
fn change_pages_simple(
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
fn change_location_simple(
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

/// Removes already known the book

#[inline]
fn remove_the_book_simple(
    ind: usize,
    book_system: &mut BookSystem,
    reader_base: &mut ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    lang: Lang,
) {
    caretaker.add_memento(reader_base, book_system, genres);

    match book_system.remove_book(ind) {
        Ok(_) => {
            fltk::dialog::message(
                500,
                500,
                match lang {
                    Lang::English => "Successfully removed",
                    Lang::Russian => "Успешно удалено",
                },
            );

            book_system.save();
            reader_base.save();
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
        }
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
    lang: Lang,
) {
    let (s3, r3) = app::channel();
    let mut get_amount = Input1::<IntInput>::new(
        match lang {
            Lang::English => "Books amount",
            Lang::Russian => "Количество книг",
        },
        match lang {
            Lang::English => "Amount of books to add",
            Lang::Russian => "Количество добавляемых книг",
        },
    );

    caretaker.add_memento(reader_base, book_system, genres);

    get_amount.show();
    (*get_amount.ok).borrow_mut().emit(s3, true);

    while app.wait() {
        if let Some(msg) = r3.recv() {
            match msg {
                true => {
                    get_amount.hide();

                    if let Ok(amount) = get_amount.set_input() {
                        match amount.first().unwrap().trim().parse::<usize>() {
                            Ok(x) => match book_system.add_books(ind, x, app, lang) {
                                Ok(_) => {
                                    fltk::dialog::message(
                                        500,
                                        500,
                                        match lang {
                                            Lang::English => "Successfully added",
                                            Lang::Russian => "Успешно добавлено",
                                        },
                                    );
                                    book_system.save();
                                }

                                Err(_) => {
                                    alert(
                                        500,
                                        500,
                                        match lang {
                                            Lang::English => "Too much books",
                                            Lang::Russian => "Слишком много книг",
                                        },
                                    );
                                    caretaker.pop();
                                    return;
                                }
                            },

                            Err(_) => {
                                alert(
                                    500,
                                    500,
                                    match lang {
                                        Lang::English => "'Amount of books' input error",
                                        Lang::Russian => "Ошибка ввода количества книг",
                                    },
                                );
                                caretaker.pop();
                                return;
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
    lang: Lang,
) {
    let (s3, r3) = app::channel();
    let mut get_ind = Input1::<IntInput>::new(
        match lang {
            Lang::English => "Book's number",
            Lang::Russian => "Номер книги",
        },
        match lang {
            Lang::English => "Book's number",
            Lang::Russian => "Номер книги",
        },
    );

    caretaker.add_memento(reader_base, book_system, genres);

    get_ind.show();
    (*get_ind.ok).borrow_mut().emit(s3, true);

    while app.wait() {
        if let Some(msg) = r3.recv() {
            match msg {
                true => {
                    get_ind.hide();

                    if let Ok(ind) = get_ind.set_input() {
                        match ind.first().unwrap().trim().parse::<usize>() {
                            Ok(x) => {
                                if x == 0 {
                                    alert(
                                        500,
                                        500,
                                        match lang {
                                            Lang::English => "Incorrect number of book",
                                            Lang::Russian => "Некорректный номер книги",
                                        },
                                    );
                                    caretaker.pop();
                                    return;
                                } else {
                                    match book_system.remove_one_book(index, x - 1) {
                                        Ok(_) => {
                                            fltk::dialog::message(
                                                500,
                                                500,
                                                match lang {
                                                    Lang::English => "Successfully removed",
                                                    Lang::Russian => "Успешно удалено",
                                                },
                                            );
                                            book_system.save();
                                            reader_base.save();
                                        }

                                        Err(_) => {
                                            alert(
                                                500,
                                                500,
                                                match lang {
                                                    Lang::English => "Incorrect number of book",
                                                    Lang::Russian => "Некорректный номер книги",
                                                },
                                            );
                                            caretaker.pop();
                                            return;
                                        }
                                    }
                                }
                            }

                            Err(_) => {
                                alert(
                                    500,
                                    500,
                                    match lang {
                                        Lang::English => "Book's number input error",
                                        Lang::Russian => "Ошибка ввода номера книги",
                                    },
                                );
                                caretaker.pop();
                                return;
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
    lang: Lang,
) {
    caretaker.add_memento(reader_base, book_system, genres);

    unsafe {
        book_system.remove_one_book_unchecked(index, s_index);
    }

    fltk::dialog::message(
        500,
        500,
        match lang {
            Lang::English => "Successfully removed",
            Lang::Russian => "Успешно удалено",
        },
    );

    book_system.save();
    reader_base.save();
}

/// Adds known The Book

#[inline]
fn add_book_simple(book_system: &mut BookSystem, the_book: &Vec<String>, app: &App, lang: Lang) {
    let (s, r) = app::channel();
    let mut am = Input1::<IntInput>::new(
        match lang {
            Lang::English => "Amount of Books",
            Lang::Russian => "Количество Книг",
        },
        match lang {
            Lang::English => "Set amount of books",
            Lang::Russian => "Укажите количество книг",
        },
    );

    am.show();
    (*am.ok).borrow_mut().emit(s, true);

    while app.wait() {
        if let Some(mes) = r.recv() {
            match mes {
                true => {
                    am.hide();

                    if let Ok(amount) = am.set_input() {
                        match amount.first().unwrap().trim().parse::<usize>() {
                            Ok(amount) => match the_book.last().unwrap().trim().parse::<u16>() {
                                Ok(x) => unsafe {
                                    match book_system.add_book(
                                            the_book.get_unchecked(0).clone(),
                                            the_book.get_unchecked(1).clone(),
                                            x,
                                            amount,
                                            app,
                                            lang
                                        ) {
                                            Ok(_) => {
                                                fltk::dialog::message(500, 500, match lang {
                                                    Lang::English => "Successfully added",
                                                    Lang::Russian => "Успешно добавлено",
                                                });
                                                book_system.save();
                                            }

                                            Err(_) => {
                                                alert(500,
                                                      500,
                                                      match lang {
                                                          Lang::English => "Book with same parameters already exists",
                                                          Lang::Russian => "Книга с предложенными параметрами уже сузествует",
                                                      }
                                                )
                                            }
                                        }
                                },

                                Err(_) => {
                                    alert(
                                        500,
                                        500,
                                        match lang {
                                            Lang::English => "Incorrect 'Amount of Pages' input",
                                            Lang::Russian => "Некорретный ввод количества страниц",
                                        },
                                    );
                                }
                            },

                            Err(_) => {
                                alert(
                                    500,
                                    500,
                                    match lang {
                                        Lang::English => "'Amount of Pages' input error",
                                        Lang::Russian => "Ошибка ввода количества страниц",
                                    },
                                );
                            }
                        }
                    }
                }
                false => (),
            }
        } else if !am.shown() {
            break;
        }
    }
}

/// Function that gives information
/// about already known simple book

pub fn book_info_simple(
    book: Option<Weak<RefCell<Book>>>,
    book_system: &BookSystem,
    app: &App,
    lang: Lang,
) {
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

            let mut table1 = VGrid::new(0, 0, 908, 170, "");
            table1.set_params(6, 1, 1);

            table1.add(&Frame::new(
                10,
                50,
                100,
                30,
                format!(
                    "{}: {}",
                    match lang {
                        Lang::English => "Title",
                        Lang::Russian => "Название",
                    },
                    (*b.upgrade().unwrap()).borrow().title
                )
                .as_str(),
            ));

            table1.add(&Frame::new(
                30,
                50,
                100,
                30,
                format!(
                    "{}: {}",
                    match lang {
                        Lang::English => "Author",
                        Lang::Russian => "Автор",
                    },
                    (*b.upgrade().unwrap()).borrow().author
                )
                .as_str(),
            ));

            table1.add(&Frame::new(
                50,
                50,
                100,
                30,
                format!(
                    "{}: {}",
                    match lang {
                        Lang::English => "Amount of Pages",
                        Lang::Russian => "Количество страниц",
                    },
                    (*b.upgrade().unwrap()).borrow().pages,
                )
                .as_str(),
            ));

            table1.add(&Frame::new(
                70,
                50,
                100,
                30,
                format!(
                    "{}: {}",
                    match lang {
                        Lang::English => "Order Number",
                        Lang::Russian => "Порядковый номер",
                    },
                    get_book_ind(book_system, b.upgrade().unwrap().as_ptr()),
                )
                .as_str(),
            ));

            table1.add(&Frame::new(
                90,
                50,
                100,
                30,
                format!(
                    "{}: {}",
                    match lang {
                        Lang::English => "Now is Read By",
                        Lang::Russian => "В данный момент читается",
                    },
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
                        match lang {
                            Lang::English => "None",
                            Lang::Russian => "Никем",
                        }
                        .to_string()
                    }
                )
                .as_str(),
            ));

            table1.add(&Frame::new(
                110,
                50,
                100,
                30,
                format!(
                    "{}: {}",
                    match lang {
                        Lang::English => "Cabinet",
                        Lang::Russian => "Шкаф",
                    },
                    (*b.upgrade().unwrap()).borrow().cabinet,
                )
                .as_str(),
            ));

            table1.add(&Frame::new(
                130,
                50,
                100,
                30,
                format!(
                    "{}: {}",
                    match lang {
                        Lang::English => "Shelf",
                        Lang::Russian => "Полка",
                    },
                    (*b.upgrade().unwrap()).borrow().shelf,
                )
                .as_str(),
            ));

            table1.add(&Frame::new(
                150,
                50,
                100,
                30,
                format!(
                    "{}:",
                    match lang {
                        Lang::English => "All Readers",
                        Lang::Russian => "Все читатели",
                    }
                )
                .as_str(),
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
                            0 => match lang {
                                Lang::English => "1-st Name",
                                Lang::Russian => "Имя",
                            },

                            1 => match lang {
                                Lang::English => "2-nd Name",
                                Lang::Russian => "Фамилия",
                            },

                            2 => match lang {
                                Lang::English => "Middle Name",
                                Lang::Russian => "Отчество",
                            },

                            3 => match lang {
                                Lang::English => "Age",
                                Lang::Russian => "Возраст",
                            },

                            4 => match lang {
                                Lang::English => "Start Date",
                                Lang::Russian => "Дата начала",
                            },

                            _ => match lang {
                                Lang::English => "Finish Date",
                                Lang::Russian => "Дедлайн",
                            },
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
    lang: Lang,
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
                "{}: {}",
                match lang {
                    Lang::English => "Title",
                    Lang::Russian => "Название",
                },
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
                "{}: {}",
                match lang {
                    Lang::English => "Author",
                    Lang::Russian => "Автор",
                },
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
                "{}: {}",
                match lang {
                    Lang::English => "Amount of Pages",
                    Lang::Russian => "Количество страниц",
                },
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
                "{}: {}",
                match lang {
                    Lang::English => "Amount of books",
                    Lang::Russian => "Количество книг",
                },
                (**book_system.books.get_unchecked(ind))
                    .borrow()
                    .books
                    .len()
            )
            .as_str(),
        ));

        table.add(&Frame::new(
            90,
            50,
            100,
            30,
            format!(
                "{}:",
                match lang {
                    Lang::English => "Genres",
                    Lang::Russian => "Жанры",
                },
            )
            .as_str(),
        ));

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
                let gen = cell_genre(row, &b, lang);
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
        match lang {
            Lang::English => "&Change/Change title\t",
            Lang::Russian => "&Изменить/Изменить название\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageTheBook::ChangeTitle,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Change/Change author\t",
            Lang::Russian => "&Изменить/Изменить автора\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageTheBook::ChangeAuthor,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Change/Change amount of pages\t",
            Lang::Russian => "&Изменить/Изменить количество страниц\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageTheBook::ChangePages,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Remove/Remove all books\t",
            Lang::Russian => "&Удалить/Удалить все книги\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageTheBook::RemoveThis,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Remove/Remove one book\t",
            Lang::Russian => "&Удалить/Удалить одну книгу\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageTheBook::RemoveSimple,
    );

    menu.add_emit(
        match lang {
            Lang::English => "&Add books\t",
            Lang::Russian => "&Добавить книги\t",
        },
        Shortcut::empty(),
        MenuFlag::Normal,
        s,
        MessageTheBook::AddSimple,
    );

    wind.show();

    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                MessageTheBook::ChangeAuthor => change_author_simple(
                    ind,
                    book_system,
                    reader_base,
                    genres,
                    caretaker,
                    app,
                    lang,
                ),

                MessageTheBook::ChangeTitle => {
                    change_title_simple(ind, book_system, reader_base, genres, caretaker, app, lang)
                }

                MessageTheBook::ChangePages => {
                    change_pages_simple(ind, book_system, reader_base, genres, caretaker, app, lang)
                }

                MessageTheBook::RemoveSimple => {
                    remove_book_simple(ind, book_system, reader_base, genres, caretaker, app, lang)
                }

                MessageTheBook::AddSimple => {
                    add_books_simple(ind, book_system, reader_base, genres, caretaker, app, lang)
                }

                MessageTheBook::RemoveThis => {
                    remove_the_book_simple(ind, book_system, reader_base, genres, caretaker, lang)
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
    lang: Lang,
) {
    let (s2, r2) = app::channel();
    let mut inp = Input3::<Input, Input, IntInput>::new(
        match lang {
            Lang::English => "Add Books",
            Lang::Russian => "Добавить книги",
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
            Lang::Russian => "Количество страниц",
        },
    );
    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    inp.hide();

                    if let Ok(books) = inp.set_input() {
                        unsafe {
                            if empty_inp_book(&books, lang) {
                                return;
                            }

                            return match books.get_unchecked(2).trim().parse::<u16>() {
                                Ok(x) => match book_system.find_book(
                                    books.get_unchecked(0),
                                    books.get_unchecked(1),
                                    x,
                                ) {
                                    Some(i) => add_books_simple(
                                        i,
                                        book_system,
                                        reader_base,
                                        genres,
                                        caretaker,
                                        app,
                                        lang,
                                    ),
                                    None => add_book_simple(book_system, &books, app, lang),
                                },

                                Err(_) => {
                                    alert(
                                        500,
                                        500,
                                        match lang {
                                            Lang::English => "Amount of Pages input error",
                                            Lang::Russian => "Ошибка ввода количества страниц",
                                        },
                                    );
                                }
                            };
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

#[inline]
pub fn remove_book(
    book_system: &mut BookSystem,
    reader_base: &mut ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) {
    let (s2, r2) = app::channel();
    let mut inp = Input3::<Input, Input, IntInput>::new(
        match lang {
            Lang::English => "Remove Book",
            Lang::Russian => "Удалить книгу",
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
            Lang::Russian => "Количество страниц",
        },
    );

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    inp.hide();

                    if let Ok(book) = inp.set_input() {
                        if let Ok(index) = check_book(book_system, &book, lang) {
                            remove_book_simple(
                                index,
                                book_system,
                                reader_base,
                                genres,
                                caretaker,
                                app,
                                lang,
                            );
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

/// **DEPRECATED**
///
/// Function that add new book and with some simple books.
/// If you have mistakes in input,
/// program will let you know

#[inline]
#[deprecated]
fn add_book(
    book_system: &mut BookSystem,
    reader_base: &ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) {
    let (s2, r2) = app::channel();
    let mut inp = Input3::<Input, Input, IntInput>::new(
        match lang {
            Lang::English => "Add New Book",
            Lang::Russian => "Добавить новую книгу",
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
            Lang::Russian => "Количество страниц",
        },
    );

    caretaker.add_memento(reader_base, book_system, genres);

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    inp.hide();

                    if let Ok(the_book) = inp.set_input() {
                        if empty_inp_book(&the_book, lang) {
                            return;
                        }

                        let (s, r) = app::channel();
                        let mut am = Input1::<IntInput>::new(
                            match lang {
                                Lang::English => "Amount of Books",
                                Lang::Russian => "Количество Книг",
                            },
                            match lang {
                                Lang::English => "Set amount of books",
                                Lang::Russian => "Укажите количество книг",
                            },
                        );

                        am.show();
                        (*am.ok).borrow_mut().emit(s, true);

                        while app.wait() {
                            if let Some(mes) = r.recv() {
                                match mes {
                                    true => {
                                        am.hide();

                                        if let Ok(amount) = am.set_input() {
                                            match amount.first().unwrap().trim().parse::<usize>() {
                                                Ok(amount) => {
                                                    match the_book
                                                        .last()
                                                        .unwrap()
                                                        .trim()
                                                        .parse::<u16>()
                                                    {
                                                        Ok(x) => unsafe {
                                                            match book_system.add_book(
                                                                the_book.get_unchecked(0).clone(),
                                                                the_book.get_unchecked(1).clone(),
                                                                x,
                                                                amount,
                                                                app,
                                                                lang,
                                                            ) {
                                                                Ok(_) => {
                                                                    fltk::dialog::message(500, 500, match lang {
                                                                        Lang::English => "Successfully added",
                                                                        Lang::Russian => "Успешно добавлено",
                                                                    });
                                                                    book_system.save();
                                                                }

                                                                Err(_) => {
                                                                    alert(500,
                                                                          500,
                                                                          match lang {
                                                                              Lang::English => "Book with same parameters already exists",
                                                                              Lang::Russian => concat!("Книга с предложенными", 
                                                                              " параметрами уже существует"),
                                                                          }
                                                                    )
                                                                }
                                                            }
                                                        },

                                                        Err(_) => {
                                                            alert(
                                                                500,
                                                                500,
                                                                match lang {
                                                                      Lang::English => "Incorrect 'Amount of Pages' input",
                                                                      Lang::Russian => "Некорретный ввод количества страниц",
                                                                  }
                                                            );
                                                        }
                                                    }
                                                }

                                                Err(_) => {
                                                    alert(
                                                        500,
                                                        500,
                                                        match lang {
                                                            Lang::English => {
                                                                "'Amount of Pages' input error"
                                                            }
                                                            Lang::Russian => {
                                                                "Ошибка ввода количества страниц"
                                                            }
                                                        },
                                                    );
                                                }
                                            }
                                        }
                                    }
                                    false => (),
                                }
                            } else if !am.shown() {
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
    lang: Lang,
) {
    let (s2, r2) = app::channel();
    let mut inp = Input3::<Input, Input, IntInput>::new(
        match lang {
            Lang::English => "Remove Books",
            Lang::Russian => "Удалить книги",
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
            Lang::Russian => "Количество страниц",
        },
    );

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    inp.hide();

                    if let Ok(the_book) = inp.set_input() {
                        if let Ok(index) = check_book(book_system, &the_book, lang) {
                            remove_the_book_simple(
                                index,
                                book_system,
                                reader_base,
                                genres,
                                caretaker,
                                lang,
                            );
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

#[inline]
pub fn change_title(
    book_system: &mut BookSystem,
    reader_base: &mut ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) {
    let (s2, r2) = app::channel();
    let mut inp = Input3::<Input, Input, IntInput>::new(
        match lang {
            Lang::English => "Change title",
            Lang::Russian => "Изменить название",
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
            Lang::Russian => "Количество страниц",
        },
    );

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    inp.hide();

                    if let Ok(book) = inp.set_input() {
                        if let Ok(index) = check_book(book_system, &book, lang) {
                            change_title_simple(
                                index,
                                book_system,
                                reader_base,
                                genres,
                                caretaker,
                                app,
                                lang,
                            );
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

#[inline]
pub fn change_author(
    book_system: &mut BookSystem,
    reader_base: &mut ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) {
    let (s2, r2) = app::channel();
    let mut inp = Input3::<Input, Input, IntInput>::new(
        match lang {
            Lang::English => "Change author",
            Lang::Russian => "Изменить автора",
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
            Lang::Russian => "Количество страниц",
        },
    );

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    inp.hide();

                    if let Ok(book) = inp.set_input() {
                        if let Ok(index) = check_book(book_system, &book, lang) {
                            change_author_simple(
                                index,
                                book_system,
                                reader_base,
                                genres,
                                caretaker,
                                app,
                                lang,
                            );
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

#[inline]
pub fn change_pages(
    book_system: &mut BookSystem,
    reader_base: &mut ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) {
    let (s2, r2) = app::channel();
    let mut inp = Input3::<Input, Input, IntInput>::new(
        match lang {
            Lang::English => "Change amount of pages",
            Lang::Russian => "Изменить количество страниц",
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
            Lang::Russian => "Количество страниц",
        },
    );

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    inp.hide();

                    if let Ok(book) = inp.set_input() {
                        if let Ok(index) = check_book(book_system, &book, lang) {
                            change_pages_simple(
                                index,
                                book_system,
                                reader_base,
                                genres,
                                caretaker,
                                app,
                                lang,
                            );
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

/// Function that changes location
/// of simple book.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn change_location(
    book_system: &mut BookSystem,
    reader_base: &mut ReaderBase,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) {
    let (s2, r2) = app::channel();
    let mut inp = Input3::<Input, Input, IntInput>::new(
        match lang {
            Lang::English => "Change book's location",
            Lang::Russian => "Переместить книгу",
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
            Lang::Russian => "Количество страниц",
        },
    );

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    inp.hide();

                    if let Ok(book) = inp.set_input() {
                        if let Ok(t_ind) = check_book(book_system, &book, lang) {
                            let (s, r) = app::channel();
                            let mut inp2 = Input1::<IntInput>::new(
                                match lang {
                                    Lang::English => "Book's number",
                                    Lang::Russian => "Номер книги",
                                },
                                match lang {
                                    Lang::English => "Book's number",
                                    Lang::Russian => "Номер книги",
                                },
                            );

                            inp2.show();
                            (*inp2.ok).borrow_mut().emit(s, true);

                            while app.wait() {
                                if let Some(mes) = r.recv() {
                                    match mes {
                                        true => {
                                            inp2.hide();

                                            if let Ok(ind) = inp2.set_input() {
                                                match ind.first().unwrap().trim().parse::<usize>() {
                                                    Err(_) => alert(
                                                        500,
                                                        500,
                                                        match lang {
                                                            Lang::English => {
                                                                "Incorrect book's number"
                                                            }
                                                            Lang::Russian => {
                                                                "Некорректный номер книги"
                                                            }
                                                        },
                                                    ),

                                                    Ok(s_ind) => change_location_simple(
                                                        t_ind,
                                                        s_ind,
                                                        book_system,
                                                        reader_base,
                                                        genres,
                                                        caretaker,
                                                        app,
                                                        lang,
                                                    ),
                                                }
                                            }
                                        }
                                        false => (),
                                    }
                                }
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

#[inline]
pub fn the_book_info(
    book_system: Rc<RefCell<BookSystem>>,
    reader_base: Rc<RefCell<ReaderBase>>,
    genres: &Genres,
    caretaker: &mut Caretaker,
    app: &App,
    lang: Lang,
) {
    let (s2, r2) = app::channel();
    let mut inp = Input3::<Input, Input, IntInput>::new(
        match lang {
            Lang::English => "Find book",
            Lang::Russian => "Найти книгу",
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
            Lang::Russian => "Количество страниц",
        },
    );

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    inp.hide();

                    if let Ok(the_book) = inp.set_input() {
                        let index;

                        match check_book(&(*book_system).borrow(), &the_book, lang) {
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
                            lang,
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
pub fn book_info(book_system: &BookSystem, app: &App, lang: Lang) {
    let (s2, r2) = app::channel();
    let mut inp = Input3::<Input, Input, IntInput>::new(
        match lang {
            Lang::English => "Find book",
            Lang::Russian => "Найти книгу",
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
            Lang::Russian => "Количество страниц",
        },
    );

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    inp.hide();

                    if let Ok(the_book) = inp.set_input() {
                        let index;

                        match check_book(book_system, &the_book, lang) {
                            Ok(x) => index = x,
                            Err(_) => return,
                        }

                        let (s, r) = app::channel();
                        let mut inp2 = Input1::<IntInput>::new(
                            match lang {
                                Lang::English => "Number of Book",
                                Lang::Russian => "Номер книги",
                            },
                            match lang {
                                Lang::English => "Number of Book",
                                Lang::Russian => "Номер книги",
                            },
                        );

                        inp2.show();
                        (*inp2.ok).borrow_mut().emit(s, true);

                        while app.wait() {
                            if let Some(msg) = r.recv() {
                                match msg {
                                    true => {
                                        inp2.hide();

                                        if let Ok(bind_v) = inp2.set_input() {
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
                                                    || bind == 0
                                                {
                                                    alert(
                                                        500,
                                                        500,
                                                        match lang {
                                                            Lang::English => {
                                                                "Incorrect number of book"
                                                            }
                                                            Lang::Russian => {
                                                                "Некорректный номер книги"
                                                            }
                                                        },
                                                    );
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
                                                    lang,
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
pub fn show_all_books(book_system: Rc<RefCell<BookSystem>>, lang: Lang) {
    let mut wind = SingleWindow::default()
        .with_label(match lang {
            Lang::English => "All Books",
            Lang::Russian => "Все книги",
        })
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
                    0 => match lang {
                        Lang::English => "Title",
                        Lang::Russian => "Название",
                    },

                    1 => match lang {
                        Lang::English => "Author",
                        Lang::Russian => "Автор",
                    },

                    2 => match lang {
                        Lang::English => "Amount of Pages",
                        Lang::Russian => "Количество страниц",
                    },

                    _ => match lang {
                        Lang::English => "Amount of available books",
                        Lang::Russian => "Количество доступных книг",
                    },
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
