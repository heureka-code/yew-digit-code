use std::sync::Arc;

use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::events::InputEvent;
use yew::prelude::*;
use yew::{html, Html};

use crate::focus_offset::FocusResult;
use crate::{ArcStrOrChar, DigitCode};

use super::DigitCodeProfile;

fn handle_input<PROFILE: DigitCodeProfile + 'static>(
    index: usize,
    profile: Arc<PROFILE>,
    focus_next: Callback<usize, FocusResult>,
    set_value: Callback<Option<ArcStrOrChar>>,
) -> Callback<InputEvent> {
    Callback::from(move |e: InputEvent| {
        let profile = profile.clone();
        let target: Option<EventTarget> = e.target();
        let focus_next = focus_next.clone();
        let set_value = set_value.clone();
        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
        if let Some(input) = input {
            let chr: String = input.value();

            #[cfg(feature = "unicode-segmentation")]
            let chr = profile.is_valid_char(&chr).then_some(chr.into());
            #[cfg(not(feature = "unicode-segmentation"))]
            let chr = {
                if chr.len() == 1 {
                    if let Some(chr) = chr.chars().last() {
                        profile.is_valid_char(chr).then_some(chr)
                    } else {
                        None
                    }
                } else {
                    None
                }
            };
            let valid = chr.is_some();
            set_value.emit(chr);

            if valid {
                focus_next.emit(index);
            }
        }
    })
}

fn handle_keydown<PROFILE: DigitCodeProfile + 'static>(
    index: usize,
    profile: Arc<PROFILE>,
    focus_next: Callback<usize, FocusResult>,
    focus_prev: Callback<usize, FocusResult>,
    enter_hit: Callback<usize>,
    set_value: Callback<Option<ArcStrOrChar>>,
) -> Callback<KeyboardEvent> {
    Callback::from(move |e: KeyboardEvent| {
        let key = e.key();
        #[cfg(feature = "log")]
        log::trace!("Keydown: {key}");

        if key == "ArrowLeft" {
            focus_prev.emit(index);
        } else if key == "ArrowRight" {
            focus_next.emit(index);
        } else if key == "Enter" {
            enter_hit.emit(index);
        } else if key == "Backspace" {
            e.prevent_default();
            set_value.emit(None);
            focus_prev.emit(index);
        } else {
            #[cfg(not(feature = "unicode-segmentation"))]
            if key.len() == 1 {
                if let Some(c) = key.chars().last() {
                    if profile.is_valid_char(c) {
                        set_value.emit(None);
                    }
                }
            }
            #[cfg(feature = "unicode-segmentation")]
            if profile.is_valid_char(&key) {
                set_value.emit(None);
            }
        }
    })
}

#[derive(Properties, PartialEq)]
pub(super) struct Props<PROFILE: DigitCodeProfile + 'static> {
    pub index: usize,
    pub whole_code: UseStateHandle<DigitCode<PROFILE>>,
    pub focus_next: Callback<usize, FocusResult>,
    pub focus_prev: Callback<usize, FocusResult>,
    pub enter_hit: Callback<usize>,
    pub disabled: bool,
    pub profile: Arc<PROFILE>,
    pub set_value: Callback<(usize, Option<ArcStrOrChar>)>,
}

#[function_component(CodeSingleDigitElement)]
pub(super) fn code_single_digit_element<T: DigitCodeProfile + 'static>(
    Props {
        index,
        focus_next,
        whole_code,
        focus_prev,
        enter_hit,
        disabled,
        profile,
        set_value,
    }: &Props<T>,
) -> Html {
    let index = *index;
    let whole_code = whole_code.clone();
    let get_value = || (*whole_code).get(index).clone();

    let cloned_set_value = set_value.clone();
    let set_value =
        Callback::from(move |val: Option<ArcStrOrChar>| cloned_set_value.emit((index, val)));

    let handle_input = handle_input(
        index,
        profile.clone(),
        focus_next.clone(),
        set_value.clone(),
    );

    let handle_keydown = handle_keydown(
        index,
        profile.clone(),
        focus_next.clone(),
        focus_prev.clone(),
        enter_hit.clone(),
        set_value,
    );
    let input_mode = profile.input_mode(index).to_owned();

    let value = get_value().map(|s| s.to_string()).unwrap_or_default();
    html!(
        <input type={"text"} maxlength={1} inputmode={input_mode} disabled={*disabled} value={value} oninput={handle_input} onkeydown={handle_keydown} data-index={index.to_string()}/>
    )
}
