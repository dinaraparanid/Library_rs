extern crate fltk;

use crate::{change::Inputable, Lang};
use fltk::{button::Button, dialog::alert, frame::Frame, prelude::*, window::SingleWindow};
use std::{cell::RefCell, rc::Rc};

/// Changes one value

pub struct Input1<I>
where
    I: InputExt + WidgetBase,
{
    wind: Rc<RefCell<SingleWindow>>,
    pub ok: Rc<RefCell<Button>>,
    #[allow(dead_code)]
    what: Rc<RefCell<Frame>>,
    input: Rc<RefCell<I>>,
}

impl<I: InputExt + WidgetBase> Inputable for Input1<I> {
    #[inline]
    fn get_wind(&self) -> &Rc<RefCell<SingleWindow>> {
        &self.wind
    }

    #[inline]
    fn set_input(&mut self, lang: Lang) -> Result<Vec<String>, ()> {
        if !InputExt::value(&*((*self.input).borrow())).is_empty() {
            return Ok(vec![InputExt::value(&*(self.input).borrow())]);
        } else {
            self.hide();
            alert(
                500,
                500,
                match lang {
                    Lang::English => "Nothing inputted",
                    Lang::Russian => "Ничего не введено",
                },
            );
        }

        InputExt::set_value(&mut *(*self.input).borrow_mut(), "");
        Err(())
    }
}

impl<I: InputExt + WidgetBase> Input1<I> {
    /// Creates window with asking message and 1 input label

    #[inline]
    pub fn new(title: &str, what_mes: &str) -> Self {
        let win: Rc<RefCell<SingleWindow>> =
            Rc::new(RefCell::new(WidgetBase::new(500, 500, 500, 200, None)));
        win.borrow_mut().set_label(title);

        let but: Rc<RefCell<Button>> =
            Rc::new(RefCell::new(WidgetBase::new(410, 170, 75, 25, Some("OK"))));

        let wat: Rc<RefCell<Frame>> =
            Rc::new(RefCell::new(WidgetBase::new(-20, 80, 200, 30, None)));
        wat.borrow_mut().set_label(what_mes);

        let inp: Rc<RefCell<I>> = Rc::new(RefCell::new(WidgetBase::new(150, 80, 300, 30, None)));

        WidgetExt::set_label_size(&mut *(*wat).borrow_mut(), 12);
        InputExt::set_text_size(&mut *(inp).borrow_mut(), 15);
        GroupExt::end(&*(*win).borrow());

        Self {
            wind: win,
            ok: but,
            what: wat,
            input: inp,
        }
    }
}
