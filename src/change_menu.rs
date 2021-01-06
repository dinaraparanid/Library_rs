use fltk::button::Button;
use fltk::dialog::*;
use fltk::frame::Frame;
use fltk::input::{Input, InputExt, IntInput, SecretInput};
use fltk::window::SingleWindow;
use fltk::WidgetExt;
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

pub struct Input1<I>
where
    I: InputExt + WidgetBase,
{
    wind: Rc<RefCell<SingleWindow>>,
    pub ok: Rc<RefCell<Button>>,
    what: Rc<RefCell<Frame>>,
    input: Rc<RefCell<I>>,
}

/// Changes two values

pub struct Input2<I, J>
where
    I: InputExt + WidgetBase,
    J: InputExt + WidgetBase,
{
    wind: Rc<RefCell<SingleWindow>>,
    pub ok: Rc<RefCell<Button>>,
    what1: Rc<RefCell<Frame>>,
    input1: Rc<RefCell<I>>,
    what2: Rc<RefCell<Frame>>,
    input2: Rc<RefCell<J>>,
}

/// Changes three values

pub struct Input3<I, J, L>
where
    I: InputExt + WidgetBase,
    J: InputExt + WidgetBase,
    L: InputExt + WidgetBase,
{
    wind: Rc<RefCell<SingleWindow>>,
    pub ok: Rc<RefCell<Button>>,
    what1: Rc<RefCell<Frame>>,
    input1: Rc<RefCell<I>>,
    what2: Rc<RefCell<Frame>>,
    input2: Rc<RefCell<J>>,
    what3: Rc<RefCell<Frame>>,
    input3: Rc<RefCell<L>>,
}

/// Changes four values

pub struct Input4<I, J, L, K>
where
    I: InputExt + WidgetBase,
    J: InputExt + WidgetBase,
    L: InputExt + WidgetBase,
    K: InputExt + WidgetBase,
{
    wind: Rc<RefCell<SingleWindow>>,
    pub ok: Rc<RefCell<Button>>,
    what1: Rc<RefCell<Frame>>,
    input1: Rc<RefCell<I>>,
    what2: Rc<RefCell<Frame>>,
    input2: Rc<RefCell<J>>,
    what3: Rc<RefCell<Frame>>,
    input3: Rc<RefCell<L>>,
    what4: Rc<RefCell<Frame>>,
    input4: Rc<RefCell<K>>,
}

impl<I: InputExt + WidgetBase> Inputable for Input1<I> {
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

impl<I: InputExt + WidgetBase> Input1<I> {
    /// Creates window with asking message and 1 input label

    #[inline]
    pub fn new(title: &str, what_mes: &str) -> Self {
        let win: Rc<RefCell<SingleWindow>> =
            Rc::new(RefCell::new(WidgetBase::new(500, 500, 500, 200, title)));
        let but: Rc<RefCell<Button>> =
            Rc::new(RefCell::new(WidgetBase::new(410, 170, 75, 25, "OK")));

        let wat: Rc<RefCell<Frame>> =
            Rc::new(RefCell::new(WidgetBase::new(-20, 80, 200, 30, what_mes)));
        let inp: Rc<RefCell<I>> = Rc::new(RefCell::new(WidgetBase::new(150, 80, 300, 30, "")));

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

impl<I, J> Inputable for Input2<I, J>
where
    I: InputExt + WidgetBase,
    J: InputExt + WidgetBase,
{
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

impl<I, J> Input2<I, J>
where
    I: InputExt + WidgetBase,
    J: InputExt + WidgetBase,
{
    /// Creates window with asking message and 2 input labels

    #[inline]
    pub fn new(title: &str, what_mes1: &str, what_mes2: &str) -> Self
/*where
        I: InputExt + WidgetBase,
        J: InputExt + WidgetBase,            */ {
        let win: Rc<RefCell<SingleWindow>> =
            Rc::new(RefCell::new(WidgetBase::new(500, 500, 500, 200, title)));
        let but: Rc<RefCell<Button>> =
            Rc::new(RefCell::new(WidgetBase::new(410, 170, 75, 25, "OK")));

        let wat1: Rc<RefCell<Frame>> =
            Rc::new(RefCell::new(WidgetBase::new(-20, 20, 200, 30, what_mes1)));
        let inp1: Rc<RefCell<I>> = Rc::new(RefCell::new(WidgetBase::new(150, 20, 300, 30, "")));

        let wat2: Rc<RefCell<Frame>> =
            Rc::new(RefCell::new(WidgetBase::new(-20, 100, 200, 30, what_mes2)));
        let inp2: Rc<RefCell<J>> = Rc::new(RefCell::new(WidgetBase::new(150, 100, 300, 30, "")));

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

impl<I, J, L> Inputable for Input3<I, J, L>
where
    I: InputExt + WidgetBase,
    J: InputExt + WidgetBase,
    L: InputExt + WidgetBase,
{
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

impl<I, J, L> Input3<I, J, L>
where
    I: InputExt + WidgetBase,
    J: InputExt + WidgetBase,
    L: InputExt + WidgetBase,
{
    /// Creates window with asking message and 3 input labels

    #[inline]
    pub fn new(title: &str, what_mes1: &str, what_mes2: &str, what_mes3: &str) -> Self
/*where
        I: InputExt + WidgetBase,
        J: InputExt + WidgetBase,
        L: InputExt + WidgetBase,      */ {
        let win: Rc<RefCell<SingleWindow>> =
            Rc::new(RefCell::new(WidgetBase::new(500, 500, 500, 200, title)));
        let but: Rc<RefCell<Button>> =
            Rc::new(RefCell::new(WidgetBase::new(410, 170, 75, 25, "OK")));

        let wat1: Rc<RefCell<Frame>> =
            Rc::new(RefCell::new(WidgetBase::new(-20, 20, 200, 30, what_mes1)));
        let inp1: Rc<RefCell<I>> = Rc::new(RefCell::new(WidgetBase::new(150, 20, 300, 30, "")));

        let wat2: Rc<RefCell<Frame>> =
            Rc::new(RefCell::new(WidgetBase::new(-20, 60, 200, 30, what_mes2)));
        let inp2: Rc<RefCell<J>> = Rc::new(RefCell::new(WidgetBase::new(150, 60, 300, 30, "")));

        let wat3: Rc<RefCell<Frame>> =
            Rc::new(RefCell::new(WidgetBase::new(-20, 100, 200, 30, what_mes3)));
        let inp3: Rc<RefCell<L>> = Rc::new(RefCell::new(WidgetBase::new(150, 100, 300, 30, "")));

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

impl<I, J, L, K> Inputable for Input4<I, J, L, K>
where
    I: InputExt + WidgetBase,
    J: InputExt + WidgetBase,
    L: InputExt + WidgetBase,
    K: InputExt + WidgetBase,
{
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

impl<I, J, L, K> Input4<I, J, L, K>
where
    I: InputExt + WidgetBase,
    J: InputExt + WidgetBase,
    L: InputExt + WidgetBase,
    K: InputExt + WidgetBase,
{
    /// Creates window with asking message and 4 input labels

    #[inline]
    pub fn new(
        title: &str,
        what_mes1: &str,
        what_mes2: &str,
        what_mes3: &str,
        what_mes4: &str,
    ) -> Self
    where
        I: InputExt + WidgetBase,
        J: InputExt + WidgetBase,
        L: InputExt + WidgetBase,
        K: InputExt + WidgetBase,
    {
        let win: Rc<RefCell<SingleWindow>> =
            Rc::new(RefCell::new(WidgetBase::new(500, 500, 500, 220, title)));
        let but: Rc<RefCell<Button>> =
            Rc::new(RefCell::new(WidgetBase::new(410, 190, 75, 25, "OK")));

        let wat1: Rc<RefCell<Frame>> =
            Rc::new(RefCell::new(WidgetBase::new(-20, 10, 200, 30, what_mes1)));
        let inp1: Rc<RefCell<I>> = Rc::new(RefCell::new(WidgetBase::new(150, 10, 300, 30, "")));

        let wat2: Rc<RefCell<Frame>> =
            Rc::new(RefCell::new(WidgetBase::new(-20, 50, 200, 30, what_mes2)));
        let inp2: Rc<RefCell<J>> = Rc::new(RefCell::new(WidgetBase::new(150, 50, 300, 30, "")));

        let wat3: Rc<RefCell<Frame>> =
            Rc::new(RefCell::new(WidgetBase::new(-20, 90, 200, 30, what_mes3)));
        let inp3: Rc<RefCell<L>> = Rc::new(RefCell::new(WidgetBase::new(150, 90, 300, 30, "")));

        let wat4: Rc<RefCell<Frame>> =
            Rc::new(RefCell::new(WidgetBase::new(-20, 130, 200, 30, what_mes4)));
        let inp4: Rc<RefCell<K>> = Rc::new(RefCell::new(WidgetBase::new(150, 130, 300, 30, "")));

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
