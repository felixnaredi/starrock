use std::{
    cell::RefCell,
    collections::HashSet,
    rc::Rc,
};

use wasm_bindgen::{
    prelude::*,
    JsCast,
};
use web_sys::KeyboardEvent;

use crate::dom;

pub struct KeyboardEventBus
{
    keys_held_down: Rc<RefCell<HashSet<char>>>,
}

impl KeyboardEventBus
{
    pub fn new() -> Result<KeyboardEventBus, JsValue>
    {
        let keys_held_down = Rc::new(RefCell::new(HashSet::new()));

        //
        // Set on keydown closure.
        //
        let closure = Closure::wrap(Box::new({
            let keys_held_down = Rc::downgrade(&keys_held_down);
            move |event: KeyboardEvent| {
                keys_held_down.upgrade().map(|keys_held_down| {
                    // TODO:
                    //   There should be some kind of mechanic controlling whether or not an event
                    //   should prevent default.
                    event.prevent_default();

                    event
                        .key()
                        .chars()
                        .next()
                        .map(|key| keys_held_down.borrow_mut().insert(key));
                });
            }
        }) as Box<dyn FnMut(_)>);

        dom::window()
            .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
        closure.forget();

        //
        // Set on keyup closure.
        //
        let closure = Closure::wrap(Box::new({
            let keys_held_down = Rc::downgrade(&keys_held_down);
            move |event: KeyboardEvent| {
                keys_held_down.upgrade().map(|keys_held_down| {
                    // TODO:
                    //   There should be some kind of mechanic controlling whether or not an event
                    //   should prevent default.
                    event.prevent_default();

                    event
                        .key()
                        .chars()
                        .next()
                        .map(|key| keys_held_down.borrow_mut().remove(&key));
                });
            }
        }) as Box<dyn FnMut(_)>);

        dom::window()
            .add_event_listener_with_callback("keyup", closure.as_ref().unchecked_ref())?;
        closure.forget();

        Ok(KeyboardEventBus { keys_held_down })
    }

    pub fn keys_held_down(&self) -> impl Iterator<Item = char>
    {
        let keys: Vec<char> = self.keys_held_down.borrow().iter().cloned().collect();
        keys.into_iter()
    }

    pub fn key_is_down(&self, key: char) -> bool
    {
        self.keys_held_down.borrow().contains(&key)
    }
}
