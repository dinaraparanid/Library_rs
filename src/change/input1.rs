extern crate fltk;
use crate::change::Inputable;
use fltk::button::Button;
use fltk::dialog::alert;
use fltk::frame::Frame;
use fltk::prelude::*;
use fltk::window::SingleWindow;
use std::cell::RefCell;
use std::rc::Rc;

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
            self.hide();
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
