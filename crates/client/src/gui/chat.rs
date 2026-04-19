use common::{bastion::Bastion, message::Message, user::User};
use gpui::{
    AppContext, Context, Entity, FontWeight, InteractiveElement, IntoElement, ParentElement,
    StatefulInteractiveElement, Styled, Window, div, rgb,
};
use gpui_component::{
    WindowExt,
    input::{Input, InputEvent, InputState},
    notification::Notification,
};

use crate::gui::Nefes;

pub struct Chat {
    bastion: Bastion,
    users: Vec<User>,
    chat_messages: Vec<Message>,
    message_input: Entity<InputState>,
}

impl Chat {
    pub fn new(window: &mut Window, cx: &mut Context<Nefes>) -> Self {
        let message_input = cx.new(|cx| {
            let mut input_state = InputState::new(window, cx);
            input_state.set_placeholder("", window, cx);

            input_state
        });

        let bastion = Bastion::new("Bastion").unwrap();

        let bastion = || bastion.clone();
        cx.subscribe_in(&message_input, window, |this, input, event, window, cx| {
            if matches!(event, InputEvent::PressEnter { .. }) {
                match Message::new(
                    this.login.get_user().unwrap(),
                    this.chat.bastion.clone(),
                    input.read(cx).text(),
                ) {
                    Ok(message) => {
                        this.chat.chat_messages.push(message);
                        input.update(cx, |input_state, cx| {
                            input_state.set_value("", window, cx);
                        });
                    }
                    Err(error_value) => {
                        window.push_notification(Notification::error(error_value.to_string()), cx)
                    }
                }
            }
        })
        .detach();

        let person_1 = User::new("Ahmet").unwrap();
        let person_2 = User::new("Kaan").unwrap();
        let message_1 = Message::new(person_1, bastion(), "Hello, how are you?").unwrap();
        let message_2 = Message::new(person_2, bastion(), "I'm doing well. How are you?").unwrap();
        let person_1 = User::new("Ahmet").unwrap();
        let message_3 =
            Message::new(person_1, bastion(), "I'm good too. Thanks for asking!").unwrap();
        let person_2 = User::new("Kaan").unwrap();
        let message_4 = Message::new(person_2, bastion(), "A".repeat(1024)).unwrap();
        let person_1 = User::new("Ahmet").unwrap();
        let person_2 = User::new("Kaan").unwrap();
        Self {
            bastion: bastion(),
            users: vec![person_1, person_2],
            chat_messages: vec![message_1, message_2, message_3, message_4],
            message_input,
        }
    }

    pub fn focus(&self, window: &mut Window, cx: &mut Context<'_, Nefes>) {
        self.message_input.update(cx, |input, cx| {
            input.focus(window, cx);
        });
    }
}

impl Nefes {
    pub fn chat_page(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .size_full()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .min_w_0()
                    .child(
                        div()
                            .id("chat_messages")
                            .flex()
                            .flex_col()
                            .flex_1()
                            .min_w_0()
                            .overflow_y_scroll()
                            .p_2()
                            .gap_1()
                            .children(self.chat.chat_messages.iter().map(render_message)),
                    )
                    .child(
                        div()
                            .p_2()
                            .border_t_1()
                            .border_color(rgb(0x2f3136))
                            .child(Input::new(&self.chat.message_input)),
                    ),
            )
            .child(
                div()
                    .id("user_list")
                    .h_full()
                    .flex()
                    .flex_col()
                    .flex_shrink_0()
                    .w_1_5()
                    .border_l_1()
                    .border_color(rgb(0x2f3136))
                    .bg(rgb(0x2b2d31))
                    .p_2()
                    .gap_1()
                    .child(
                        div()
                            .text_color(rgb(0x949ba4))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child(format!("Online - {}", self.chat.users.len())),
                    )
                    .children(
                        self.chat
                            .users
                            .iter()
                            .map(|user| div().p_1().child(user.get_username().clone())),
                    ),
            )
    }
}

fn render_message(message: &Message) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .p_1()
        .child(
            div()
                .text_color(rgb(0x7289da))
                .font_weight(FontWeight::BOLD)
                .child(message.get_user().get_username().clone()),
        )
        .child(div().pl_2().child(message.get_message().clone()))
}
