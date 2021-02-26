extern crate fltk;

use crate::{
    actions::book::utils::*,
    books::{book_sys::BookSystem, genres::Genres},
    change::{input1::Input1, input3::Input3, Inputable},
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

/// Removes already known the book

#[inline]
pub(crate) fn remove_the_book_simple(
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
            return;
        }
    }
}

/// Adds simple books to already known the book

#[inline]
pub(crate) fn add_books_simple(
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
pub(crate) fn remove_book_simple(
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
pub(crate) fn remove_book_simple2(
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
pub(crate) fn add_book_simple(
    book_system: &mut BookSystem,
    the_book: &Vec<String>,
    app: &App,
    lang: Lang,
) {
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
