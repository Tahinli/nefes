use common::user::User;
use gpui::{AppContext, Context, Entity, IntoElement, ParentElement, Styled, Window, div};
use gpui_component::{
    WindowExt,
    button::Button,
    input::{Input, InputEvent, InputState},
    notification::Notification,
};

use crate::gui::{Nefes, Route};

pub struct Login {
    username_input: Entity<InputState>,
    user: Option<User>,
}

impl Login {
    pub fn new(window: &mut Window, cx: &mut Context<Nefes>) -> Self {
        let username_input = cx.new(|cx| {
            let mut input_state = InputState::new(window, cx);
            input_state.set_placeholder("Username...", window, cx);
            input_state.focus(window, cx);

            input_state
        });

        cx.subscribe_in(&username_input, window, |this, input, event, window, cx| {
            if matches!(event, InputEvent::PressEnter { .. }) {
                login_action(this, input, window, cx);
            }
        })
        .detach();

        Self {
            username_input,
            user: None,
        }
    }

    pub fn get_user(&self) -> Option<User> {
        self.user.clone()
    }

    pub fn focus(&self, window: &mut Window, cx: &mut Context<'_, Nefes>) {
        self.username_input.update(cx, |input, cx| {
            input.focus(window, cx);
        });
    }
}

fn login_action(
    this: &mut Nefes,
    input: &Entity<InputState>,
    window: &mut Window,
    cx: &mut Context<'_, Nefes>,
) {
    let username = input.read(cx).text().to_string();
    match User::new(username) {
        Ok(user) => {
            this.login.user = Some(user);
            this.change_page(Route::Chat, window, cx);
        }
        Err(error_value) => {
            window.push_notification(Notification::error(error_value.to_string()), cx)
        }
    }
}

impl Nefes {
    pub fn login_page(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let entity = cx.entity();
        let username_input = self.login.username_input.clone();
        let login_button = Button::new("login_button")
            .on_click(move |_, window, cx| {
                entity.update(cx, |this, cx| {
                    login_action(this, &username_input, window, cx);
                })
            })
            .label("Login");

        div()
            .w_1_5()
            .child("Username")
            .child(Input::new(&self.login.username_input))
            .child(login_button)
            .into_element()
    }
}
