use crate::change_menu::*;
use crate::reader::ReaderBase;
use fltk::app;
use fltk::app::App;
use fltk::dialog::alert;
use fltk::WidgetExt;

pub fn add_reader(reader_base: &mut ReaderBase, app: &App) {
    let (s2, r2) = app::channel();

    let mut inp = Input4::new(
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
                        match reader.last().unwrap().trim().parse::<u8>() {
                            Ok(a) => unsafe {
                                match reader_base.add_reader(
                                    reader.get_unchecked(0).clone(),
                                    reader.get_unchecked(1).clone(),
                                    reader.get_unchecked(2).clone(),
                                    a,
                                ) {
                                    Ok(_) => fltk::dialog::message(500, 500, "Successfully added"),

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

pub fn remove_reader(reader_base: &mut ReaderBase, app: &App) {
    let (s2, r2) = app::channel();

    let mut inp = Input4::new(
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
                        match reader.last().unwrap().trim().parse::<u8>() {
                            Ok(a) => unsafe {
                                match reader_base.remove_reader(
                                    reader.get_unchecked(0),
                                    reader.get_unchecked(1),
                                    reader.get_unchecked(2),
                                    a,
                                ) {
                                    Ok(_) => {
                                        fltk::dialog::message(500, 500, "Successfully removed")
                                    }

                                    Err(_) => {
                                        alert(500, 500, "Reader not found");
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

pub fn change_name(reader_base: &mut ReaderBase, app: &App) {
    let (s2, r2) = app::channel();

    let mut inp = Input4::new(
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
                        let (s3, r3) = app::channel();

                        let mut get_name = Input1::new("New Name", "New Name");
                        get_name.show();
                        (*get_name.ok).borrow_mut().emit(s3, true);

                        while app.wait() {
                            if let Some(mes) = r3.recv() {
                                match mes {
                                    true => {
                                        let new_name_param = get_name.set_input();
                                        get_name.hide();

                                        if let Ok(new_name) = new_name_param {
                                            match reader.last().unwrap().trim().parse::<u8>() {
                                                Ok(a) => unsafe {
                                                    match reader_base.change_name(
                                                        reader.get_unchecked(0),
                                                        reader.get_unchecked(1),
                                                        reader.get_unchecked(2),
                                                        a,
                                                        new_name.get_unchecked(0).clone(),
                                                    ) {
                                                        Ok(_) => fltk::dialog::message(
                                                            500,
                                                            500,
                                                            "Successfully changed",
                                                        ),

                                                        Err(0) => {
                                                            alert(500, 500, "Reader not found");
                                                        }

                                                        Err(1) => {
                                                            alert(
                                                                500,
                                                                500,
                                                                "Reader already exists",
                                                            );
                                                        }

                                                        Err(_) => {
                                                            alert(500, 500, "New name is empty");
                                                        }
                                                    }
                                                },

                                                Err(_) => {
                                                    alert(500, 500, "Age input error");

                                                    println!(
                                                        "{:?}",
                                                        reader.last().unwrap().trim().parse::<u8>()
                                                    )
                                                }
                                            }
                                        }
                                    }
                                    false => (),
                                }
                                break;
                            } else if !get_name.shown() {
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

pub fn change_family(reader_base: &mut ReaderBase, app: &App) {
    let (s2, r2) = app::channel();

    let mut inp = Input4::new(
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
                        let (s3, r3) = app::channel();

                        let mut get_family = Input1::new("New Second Name", "New Second Name");
                        get_family.show();
                        (*get_family.ok).borrow_mut().emit(s3, true);

                        while app.wait() {
                            if let Some(mes) = r3.recv() {
                                match mes {
                                    true => {
                                        let new_family_param = get_family.set_input();
                                        get_family.hide();

                                        if let Ok(new_family) = new_family_param {
                                            match reader.last().unwrap().trim().parse::<u8>() {
                                                Ok(a) => unsafe {
                                                    match reader_base.change_family(
                                                        reader.get_unchecked(0),
                                                        reader.get_unchecked(1),
                                                        reader.get_unchecked(2),
                                                        a,
                                                        new_family.get_unchecked(0).clone(),
                                                    ) {
                                                        Ok(_) => fltk::dialog::message(
                                                            500,
                                                            500,
                                                            "Successfully changed",
                                                        ),

                                                        Err(0) => {
                                                            alert(500, 500, "Reader isn't found");
                                                        }

                                                        Err(1) => {
                                                            alert(
                                                                500,
                                                                500,
                                                                "Reader with same parameters already exists",
                                                            );
                                                        }

                                                        Err(_) => {
                                                            alert(
                                                                500,
                                                                500,
                                                                "New second name is empty",
                                                            );
                                                        }
                                                    }
                                                },

                                                Err(_) => {
                                                    alert(500, 500, "Age input error");

                                                    println!(
                                                        "{:?}",
                                                        reader.last().unwrap().trim().parse::<u8>()
                                                    )
                                                }
                                            }
                                        }
                                    }
                                    false => (),
                                }
                                break;
                            } else if !get_family.shown() {
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

pub fn change_father(reader_base: &mut ReaderBase, app: &App) {
    let (s2, r2) = app::channel();

    let mut inp = Input4::new(
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
                        let (s3, r3) = app::channel();

                        let mut get_father = Input1::new("New Middle Name", "New Middle Name");
                        get_father.show();
                        (*get_father.ok).borrow_mut().emit(s3, true);

                        while app.wait() {
                            if let Some(mes) = r3.recv() {
                                match mes {
                                    true => {
                                        let new_father_param = get_father.set_input();
                                        get_father.hide();

                                        if let Ok(new_father) = new_father_param {
                                            match reader.last().unwrap().trim().parse::<u8>() {
                                                Ok(a) => unsafe {
                                                    match reader_base.change_father(
                                                        reader.get_unchecked(0),
                                                        reader.get_unchecked(1),
                                                        reader.get_unchecked(2),
                                                        a,
                                                        new_father.get_unchecked(0).clone(),
                                                    ) {
                                                        Ok(_) => fltk::dialog::message(
                                                            500,
                                                            500,
                                                            "Successfully changed",
                                                        ),

                                                        Err(0) => {
                                                            alert(500, 500, "Reader isn't found");
                                                        }

                                                        Err(1) => {
                                                            alert(
                                                                500,
                                                                500,
                                                                "Reader with same parameters already exists",
                                                            );
                                                        }

                                                        Err(_) => {
                                                            alert(
                                                                500,
                                                                500,
                                                                "New middle name is empty",
                                                            );
                                                        }
                                                    }
                                                },

                                                Err(_) => {
                                                    alert(500, 500, "Age input error");

                                                    println!(
                                                        "{:?}",
                                                        reader.last().unwrap().trim().parse::<u8>()
                                                    )
                                                }
                                            }
                                        }
                                    }
                                    false => (),
                                }
                                break;
                            } else if !get_father.shown() {
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

pub fn change_age(reader_base: &mut ReaderBase, app: &App) {
    let (s2, r2) = app::channel();

    let mut inp = Input4::new(
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
                        let (s3, r3) = app::channel();

                        let mut get_age = Input1::new("New Age", "New Age");
                        get_age.show();
                        (*get_age.ok).borrow_mut().emit(s3, true);

                        while app.wait() {
                            if let Some(mes) = r3.recv() {
                                match mes {
                                    true => {
                                        let new_age_param = get_age.set_input();
                                        get_age.hide();

                                        if let Ok(new_age) = new_age_param {
                                            match reader.last().unwrap().trim().parse::<u8>() {
                                                Ok(a) => unsafe {
                                                    match reader_base.change_father(
                                                        reader.get_unchecked(0),
                                                        reader.get_unchecked(1),
                                                        reader.get_unchecked(2),
                                                        a,
                                                        new_age.get_unchecked(0).clone(),
                                                    ) {
                                                        Ok(_) => fltk::dialog::message(
                                                            500,
                                                            500,
                                                            "Successfully changed",
                                                        ),

                                                        Err(0) => {
                                                            alert(500, 500, "New age input error");
                                                        }

                                                        Err(1) => {
                                                            alert(500, 500, "Reader isn't found");
                                                        }

                                                        Err(_) => {
                                                            alert(
                                                                500,
                                                                500,
                                                                "Reader with same parameters already exists",
                                                            );
                                                        }
                                                    }
                                                },

                                                Err(_) => {
                                                    alert(500, 500, "Age input error");

                                                    println!(
                                                        "{:?}",
                                                        reader.last().unwrap().trim().parse::<u8>()
                                                    )
                                                }
                                            }
                                        }
                                    }
                                    false => (),
                                }
                                break;
                            } else if !get_age.shown() {
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

pub fn reader_info(reader_base: &mut ReaderBase, app: &App) {
    unimplemented!()
}
