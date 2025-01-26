use crate::message::Message;
use iced::keyboard::key::{Key, Named};
use iced::widget::text_editor::{Binding, KeyPress, Status};


// Oh man, this was hard!!!
// Thanks to @kiilerix for the help!
// https://discourse.iced.rs/t/a-spesific-keybinding-applies-to-all-text-editors-while-it-should-not/784


pub fn editor_kp_bindings(kp: KeyPress) -> Option<Binding<Message>> {
    let bnd: Option<Binding<Message>> =
        if kp.key == Key::Named(Named::Enter) && kp.modifiers.shift() && matches!(kp.status, Status::Focused { .. } ) {
            let c1 = Binding::Insert::<Message>('\n');
            let c2 = Binding::Insert::<Message>('/');
            let c3 = Binding::Insert::<Message>('/');
            let c4 = Binding::Insert::<Message>('\n');

            Some(Binding::Sequence(vec![c1, c2, c3, c4]))
        } else if kp.key == Key::Named(Named::Enter) {
            None
        } else if kp.key == Key::Named(Named::Delete) {
            Some(Binding::Delete)
        } else {
            Binding::from_key_press(kp)
        };
    bnd
}

