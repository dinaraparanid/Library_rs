use fltk::button::Button;
use fltk::dialog::*;
use fltk::frame::Frame;
use fltk::input::{Input, IntInput};
use fltk::prelude::*;
use fltk::window::SingleWindow;
use std::cell::RefCell;
use std::rc::Rc;

/// Trait for change messages.

pub trait Inputable {
    /// Gets menu's window

    fn get_wind(&self) -> &Rc<RefCell<SingleWindow>>;

    /// Returns input by vector of strings if everything is ok
    /// else it returns error

    fn set_input(&mut self) -> Result<Vec<String>, ()>;

    /// Checks if window is shown

    #[inline]
    fn shown(&self) -> bool {
        (*self.get_wind()).borrow().shown()
    }

    /// Hides window

    #[inline]
    fn hide(&self) {
        (*self.get_wind()).borrow_mut().hide()
    }

    /// Shows window

    #[inline]
    fn show(&self) {
        (*self.get_wind()).borrow_mut().show_with_env_args()
    }
}

/// Changes one value

pub struct Input1 {
    wind: Rc<RefCell<SingleWindow>>,
    pub ok: Rc<RefCell<Button>>,
    what: Rc<RefCell<Frame>>,
    input: Rc<RefCell<Input>>,
}

/// Changes two values

pub struct Input2 {
    wind: Rc<RefCell<SingleWindow>>,
    pub ok: Rc<RefCell<Button>>,
    what1: Rc<RefCell<Frame>>,
    input1: Rc<RefCell<Input>>,
    what2: Rc<RefCell<Frame>>,
    input2: Rc<RefCell<Input>>,
}

/// Changes three values

pub struct Input3 {
    wind: Rc<RefCell<SingleWindow>>,
    pub ok: Rc<RefCell<Button>>,
    what1: Rc<RefCell<Frame>>,
    input1: Rc<RefCell<Input>>,
    what2: Rc<RefCell<Frame>>,
    input2: Rc<RefCell<Input>>,
    what3: Rc<RefCell<Frame>>,
    input3: Rc<RefCell<Input>>,
}

/// Changes four values

pub struct Input4 {
    wind: Rc<RefCell<SingleWindow>>,
    pub ok: Rc<RefCell<Button>>,
    what1: Rc<RefCell<Frame>>,
    input1: Rc<RefCell<Input>>,
    what2: Rc<RefCell<Frame>>,
    input2: Rc<RefCell<Input>>,
    what3: Rc<RefCell<Frame>>,
    input3: Rc<RefCell<Input>>,
    what4: Rc<RefCell<Frame>>,
    input4: Rc<RefCell<IntInput>>,
}

impl Inputable for Input1 {
    #[inline]
    fn get_wind(&self) -> &Rc<RefCell<SingleWindow>> {
        &self.wind
    }

    #[inline]
    fn set_input(&mut self) -> Result<Vec<String>, ()> {
        if !InputExt::value(&*((*self.input).borrow())).is_empty() {
            return Ok(vec![InputExt::value(&*(self.input).borrow())]);
        } else {
            alert(500, 500, "Nothing inputted");
        }

        InputExt::set_value(&*(*self.input).borrow(), "");
        Err(())
    }
}

impl Input1 {
    /// Creates window with asking message and 1 input label

    #[inline]
    pub fn new(title: &str, what_mes: &str) -> Self {
        let win: Rc<RefCell<SingleWindow>> =
            Rc::new(RefCell::new(WidgetBase::new(500, 500, 500, 200, title)));
        let but: Rc<RefCell<Button>> =
            Rc::new(RefCell::new(WidgetBase::new(410, 170, 75, 25, "OK")));

        let wat: Rc<RefCell<Frame>> =
            Rc::new(RefCell::new(WidgetBase::new(-20, 80, 200, 30, what_mes)));
        let inp: Rc<RefCell<Input>> = Rc::new(RefCell::new(WidgetBase::new(150, 80, 300, 30, "")));

        WidgetExt::set_label_size(&mut *(*wat).borrow_mut(), 12);
        InputExt::set_text_size(&mut *(inp).borrow_mut(), 15);
        GroupExt::end(&*(*win).borrow());

        Input1 {
            wind: win,
            ok: but,
            what: wat,
            input: inp,
        }
    }
}

impl Inputable for Input2 {
    #[inline]
    fn get_wind(&self) -> &Rc<RefCell<SingleWindow>> {
        &self.wind
    }

    #[inline]
    fn set_input(&mut self) -> Result<Vec<String>, ()> {
        if !InputExt::value(&*((*self.input1).borrow())).is_empty()
            && !InputExt::value(&*((*self.input2).borrow())).is_empty()
        {
            return Ok(vec![
                InputExt::value(&*(self.input1).borrow()),
                InputExt::value(&*(self.input2).borrow()),
            ]);
        } else {
            alert(500, 500, "Nothing inputted");
        }

        InputExt::set_value(&*(*self.input1).borrow(), "");
        InputExt::set_value(&*(*self.input2).borrow(), "");
        Err(())
    }
}

impl Input2 {
    /// Creates window with asking message and 2 input labels

    #[inline]
    pub fn new(title: &str, what_mes1: &str, what_mes2: &str) -> Self {
        let win: Rc<RefCell<SingleWindow>> =
            Rc::new(RefCell::new(WidgetBase::new(500, 500, 500, 200, title)));
        let but: Rc<RefCell<Button>> =
            Rc::new(RefCell::new(WidgetBase::new(410, 170, 75, 25, "OK")));

        let wat1: Rc<RefCell<Frame>> =
            Rc::new(RefCell::new(WidgetBase::new(-20, 20, 200, 30, what_mes1)));
        let inp1: Rc<RefCell<Input>> = Rc::new(RefCell::new(WidgetBase::new(150, 20, 300, 30, "")));

        let wat2: Rc<RefCell<Frame>> =
            Rc::new(RefCell::new(WidgetBase::new(-20, 100, 200, 30, what_mes2)));
        let inp2: Rc<RefCell<Input>> =
            Rc::new(RefCell::new(WidgetBase::new(150, 100, 300, 30, "")));

        WidgetExt::set_label_size(&mut *(*wat1).borrow_mut(), 12);
        WidgetExt::set_label_size(&mut *(*wat2).borrow_mut(), 12);
        InputExt::set_text_size(&mut *(inp1).borrow_mut(), 15);
        InputExt::set_text_size(&mut *(inp2).borrow_mut(), 15);
        GroupExt::end(&*(*win).borrow());

        Input2 {
            wind: win,
            ok: but,
            what1: wat1,
            input1: inp1,
            what2: wat2,
            input2: inp2,
        }
    }
}

impl Inputable for Input3 {
    #[inline]
    fn get_wind(&self) -> &Rc<RefCell<SingleWindow>> {
        &self.wind
    }

    #[inline]
    fn set_input(&mut self) -> Result<Vec<String>, ()> {
        if !InputExt::value(&*((*self.input1).borrow())).is_empty()
            && !InputExt::value(&*((*self.input1).borrow())).is_empty()
            && !InputExt::value(&*((*self.input3).borrow())).is_empty()
        {
            return Ok(vec![
                InputExt::value(&*(self.input1).borrow()),
                InputExt::value(&*(self.input2).borrow()),
                InputExt::value(&*(self.input3).borrow()),
            ]);
        } else {
            alert(500, 500, "Nothing inputted");
        }

        InputExt::set_value(&*(*self.input1).borrow(), "");
        InputExt::set_value(&*(*self.input2).borrow(), "");
        InputExt::set_value(&*(*self.input3).borrow(), "");
        Err(())
    }
}

impl Input3 {
    /// Creates window with asking message and 3 input labels

    #[inline]
    pub fn new(title: &str, what_mes1: &str, what_mes2: &str, what_mes3: &str) -> Self {
        let win: Rc<RefCell<SingleWindow>> =
            Rc::new(RefCell::new(WidgetBase::new(500, 500, 500, 200, title)));
        let but: Rc<RefCell<Button>> =
            Rc::new(RefCell::new(WidgetBase::new(410, 170, 75, 25, "OK")));

        let wat1: Rc<RefCell<Frame>> =
            Rc::new(RefCell::new(WidgetBase::new(-20, 20, 200, 30, what_mes1)));
        let inp1: Rc<RefCell<Input>> = Rc::new(RefCell::new(WidgetBase::new(150, 20, 300, 30, "")));

        let wat2: Rc<RefCell<Frame>> =
            Rc::new(RefCell::new(WidgetBase::new(-20, 60, 200, 30, what_mes2)));
        let inp2: Rc<RefCell<Input>> = Rc::new(RefCell::new(WidgetBase::new(150, 60, 300, 30, "")));

        let wat3: Rc<RefCell<Frame>> =
            Rc::new(RefCell::new(WidgetBase::new(-20, 100, 200, 30, what_mes3)));
        let inp3: Rc<RefCell<Input>> =
            Rc::new(RefCell::new(WidgetBase::new(150, 100, 300, 30, "")));

        WidgetExt::set_label_size(&mut *(*wat1).borrow_mut(), 12);
        WidgetExt::set_label_size(&mut *(*wat2).borrow_mut(), 12);
        WidgetExt::set_label_size(&mut *(*wat3).borrow_mut(), 12);
        InputExt::set_text_size(&mut *(inp1).borrow_mut(), 15);
        InputExt::set_text_size(&mut *(inp2).borrow_mut(), 15);
        InputExt::set_text_size(&mut *(inp3).borrow_mut(), 15);
        GroupExt::end(&*(*win).borrow());

        Input3 {
            wind: win,
            ok: but,
            what1: wat1,
            input1: inp1,
            what2: wat2,
            input2: inp2,
            what3: wat3,
            input3: inp3,
        }
    }
}

impl Inputable for Input4 {
    #[inline]
    fn get_wind(&self) -> &Rc<RefCell<SingleWindow>> {
        &self.wind
    }

    #[inline]
    fn set_input(&mut self) -> Result<Vec<String>, ()> {
        if !InputExt::value(&*((*self.input1).borrow())).is_empty()
            && !InputExt::value(&*((*self.input2).borrow())).is_empty()
            && !InputExt::value(&*((*self.input3).borrow())).is_empty()
            && !InputExt::value(&*((*self.input4).borrow())).is_empty()
        {
            return Ok(vec![
                InputExt::value(&*(self.input1).borrow()),
                InputExt::value(&*(self.input2).borrow()),
                InputExt::value(&*(self.input3).borrow()),
                InputExt::value(&*(self.input4).borrow()),
            ]);
        } else {
            alert(500, 500, "Nothing inputted");
        }

        InputExt::set_value(&*(*self.input1).borrow(), "");
        InputExt::set_value(&*(*self.input2).borrow(), "");
        InputExt::set_value(&*(*self.input3).borrow(), "");
        InputExt::set_value(&*(*self.input4).borrow(), "");
        Err(())
    }
}

impl Input4 {
    /// Creates window with asking message and 4 input labels

    #[inline]
    pub fn new(
        title: &str,
        what_mes1: &str,
        what_mes2: &str,
        what_mes3: &str,
        what_mes4: &str,
    ) -> Self {
        let win: Rc<RefCell<SingleWindow>> =
            Rc::new(RefCell::new(WidgetBase::new(500, 500, 500, 220, title)));
        let but: Rc<RefCell<Button>> =
            Rc::new(RefCell::new(WidgetBase::new(410, 190, 75, 25, "OK")));

        let wat1: Rc<RefCell<Frame>> =
            Rc::new(RefCell::new(WidgetBase::new(-20, 10, 200, 30, what_mes1)));
        let inp1: Rc<RefCell<Input>> = Rc::new(RefCell::new(WidgetBase::new(150, 10, 300, 30, "")));

        let wat2: Rc<RefCell<Frame>> =
            Rc::new(RefCell::new(WidgetBase::new(-20, 50, 200, 30, what_mes2)));
        let inp2: Rc<RefCell<Input>> = Rc::new(RefCell::new(WidgetBase::new(150, 50, 300, 30, "")));

        let wat3: Rc<RefCell<Frame>> =
            Rc::new(RefCell::new(WidgetBase::new(-20, 90, 200, 30, what_mes3)));
        let inp3: Rc<RefCell<Input>> = Rc::new(RefCell::new(WidgetBase::new(150, 90, 300, 30, "")));

        let wat4: Rc<RefCell<Frame>> =
            Rc::new(RefCell::new(WidgetBase::new(-20, 130, 200, 30, what_mes4)));
        let inp4: Rc<RefCell<IntInput>> =
            Rc::new(RefCell::new(WidgetBase::new(150, 130, 300, 30, "")));

        WidgetExt::set_label_size(&mut *(*wat1).borrow_mut(), 12);
        WidgetExt::set_label_size(&mut *(*wat2).borrow_mut(), 12);
        WidgetExt::set_label_size(&mut *(*wat3).borrow_mut(), 12);
        WidgetExt::set_label_size(&mut *(*wat4).borrow_mut(), 12);
        InputExt::set_text_size(&mut *(inp1).borrow_mut(), 15);
        InputExt::set_text_size(&mut *(inp2).borrow_mut(), 15);
        InputExt::set_text_size(&mut *(inp3).borrow_mut(), 15);
        InputExt::set_text_size(&mut *(inp4).borrow_mut(), 15);
        GroupExt::end(&*(*win).borrow());

        Input4 {
            wind: win,
            ok: but,
            what1: wat1,
            input1: inp1,
            what2: wat2,
            input2: inp2,
            what3: wat3,
            input3: inp3,
            what4: wat4,
            input4: inp4,
        }
    }
}
