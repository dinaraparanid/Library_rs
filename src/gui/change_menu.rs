use crate::gui::message_dialog::MessageDialog;
use fltk::button::Button;
use fltk::frame::Frame;
use fltk::input::Input;
use fltk::window::SingleWindow;
use fltk::GroupExt;
use fltk::InputExt;
use fltk::WidgetBase;
use fltk::WidgetExt;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Inputable {
    fn set_input(&mut self);
    fn show(&self);
}

pub struct Input1<'a> {
    wind: Rc<RefCell<SingleWindow>>,
    pub(crate) ok: Rc<RefCell<Button>>,
    what: Rc<RefCell<Frame>>,
    input: Rc<RefCell<Input>>,
    change: &'a mut String,
}

pub struct Input2<'a> {
    wind: Rc<RefCell<SingleWindow>>,
    pub(crate) ok: Rc<RefCell<Button>>,
    what1: Rc<RefCell<Frame>>,
    input1: Rc<RefCell<Input>>,
    change1: &'a mut String,
    what2: Rc<RefCell<Frame>>,
    input2: Rc<RefCell<Input>>,
    change2: &'a mut String,
}

pub struct Input3<'a> {
    wind: Rc<RefCell<SingleWindow>>,
    pub(crate) ok: Rc<RefCell<Button>>,
    what1: Rc<RefCell<Frame>>,
    input1: Rc<RefCell<Input>>,
    change1: &'a mut u8,
    what2: Rc<RefCell<Frame>>,
    input2: Rc<RefCell<Input>>,
    change2: &'a mut u8,
    what3: Rc<RefCell<Frame>>,
    input3: Rc<RefCell<Input>>,
    change3: &'a mut u16,
}

impl<'a> Inputable for Input1<'a> {
    #[inline]
    fn set_input(&mut self) {
        if !InputExt::value(&*((*self.input).borrow())).is_empty() {
            *self.change = WidgetExt::label(&*(self.input).borrow());
        } else {
            let fail = MessageDialog::new("Nothing inputted", 0);
            fail.show();
        }
        InputExt::set_value(&*(*self.input).borrow(), "");
    }

    #[inline]
    fn show(&self) {
        (*self.wind).borrow_mut().show_with_env_args()
    }
}

impl<'a> Input1<'a> {
    #[inline]
    pub fn new(title: &str, what_mes: &str, chng: &'a mut String) -> Self {
        let win: Rc<RefCell<SingleWindow>> =
            Rc::new(RefCell::new(WidgetBase::new(500, 500, 500, 200, title)));
        let but: Rc<RefCell<Button>> =
            Rc::new(RefCell::new(WidgetBase::new(410, 170, 75, 25, "OK")));

        let wat: Rc<RefCell<Frame>> =
            Rc::new(RefCell::new(WidgetBase::new(10, 80, 200, 30, what_mes)));
        let inp: Rc<RefCell<Input>> = Rc::new(RefCell::new(WidgetBase::new(150, 80, 300, 30, "")));

        WidgetExt::set_label_size(&mut *(*wat).borrow_mut(), 12);
        InputExt::set_text_size(&mut *(inp).borrow_mut(), 15);
        GroupExt::end(&*(*win).borrow());

        Input1 {
            wind: win,
            ok: but,
            what: wat,
            input: inp,
            change: chng,
        }
    }
}

impl<'a> Inputable for Input2<'a> {
    #[inline]
    fn set_input(&mut self) {
        if !InputExt::value(&*((*self.input1).borrow())).is_empty()
            && !InputExt::value(&*((*self.input2).borrow())).is_empty()
        {
            *self.change1 = WidgetExt::label(&*(self.input1).borrow());
            *self.change2 = WidgetExt::label(&*(self.input2).borrow());
        } else {
            let fail = MessageDialog::new("Nothing inputted", 0);
            fail.show();
        }

        InputExt::set_value(&*(*self.input1).borrow(), "");
        InputExt::set_value(&*(*self.input2).borrow(), "");
    }

    #[inline]
    fn show(&self) {
        (*self.wind).borrow_mut().show_with_env_args()
    }
}

impl<'a> Input2<'a> {
    #[inline]
    pub fn new(
        title: &str,
        what_mes1: &str,
        what_mes2: &str,
        chng1: &'a mut String,
        chng2: &'a mut String,
    ) -> Self {
        let win: Rc<RefCell<SingleWindow>> =
            Rc::new(RefCell::new(WidgetBase::new(500, 500, 500, 200, title)));
        let but: Rc<RefCell<Button>> =
            Rc::new(RefCell::new(WidgetBase::new(410, 170, 75, 25, "OK")));

        let wat1: Rc<RefCell<Frame>> =
            Rc::new(RefCell::new(WidgetBase::new(10, 20, 200, 30, what_mes1)));
        let inp1: Rc<RefCell<Input>> = Rc::new(RefCell::new(WidgetBase::new(150, 20, 300, 30, "")));

        let wat2: Rc<RefCell<Frame>> =
            Rc::new(RefCell::new(WidgetBase::new(10, 100, 200, 30, what_mes2)));
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
            change1: chng1,
            change2: chng2,
        }
    }
}

impl<'a> Inputable for Input3<'a> {
    #[inline]
    fn set_input(&mut self) {
        if !InputExt::value(&*((*self.input1).borrow())).is_empty()
            && !InputExt::value(&*((*self.input2).borrow())).is_empty()
            && !InputExt::value(&*((*self.input3).borrow())).is_empty()
        {
            *self.change1 = WidgetExt::label(&*(self.input1).borrow())
                .trim()
                .parse()
                .unwrap();
            *self.change2 = WidgetExt::label(&*(self.input2).borrow())
                .trim()
                .parse()
                .unwrap();
            *self.change3 = WidgetExt::label(&*(self.input3).borrow())
                .trim()
                .parse()
                .unwrap();
        } else {
            let fail = MessageDialog::new("Nothing inputted", 0);
            fail.show();
        }

        InputExt::set_value(&*(*self.input1).borrow(), "");
        InputExt::set_value(&*(*self.input2).borrow(), "");
        InputExt::set_value(&*(*self.input3).borrow(), "");
    }

    #[inline]
    fn show(&self) {
        (*self.wind).borrow_mut().show_with_env_args()
    }
}

impl<'a> Input3<'a> {
    #[inline]
    pub fn new(
        title: &str,
        what_mes1: &str,
        what_mes2: &str,
        what_mes3: &str,
        chng1: &'a mut u8,
        chng2: &'a mut u8,
        chng3: &'a mut u16,
    ) -> Self {
        let win: Rc<RefCell<SingleWindow>> =
            Rc::new(RefCell::new(WidgetBase::new(500, 500, 500, 200, title)));
        let but: Rc<RefCell<Button>> =
            Rc::new(RefCell::new(WidgetBase::new(410, 170, 75, 25, "OK")));

        let wat1: Rc<RefCell<Frame>> =
            Rc::new(RefCell::new(WidgetBase::new(10, 20, 200, 30, what_mes1)));
        let inp1: Rc<RefCell<Input>> = Rc::new(RefCell::new(WidgetBase::new(150, 20, 300, 30, "")));

        let wat2: Rc<RefCell<Frame>> =
            Rc::new(RefCell::new(WidgetBase::new(10, 60, 200, 30, what_mes2)));
        let inp2: Rc<RefCell<Input>> = Rc::new(RefCell::new(WidgetBase::new(150, 60, 300, 30, "")));

        let wat3: Rc<RefCell<Frame>> =
            Rc::new(RefCell::new(WidgetBase::new(10, 100, 200, 30, what_mes3)));
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
            change1: chng1,
            change2: chng2,
            change3: chng3,
        }
    }
}
