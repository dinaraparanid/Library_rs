extern crate fltk;
use self::fltk::menu::MenuFlag;
use crate::actions::tables::*;
use crate::books::book::Book;
use crate::books::book_sys::BookSystem;
use crate::change::input1::Input1;
use crate::change::input4::Input4;
use crate::change::Inputable;
use crate::reading::read_base::ReaderBase;
use fltk::app::App;
use fltk::dialog::alert;
use fltk::frame::Frame;
use fltk::group::VGrid;
use fltk::input::*;
use fltk::menu::MenuBar;
use fltk::prelude::*;
use fltk::table::Table;
use fltk::window::SingleWindow;
use fltk::{app, draw};
use std::cell::RefCell;
use std::cmp::max;
use std::num::ParseIntError;
use std::rc::Rc;

/// Function that checks if input was empty

#[inline]
pub(crate) fn empty_inp_reader(inp: &Vec<String>) -> bool {
    unsafe {
        return if inp.get_unchecked(0).is_empty() {
            alert(500, 500, "Name is empty");
            true
        } else if inp.get_unchecked(1).is_empty() {
            alert(500, 500, "Family is empty");
            true
        } else if inp.get_unchecked(2).is_empty() {
            alert(500, 500, "Father is empty");
            true
        } else if inp.get_unchecked(2).is_empty() {
            alert(500, 500, "Age is empty");
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
pub(crate) fn check_reader(reader_base: &ReaderBase, reader: &Vec<String>) -> Result<usize, ()> {
    let age;
    let ind;

    unsafe {
        if empty_inp_reader(reader) {
            return Err(());
        }

        match reader.get_unchecked(3).trim().parse::<u8>() {
            Ok(x) => age = x,
            Err(_) => {
                alert(500, 500, "Age input error");
                return Err(());
            }
        }

        ind = reader_base.find_reader(
            reader.get_unchecked(0),
            reader.get_unchecked(1),
            reader.get_unchecked(2),
            age,
        );
    }

    return match ind {
        None => {
            alert(500, 500, "Reader isn't found");
            Err(())
        }

        Some(i) => Ok(i),
    };
}

/// Function that returns index of simple book.
/// Panics if book is not in vec of books.

#[inline]
pub(crate) fn get_book_ind(book_system: &BookSystem, book: *mut Book) -> usize {
    if book.is_null() {
        panic!("nullptr in actions/read get_book_ind");
    }

    unsafe {
        match book_system.find_book(&(*book).title, &(*book).author, (*book).pages) {
            None => panic!("Index out of range"),
            Some(ind) => {
                (*(**book_system.books.get_unchecked(ind)).borrow().books)
                    .iter()
                    .position(|x| &*(**x).borrow() as *const Book == book)
                    .unwrap()
                    + 1
            }
        }
    }
}

/// Removes already known reader

#[inline]
fn remove_reader_simple(ind: usize, reader_base: &mut ReaderBase, book_system: &mut BookSystem) {
    match reader_base.remove_reader(ind) {
        Ok(_) => {
            fltk::dialog::message(500, 500, "Successfully removed");
            reader_base.save();
            book_system.save();
        }

        Err(_) => {
            alert(500, 500, "Reader not found");
        }
    }
}

/// Change name of already known reader

#[inline]
fn change_name_simple(
    ind: usize,
    reader_base: &mut ReaderBase,
    book_system: &mut BookSystem,
    app: &App,
) {
    let (s3, r3) = app::channel();
    let mut get_name = Input1::<Input>::new("New Name", "New Name");

    get_name.show();
    (*get_name.ok).borrow_mut().emit(s3, true);

    while app.wait() {
        if let Some(mes) = r3.recv() {
            match mes {
                true => {
                    let new_name_param = get_name.set_input();
                    get_name.hide();

                    if let Ok(new_name) = new_name_param {
                        unsafe {
                            match reader_base.change_name(ind, new_name.get_unchecked(0).clone()) {
                                Ok(_) => {
                                    fltk::dialog::message(500, 500, "Successfully changed");

                                    reader_base.save();
                                    book_system.save();
                                }

                                Err(0) => {
                                    alert(500, 500, "Reader not found");
                                }

                                Err(1) => {
                                    alert(500, 500, "Reader already exists");
                                }

                                Err(_) => {
                                    alert(500, 500, "New name is empty");
                                }
                            }
                        }
                    }
                }
                false => (),
            }
            break;
        } else if !get_name.shown() {
            return;
        }
    }
}

/// Change 2-nd name of already known reader

#[inline]
fn change_family_simple(
    ind: usize,
    reader_base: &mut ReaderBase,
    book_system: &mut BookSystem,
    app: &App,
) {
    let (s3, r3) = app::channel();
    let mut get_family = Input1::<Input>::new("New 2-nd Name", "New 2-nd Name");

    get_family.show();
    (*get_family.ok).borrow_mut().emit(s3, true);

    while app.wait() {
        if let Some(mes) = r3.recv() {
            match mes {
                true => {
                    let new_family_param = get_family.set_input();
                    get_family.hide();

                    if let Ok(new_family) = new_family_param {
                        unsafe {
                            match reader_base
                                .change_family(ind, new_family.get_unchecked(0).clone())
                            {
                                Ok(_) => {
                                    fltk::dialog::message(500, 500, "Successfully changed");

                                    reader_base.save();
                                    book_system.save();
                                }

                                Err(0) => {
                                    alert(500, 500, "Reader not found");
                                }

                                Err(1) => {
                                    alert(500, 500, "Reader already exists");
                                }

                                Err(_) => {
                                    alert(500, 500, "New 2-nd name is empty");
                                }
                            }
                        }
                    }
                }
                false => (),
            }
            break;
        } else if !get_family.shown() {
            return;
        }
    }
}

/// Change middle name of already known reader

#[inline]
fn change_father_simple(
    ind: usize,
    reader_base: &mut ReaderBase,
    book_system: &mut BookSystem,
    app: &App,
) {
    let (s3, r3) = app::channel();
    let mut get_father = Input1::<Input>::new("New Middle Name", "New Middle Name");

    get_father.show();
    (*get_father.ok).borrow_mut().emit(s3, true);

    while app.wait() {
        if let Some(mes) = r3.recv() {
            match mes {
                true => {
                    let new_father_param = get_father.set_input();
                    get_father.hide();

                    if let Ok(new_father) = new_father_param {
                        unsafe {
                            match reader_base
                                .change_father(ind, new_father.get_unchecked(0).clone())
                            {
                                Ok(_) => {
                                    fltk::dialog::message(500, 500, "Successfully changed");

                                    reader_base.save();
                                    book_system.save();
                                }

                                Err(0) => {
                                    alert(500, 500, "Reader not found");
                                }

                                Err(1) => {
                                    alert(500, 500, "Reader already exists");
                                }

                                Err(_) => {
                                    alert(500, 500, "New mid. name is empty");
                                }
                            }
                        }
                    }
                }
                false => (),
            }
            break;
        } else if !get_father.shown() {
            return;
        }
    }
}

/// Changes age of already known reader

#[inline]
fn change_age_simple(
    ind: usize,
    reader_base: &mut ReaderBase,
    book_system: &mut BookSystem,
    app: &App,
) {
    let (s3, r3) = app::channel();

    let mut get_age = Input1::<IntInput>::new("New Age", "New Age");
    get_age.show();
    (*get_age.ok).borrow_mut().emit(s3, true);

    while app.wait() {
        if let Some(mes) = r3.recv() {
            match mes {
                true => {
                    let new_age_param = get_age.set_input();
                    get_age.hide();

                    if let Ok(new_age) = new_age_param {
                        if new_age.first().unwrap().is_empty() {
                            alert(500, 500, "New age is empty");
                            return;
                        }

                        unsafe {
                            match reader_base.change_age(ind, new_age.get_unchecked(0).clone()) {
                                Ok(_) => {
                                    fltk::dialog::message(500, 500, "Successfully changed");

                                    reader_base.save();
                                    book_system.save();
                                }

                                Err(0) => alert(500, 500, "Age input error"),

                                Err(_) => alert(500, 500, "Reader already exists"),
                            }
                        }
                    }
                }
                false => (),
            }
            break;
        } else if !get_age.shown() {
            return;
        }
    }
}

/// Function that adds reader.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn add_reader(reader_base: &mut ReaderBase, app: &App) {
    let (s2, r2) = app::channel();

    let mut inp = Input4::<Input, Input, Input, IntInput>::new(
        "Add Reader",
        "First Name",
        "Second Names",
        "Middle Name",
        "Age",
    );

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    let new_reader_params = inp.set_input();
                    inp.hide();

                    if let Ok(reader) = new_reader_params {
                        if empty_inp_reader(&reader) {
                            return;
                        }

                        match reader.last().unwrap().trim().parse::<u8>() {
                            Ok(a) => unsafe {
                                match reader_base.add_reader(
                                    reader.get_unchecked(0).clone(),
                                    reader.get_unchecked(1).clone(),
                                    reader.get_unchecked(2).clone(),
                                    a,
                                ) {
                                    Ok(_) => {
                                        fltk::dialog::message(500, 500, "Successfully added");
                                        reader_base.save();
                                    }

                                    Err(_) => {
                                        alert(500, 500, "Reader already exists");
                                    }
                                }
                            },

                            Err(_) => {
                                alert(500, 500, "Age input error");
                                println!("{:?}", reader.last().unwrap().trim().parse::<u8>())
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

/// Function that removes reader.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn remove_reader(reader_base: &mut ReaderBase, book_system: &mut BookSystem, app: &App) {
    let (s2, r2) = app::channel();

    let mut inp = Input4::<Input, Input, Input, IntInput>::new(
        "Remove Reader",
        "First Name",
        "Second Names",
        "Middle Name",
        "Age",
    );

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    let rem_reader_params = inp.set_input();
                    inp.hide();

                    if let Ok(reader) = rem_reader_params {
                        let rind;

                        match check_reader(reader_base, &reader) {
                            Ok(x) => rind = x,
                            Err(_) => return,
                        }

                        remove_reader_simple(rind, reader_base, book_system);
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

/// Function that changes reader's name.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn change_name(reader_base: &mut ReaderBase, book_system: &mut BookSystem, app: &App) {
    let (s2, r2) = app::channel();

    let mut inp = Input4::<Input, Input, Input, IntInput>::new(
        "Change Name",
        "First Name",
        "Second Names",
        "Middle Name",
        "Age",
    );

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    let chng_reader_params = inp.set_input();
                    inp.hide();

                    if let Ok(reader) = chng_reader_params {
                        let rind;

                        match check_reader(reader_base, &reader) {
                            Ok(x) => rind = x,
                            Err(_) => return,
                        }

                        change_name_simple(rind, reader_base, book_system, app);
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

/// Function that changes reader's second name.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn change_family(reader_base: &mut ReaderBase, book_system: &mut BookSystem, app: &App) {
    let (s2, r2) = app::channel();

    let mut inp = Input4::<Input, Input, Input, IntInput>::new(
        "Change 2-nd Name",
        "First Name",
        "Second Names",
        "Middle Name",
        "Age",
    );

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    let chng_reader_params = inp.set_input();
                    inp.hide();

                    if let Ok(reader) = chng_reader_params {
                        let rind;

                        match check_reader(reader_base, &reader) {
                            Ok(x) => rind = x,
                            Err(_) => return,
                        }

                        change_family_simple(rind, reader_base, book_system, app);
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

/// Function that changes reader's middle name.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn change_father(reader_base: &mut ReaderBase, book_system: &mut BookSystem, app: &App) {
    let (s2, r2) = app::channel();

    let mut inp = Input4::<Input, Input, Input, IntInput>::new(
        "Change Middle Name",
        "First Name",
        "Second Names",
        "Middle Name",
        "Age",
    );

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    let chng_reader_params = inp.set_input();
                    inp.hide();

                    if let Ok(reader) = chng_reader_params {
                        let rind;

                        match check_reader(reader_base, &reader) {
                            Ok(x) => rind = x,
                            Err(_) => return,
                        }

                        change_father_simple(rind, reader_base, book_system, app);
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

/// Function that changes reader's age.
/// If you have mistakes in input,
/// program will let you know

#[inline]
pub fn change_age(reader_base: &mut ReaderBase, book_system: &mut BookSystem, app: &App) {
    let (s2, r2) = app::channel();

    let mut inp = Input4::<Input, Input, Input, IntInput>::new(
        "Change Age",
        "First Name",
        "Second Names",
        "Middle Name",
        "Age",
    );

    inp.show();
    (*inp.ok).borrow_mut().emit(s2, true);

    while app.wait() {
        if let Some(message) = r2.recv() {
            match message {
                true => {
                    let chng_reader_params = inp.set_input();
                    inp.hide();

                    if let Ok(reader) = chng_reader_params {
                        let rind;

                        match check_reader(reader_base, &reader) {
                            Ok(x) => rind = x,
                            Err(_) => return,
                        }

                        change_age_simple(rind, reader_base, book_system, app);
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

/// Messages for info menu

#[derive(Clone, Copy)]
enum MessageReader {
    ChangeName,
    ChangeFamily,
    ChangeFather,
    ChangeAge,
    RemoveThis,
}

/// Function that gives info about reader.
/// If you have mistakes in input,
/// program will let you know

pub fn reader_info(
    reader_base: &'static mut ReaderBase,
    book_system: &'static mut BookSystem,
    app: &App,
) {
    let (s2, r2) = app::channel();
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
                        match reader.last().unwrap().trim().parse::<u8>() {
                            Ok(x) => unsafe {
                                let ind = reader_base.find_reader(
                                    reader.get_unchecked(0),
                                    reader.get_unchecked(1),
                                    reader.get_unchecked(2),
                                    x,
                                );

                                if ind.is_none() {
                                    alert(500, 500, "Reader isn't found");
                                    return;
                                }

                                let mut wind = SingleWindow::new(
                                    800,
                                    100,
                                    570,
                                    600,
                                    format!(
                                        "{} {} {}",
                                        reader.get_unchecked(0).as_str(),
                                        reader.get_unchecked(1).as_str(),
                                        reader.get_unchecked(2).as_str()
                                    )
                                    .as_str(),
                                )
                                .center_screen();

                                let mut table1 = VGrid::new(0, 0, 570, 100, "");
                                table1.set_params(6, 1, 1);

                                table1.add(&Frame::new(
                                    10,
                                    50,
                                    100,
                                    30,
                                    format!("First Name: {}", reader.get_unchecked(0)).as_str(),
                                ));

                                table1.add(&Frame::new(
                                    30,
                                    50,
                                    100,
                                    30,
                                    format!("Second Name: {}", reader.get_unchecked(1).as_str())
                                        .as_str(),
                                ));

                                table1.add(&Frame::new(
                                    50,
                                    50,
                                    100,
                                    30,
                                    format!("Middle Name: {}", reader.get_unchecked(2)).as_str(),
                                ));

                                table1.add(&Frame::new(
                                    70,
                                    50,
                                    100,
                                    30,
                                    format!("Age: {}", x).as_str(),
                                ));

                                table1.add(&Frame::new(
                                    70,
                                    50,
                                    100,
                                    30,
                                    format!(
                                        "Reading now: {}",
                                        if (**reader_base.readers.get_unchecked(ind.unwrap()))
                                            .borrow()
                                            .reading
                                            .is_some()
                                        {
                                            (*(**reader_base.readers.get_unchecked(ind.unwrap()))
                                                .borrow()
                                                .reading
                                                .as_ref()
                                                .unwrap()
                                                .upgrade()
                                                .unwrap())
                                            .borrow()
                                            .title
                                            .clone()
                                                + " "
                                                + (*(**reader_base
                                                    .readers
                                                    .get_unchecked(ind.unwrap()))
                                                .borrow()
                                                .reading
                                                .as_ref()
                                                .unwrap()
                                                .upgrade()
                                                .unwrap())
                                                .borrow()
                                                .author
                                                .as_str()
                                                + " "
                                                + (*(**reader_base
                                                    .readers
                                                    .get_unchecked(ind.unwrap()))
                                                .borrow()
                                                .reading
                                                .as_ref()
                                                .unwrap()
                                                .upgrade()
                                                .unwrap())
                                                .borrow()
                                                .pages
                                                .to_string()
                                                .as_str()
                                                + "  ("
                                                + get_book_ind(
                                                    book_system,
                                                    (*(**reader_base
                                                        .readers
                                                        .get_unchecked(ind.unwrap()))
                                                    .borrow()
                                                    .reading
                                                    .as_ref()
                                                    .unwrap()
                                                    .upgrade()
                                                    .unwrap())
                                                    .as_ptr(),
                                                )
                                                .to_string()
                                                .as_str()
                                                + ")"
                                        } else {
                                            "None".to_string()
                                        }
                                    )
                                    .as_str(),
                                ));

                                table1.add(&Frame::new(
                                    90,
                                    50,
                                    100,
                                    30,
                                    format!("Books read by reader:").as_str(),
                                ));

                                table1.auto_layout();

                                let mut table2 = Table::new(0, 127, 570, 600, "");
                                table2.set_rows(max(
                                    30,
                                    (**reader_base.readers.get_unchecked(ind.unwrap()))
                                        .borrow()
                                        .books
                                        .len() as u32,
                                ));
                                table2.set_row_header(true);
                                table2.set_cols(4);
                                table2.set_col_header(true);
                                table2.set_col_width_all(130);
                                table2.end();

                                wind.end();

                                let mut menu = MenuBar::new(0, 0, 65, 30, "");
                                wind.add(&menu);

                                let (s, r) = app::channel();

                                menu.add_emit(
                                    "&Change/Change name\t",
                                    Shortcut::empty(),
                                    MenuFlag::Normal,
                                    s,
                                    MessageReader::ChangeName,
                                );

                                menu.add_emit(
                                    "&Change/Change family\t",
                                    Shortcut::empty(),
                                    MenuFlag::Normal,
                                    s,
                                    MessageReader::ChangeFamily,
                                );

                                menu.add_emit(
                                    "&Change/Change father\t",
                                    Shortcut::empty(),
                                    MenuFlag::Normal,
                                    s,
                                    MessageReader::ChangeFather,
                                );

                                menu.add_emit(
                                    "&Change/Change age\t",
                                    Shortcut::empty(),
                                    MenuFlag::Normal,
                                    s,
                                    MessageReader::ChangeAge,
                                );

                                menu.add_emit(
                                    "Remove this reader\t",
                                    Shortcut::empty(),
                                    MenuFlag::Normal,
                                    s,
                                    MessageReader::RemoveThis,
                                );

                                wind.show();

                                let base_ptr = reader_base as *mut ReaderBase;
                                let sys_ptr = book_system as *mut BookSystem;

                                table2.draw_cell2(move |t, ctx, row, col, x, y, w, h| match ctx {
                                    fltk::table::TableContext::StartPage => {
                                        draw::set_font(Font::Helvetica, 14)
                                    }

                                    fltk::table::TableContext::ColHeader => draw_header(
                                        &format!(
                                            "{}",
                                            match col {
                                                0 => "Title",
                                                1 => "Author",
                                                2 => "Pages",
                                                _ => "Number of book",
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
                                        &format!(
                                            "{}",
                                            cell_book2(
                                                col,
                                                row,
                                                ind.unwrap(),
                                                base_ptr.as_ref().unwrap(),
                                                sys_ptr.as_ref().unwrap()
                                            )
                                        ),
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
                                    if let Some(msg) = r.recv() {
                                        match msg {
                                            MessageReader::ChangeName => change_name_simple(
                                                ind.unwrap(),
                                                reader_base,
                                                book_system,
                                                app,
                                            ),

                                            MessageReader::ChangeFamily => change_family_simple(
                                                ind.unwrap(),
                                                reader_base,
                                                book_system,
                                                app,
                                            ),

                                            MessageReader::ChangeFather => change_father_simple(
                                                ind.unwrap(),
                                                reader_base,
                                                book_system,
                                                app,
                                            ),

                                            MessageReader::ChangeAge => change_age_simple(
                                                ind.unwrap(),
                                                reader_base,
                                                book_system,
                                                app,
                                            ),

                                            MessageReader::RemoveThis => remove_reader_simple(
                                                ind.unwrap(),
                                                reader_base,
                                                book_system,
                                            ),
                                        }
                                        return;
                                    }
                                }
                            },

                            Err(_) => {
                                alert(500, 500, "Age input error");
                                println!("{:?}", reader.last().unwrap().trim().parse::<u8>())
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
