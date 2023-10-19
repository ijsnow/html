use std::sync::{Arc, Mutex};

use gloo::events::EventListener;
use html::forms::{Form, Input};
use web_sys::{Event, HtmlElement};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub struct Login {
    email: String,
    password: String,
}

impl From<Login> for Form {
    fn from(Login { email, password }: Login) -> Form {
        Form::builder()
            .push(Input::builder().type_("text").value(email).build())
            .push(Input::builder().type_("password").value(password).build())
            .build()
    }
}

impl Login {
    pub fn register_handlers(this: Arc<Mutex<Self>>, target: HtmlElement) -> Vec<EventListener> {
        let mut listeners = vec![];

        let listener_this = this.clone();

        listeners.push(EventListener::new(&target, "input", move |event| {
            let mut this = listener_this.lock().unwrap();

            Login::update_email(&mut this, &event);
        }));

        let listener_this = this.clone();

        listeners.push(EventListener::new(&target, "input", move |event| {
            let mut this = listener_this.lock().unwrap();

            Login::update_password(&mut this, &event);
        }));

        listeners
    }

    fn update_email(&mut self, event: &Event) {
        todo!()
    }

    fn update_password(&mut self, event: &Event) {
        todo!()
    }
}
