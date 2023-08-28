extern crate fltk;

use crate::{change::Inputable, Lang};
use std::{cell::RefCell, rc::Rc};

use fltk::{button::Button, dialog::alert, frame::Frame, prelude::*, window::SingleWindow};

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
    #[allow(dead_code)]
    what1: Rc<RefCell<Frame>>,
    input1: Rc<RefCell<I>>,
    #[allow(dead_code)]
    what2: Rc<RefCell<Frame>>,
    input2: Rc<RefCell<J>>,
    #[allow(dead_code)]
    what3: Rc<RefCell<Frame>>,
    input3: Rc<RefCell<L>>,
    #[allow(dead_code)]
    what4: Rc<RefCell<Frame>>,
    input4: Rc<RefCell<K>>,
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
    fn set_input(&mut self, lang: Lang) -> Result<Vec<String>, ()> {
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

        InputExt::set_value(&mut *(*self.input1).borrow_mut(), "");
        InputExt::set_value(&mut *(*self.input2).borrow_mut(), "");
        InputExt::set_value(&mut *(*self.input3).borrow_mut(), "");
        InputExt::set_value(&mut *(*self.input4).borrow_mut(), "");
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
            Rc::new(RefCell::new(WidgetBase::new(500, 500, 500, 220, None)));
        win.borrow_mut().set_label(title);

        let but: Rc<RefCell<Button>> =
            Rc::new(RefCell::new(WidgetBase::new(410, 190, 75, 25, Some("OK"))));

        let wat1: Rc<RefCell<Frame>> =
            Rc::new(RefCell::new(WidgetBase::new(-20, 10, 200, 30, None)));
        wat1.borrow_mut().set_label(what_mes1);

        let inp1: Rc<RefCell<I>> = Rc::new(RefCell::new(WidgetBase::new(150, 10, 300, 30, None)));

        let wat2: Rc<RefCell<Frame>> =
            Rc::new(RefCell::new(WidgetBase::new(-20, 50, 200, 30, None)));
        wat2.borrow_mut().set_label(what_mes2);

        let inp2: Rc<RefCell<J>> = Rc::new(RefCell::new(WidgetBase::new(150, 50, 300, 30, None)));

        let wat3: Rc<RefCell<Frame>> =
            Rc::new(RefCell::new(WidgetBase::new(-20, 90, 200, 30, None)));
        wat3.borrow_mut().set_label(what_mes3);

        let inp3: Rc<RefCell<L>> = Rc::new(RefCell::new(WidgetBase::new(150, 90, 300, 30, None)));

        let wat4: Rc<RefCell<Frame>> =
            Rc::new(RefCell::new(WidgetBase::new(-20, 130, 200, 30, None)));
        wat4.borrow_mut().set_label(what_mes4);

        let inp4: Rc<RefCell<K>> = Rc::new(RefCell::new(WidgetBase::new(150, 130, 300, 30, None)));

        WidgetExt::set_label_size(&mut *(*wat1).borrow_mut(), 12);
        WidgetExt::set_label_size(&mut *(*wat2).borrow_mut(), 12);
        WidgetExt::set_label_size(&mut *(*wat3).borrow_mut(), 12);
        WidgetExt::set_label_size(&mut *(*wat4).borrow_mut(), 12);
        InputExt::set_text_size(&mut *(inp1).borrow_mut(), 15);
        InputExt::set_text_size(&mut *(inp2).borrow_mut(), 15);
        InputExt::set_text_size(&mut *(inp3).borrow_mut(), 15);
        InputExt::set_text_size(&mut *(inp4).borrow_mut(), 15);
        GroupExt::end(&*(*win).borrow());

        Self {
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
