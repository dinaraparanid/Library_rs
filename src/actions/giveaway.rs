use crate::actions::book::{check_book, empty_inp_book};
use crate::actions::read::{check_reader, empty_inp_reader};
use crate::book::{Book, BookSystem, Date};
use crate::change_menu::*;
use crate::reader::ReaderBase;
use fltk::app;
use fltk::app::{channel, App};
use fltk::dialog::alert;
use fltk::input::*;
use fltk::WidgetExt;
use std::borrow::Borrow;
use std::num::ParseIntError;

/// Function that gives book to reader.
/// It requires you to input
/// info about reader, book and return date.
/// If you have mistakes in input,
/// program will let you know

pub fn give_book(reader_base: &mut ReaderBase, book_system: &mut BookSystem, app: &App) {
    let (s2, r2) = fltk::app::channel();
    let mut inp = Input4::<Input, Input, Input, IntInput>::new(
        "Find Reader",
        "First Name",
        "Second Name",
        "Middle Name",
        "Age",
    );

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    let reader_params = inp.set_input();
                    inp.hide();

                    if let Ok(reader) = reader_params {
                        let rind;
                        match check_reader(reader_base, &reader) {
                            Ok(x) => rind = x,
                            Err(_) => return,
                        }

                        let (s3, r3) = fltk::app::channel();
                        let mut inp2 = Input3::<Input, Input, IntInput>::new(
                            "Find Book",
                            "Title",
                            "Author",
                            "Pages",
                        );

                        inp2.show();
                        (*inp2.ok).borrow_mut().emit(s3, true);

                        while app.wait() {
                            if let Some(msg) = r3.recv() {
                                match msg {
                                    true => {
                                        let book_params = inp2.set_input();
                                        inp2.hide();

                                        if let Ok(book) = book_params {
                                            unsafe {
                                                let bind;

                                                match check_book(book_system, &book) {
                                                    Ok(x) => bind = x,
                                                    Err(_) => return,
                                                }

                                                let (s4, r4) = fltk::app::channel();
                                                let mut inp3 =
                                                    Input3::<IntInput, IntInput, IntInput>::new(
                                                        "Set Return Date",
                                                        "Day",
                                                        "Month (number)",
                                                        "Year",
                                                    );

                                                inp3.show();
                                                (*inp3.ok).borrow_mut().emit(s4, true);

                                                while app.wait() {
                                                    if let Some(mes) = r4.recv() {
                                                        match mes {
                                                            true => {
                                                                let date_params = inp3.set_input();
                                                                inp3.hide();

                                                                if let Ok(dat) = date_params {
                                                                    match dat
                                                                        .get_unchecked(0)
                                                                        .trim()
                                                                        .parse::<u8>()
                                                                    {
                                                                        Ok(day) => {
                                                                            match dat
									                                            .get_unchecked(1)
									                                            .trim()
									                                            .parse::<u8>() {
									                                            Ok(month) => {
										                                            match dat
											                                            .get_unchecked(2)
											                                            .trim()
											                                            .parse::<u16>() {
											                                            Ok(year) => {
												                                            match Date::new(
													                                            day,
													                                            month,
													                                            year) {
													                                            Err(_) => alert(
														                                            500,
														                                            500,
														                                            "Incorrect Date"
													                                            ),

													                                            Ok(date) => {
														                                            let simple_book = (*book_system
															                                            .books
															                                            .get_unchecked(bind))
															                                            .borrow_mut()
															                                            .get_unused();

														                                            if simple_book
															                                            != (*book_system
															                                            .books
															                                            .get_unchecked(bind))
															                                            .borrow_mut()
															                                            .books
															                                            .len()
														                                            {
															                                            if (**reader_base
																                                            .readers
																                                            .get_unchecked(rind))
																                                            .borrow()
																                                            .reading
																                                            .is_some() {
																                                            alert(
																	                                            500,
																	                                            500,
																	                                            "This reader is already reading another book"
																                                            );
																                                            return;
															                                            }

															                                            (*reader_base
																                                            .readers
																                                            .get_unchecked(rind))
																                                            .borrow_mut()
																                                            .start_reading(
																	                                            (*book_system
																		                                            .books
																		                                            .get_unchecked(bind))
																		                                            .borrow_mut()
																		                                            .books
																		                                            .get_unchecked(simple_book),
																                                            );

															                                            (*(*book_system
																                                            .books
																                                            .get_unchecked(bind))
																                                            .borrow_mut()
																                                            .books
																                                            .get_unchecked(simple_book))
																                                            .borrow_mut()
																                                            .start_reading(
																	                                            reader_base.readers
																		                                            .get_unchecked(rind),
																	                                            date,
																                                            );

															                                            fltk::dialog::message(
																                                            500,
																                                            500,
																                                            "Book successfully given to reader"
															                                            );

															                                            book_system.save();
															                                            reader_base.save();
														                                            } else {
															                                            alert(500, 500, "There are no free books");
														                                            }
													                                            }
												                                            }
											                                            }

											                                            Err(_) => {
												                                            alert(
													                                            500,
													                                            500,
													                                            "Year input error"
												                                            );

												                                            println!("{:?}",
												                                                     dat.get_unchecked(2)
												                                                        .trim()
												                                                        .parse::<u16>()
												                                            );
											                                            }
										                                            }
									                                            }

									                                            Err(_) => {
										                                            alert(
											                                            500,
											                                            500, "\
									                                                                    Month input error"
										                                            );

										                                            println!("{:?}",
										                                                     dat.get_unchecked(1)
										                                                        .trim()
										                                                        .parse::<u8>()
										                                            );
									                                            }
								                                            }
                                                                        }

                                                                        Err(_) => {
                                                                            alert(
                                                                                500,
                                                                                500,
                                                                                "Day input error",
                                                                            );

                                                                            println!(
                                                                                "{:?}",
                                                                                dat.get_unchecked(
                                                                                    0
                                                                                )
                                                                                .trim()
                                                                                .parse::<u8>()
                                                                            );
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            false => (),
                                                        }
                                                    } else if !inp3.shown() {
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    false => (),
                                }
                            } else if !inp2.shown() {
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

/// Function that gives book to reader.
/// It requires you to input
/// info about reader, book and return date.
/// If you have mistakes in input,
/// program will let you know

pub fn get_book(reader_base: &mut ReaderBase, book_system: &mut BookSystem, app: &App) {
    let (s2, r2) = fltk::app::channel();
    let mut inp = Input4::<Input, Input, Input, IntInput>::new(
        "Find Reader",
        "First Name",
        "Second Name",
        "Middle Name",
        "Age",
    );

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    let reader_params = inp.set_input();
                    inp.hide();

                    if let Ok(reader) = reader_params {
                        let rind;

                        match check_reader(reader_base, &reader) {
                            Ok(x) => rind = x,
                            Err(_) => return,
                        }

                        let (s3, r3) = fltk::app::channel();
                        let mut inp2 = Input3::<Input, Input, IntInput>::new(
                            "Find Book",
                            "Title",
                            "Author",
                            "Pages",
                        );

                        inp2.show();
                        (*inp2.ok).borrow_mut().emit(s3, true);

                        while app.wait() {
                            if let Some(msg) = r3.recv() {
                                match msg {
                                    true => {
                                        let book_params = inp2.set_input();
                                        inp2.hide();

                                        if let Ok(book) = book_params {
                                            let bind;

                                            match check_book(book_system, &book) {
                                                Ok(x) => bind = x,
                                                Err(_) => return,
                                            }

                                            unsafe {
                                                let simple =
                                                    (*book_system.books.get_unchecked(bind))
                                                        .borrow_mut()
                                                        .find_by_reader(
                                                            reader_base.readers.get_unchecked(rind),
                                                        );

                                                if simple
                                                    != (*book_system.books.get_unchecked(bind))
                                                        .borrow_mut()
                                                        .books
                                                        .len()
                                                {
                                                    (*reader_base.readers.get_unchecked_mut(rind))
                                                        .borrow_mut()
                                                        .finish_reading();

                                                    match (*(*book_system
                                                        .books
                                                        .get_unchecked(bind))
                                                    .borrow_mut()
                                                    .books
                                                    .get_unchecked(simple))
                                                    .borrow_mut()
                                                    .finish_reading()
                                                    {
                                                        Ok(_) => fltk::dialog::message(
                                                            500,
                                                            500,
                                                            "Book is returned",
                                                        ),

                                                        Err(_) => fltk::dialog::message(
                                                            500,
                                                            500,
                                                            "Book is returned, but reader is late",
                                                        ),
                                                    }
                                                    book_system.save();
                                                } else {
                                                    alert(
                                                        500,
                                                        500,
                                                        "This reader wasn't reading this book",
                                                    );
                                                }
                                            }
                                        }
                                    }
                                    false => (),
                                }
                            } else if !inp2.shown() {
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
