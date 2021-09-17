use std::{
    cell::RefCell,
    rc::Rc,
};

use wasm_bindgen::{
    closure::Closure,
    JsCast,
};

use crate::dom;

pub struct RunLoop(Rc<RefCell<Option<Closure<dyn FnMut()>>>>);

impl RunLoop
{
    pub fn new<F: 'static + FnMut()>(mut lambda: F) -> RunLoop
    {
        let run0 = Rc::new(RefCell::new(None));
        let run1 = Rc::clone(&run0);

        *run0.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            lambda();
            request_animation_frame(run1.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));

        RunLoop(run0)
    }

    pub fn start(&self)
    {
        request_animation_frame(self.0.borrow().as_ref().unwrap())
    }
}

fn request_animation_frame(f: &Closure<dyn FnMut()>)
{
    dom::window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap();
}
