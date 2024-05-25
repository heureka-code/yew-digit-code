use yew::prelude::*;
use yew::{html, Html};

use crate::control_flags::ControlFlags;
use crate::{ArcStrOrChar, DigitCode};

use super::single_digit_element::CodeSingleDigitElement;
use super::DigitCodeProfile;

#[derive(Properties, PartialEq)]
pub struct Props<PROFILE: DigitCodeProfile + 'static> {
    pub id: AttrValue,
    #[prop_or_default]
    pub submit_code: Option<Callback<String>>,
    pub code: UseStateHandle<super::DigitCode<PROFILE>>,
    pub flags: UseStateHandle<ControlFlags>,
    #[prop_or_default]
    pub class: Classes,
    #[cfg(feature = "yew-hooks")]
    #[prop_or_default]
    pub oninit: Callback<AttrValue>,
}

fn enter_hit<PROFILE: DigitCodeProfile + 'static>(
    submit_code: Callback<String>,
    code_state: UseStateHandle<DigitCode<PROFILE>>,
    disabled_input: UseStateHandle<bool>,
) -> Callback<usize> {
    Callback::from(move |_: usize| {
        let code_state = code_state.clone();
        #[cfg(feature = "log")]
        log::trace!("Enter hit: {:?}", *code_state);
        if let Some(code) = (*code_state).joined() {
            disabled_input.set(true);
            submit_code.emit(code);
            disabled_input.set(false);
        }
    })
}

fn set_value<PROFILE: DigitCodeProfile + 'static>(
    submit_code: Callback<String>,
    code_state: UseStateHandle<DigitCode<PROFILE>>,
    disabled_input: UseStateHandle<bool>,
) -> Callback<(usize, Option<ArcStrOrChar>)> {
    Callback::from(move |(index, value)| {
        #[cfg(feature = "log")]
        log::trace!("{index} called set_value with {value:?}");
        let code_state = code_state.clone();
        let mut v: DigitCode<PROFILE> = (*code_state).clone();
        v.change_update_indicator();
        let value: Option<ArcStrOrChar> = value;
        v.set(index, value);
        code_state.set(v.clone());

        if index == v.len() - 1 {
            if let Some(code) = v.joined() {
                disabled_input.set(true);
                submit_code.emit(code);
                disabled_input.set(false);
            }
        }
        #[cfg(feature = "log")]
        log::trace!("{index} call to set_value produced {:?}", v);
    })
}

#[function_component(InnerCodeDigitInput)]
pub fn inner_code_digit_element<PROFILE: DigitCodeProfile + 'static>(
    Props {
        id,
        submit_code,
        code,
        flags,
        class,
        #[cfg(feature = "yew-hooks")]
        oninit,
    }: &Props<PROFILE>,
) -> Html {
    #[cfg(feature = "yew-hooks")]
    let is_initialized = {
        let is_initialized_state = use_state_eq(|| false);
        use yew_hooks::prelude::*;
        let cloned_is_initialized = is_initialized_state.clone();

        let id = id.clone();
        #[cfg(feature = "log")]
        log::trace!("Try creating interval: id=\"{id}\"");

        let oninit = oninit.clone();
        use_interval(
            move || {
                if crate::focus_offset::document().is_some() {
                    cloned_is_initialized.set(true);
                    #[cfg(feature = "log")]
                    log::debug!("Init complete: id=\"{id}\"");
                    oninit.emit(id.clone());
                }
            },
            if *is_initialized_state { 0 } else { 100 },
        );
        *is_initialized_state
    };
    #[cfg(not(feature = "yew-hooks"))]
    let is_initialized = true;

    let id = id.to_string();

    use super::focus_offset::{focus_offset, FocusOffset};
    let profile = code.profile();
    let whole_code_state = code.clone();

    let disabled_input = use_state(|| false);

    let id = id.to_string();
    let digit_count = profile.len();

    let submit_code: Callback<String> = submit_code.clone().unwrap_or_else(|| {
        #[cfg(feature = "log")]
        return Callback::from(move |code| {
            log::info!("Default submit method of digit code: {code}");
        });
        #[allow(unused)]
        Callback::default()
    });

    let offset_closure_next = focus_offset(id.to_string(), digit_count, FocusOffset::Next);
    let offset_closure_prev = focus_offset(id.to_string(), digit_count, FocusOffset::Previous);

    let focus_next = Callback::from(move |i: usize| offset_closure_next(i));
    let focus_prev = Callback::from(move |i: usize| offset_closure_prev(i));

    #[cfg(feature = "log")]
    log::trace!("Try working on flags: {is_initialized}");
    if is_initialized {
        let current_flags = (*flags).clone();
        let mut builder = current_flags.change();

        if let Some(focus_num) = builder.focus {
            builder = builder.unset_focus();
            if focus_num == 0 {
                focus_prev.emit(1);
            } else {
                focus_next.emit(focus_num - 1);
            }
        }
        if builder.clear.is_some() {
            builder = builder.unset_clear();
            whole_code_state.set(whole_code_state.as_empty());
        }
        let new_flags = builder.apply();
        if new_flags != *current_flags {
            current_flags.set(new_flags);
        }
    }

    let enter_hit = enter_hit(
        submit_code.clone(),
        whole_code_state.clone(),
        disabled_input.clone(),
    );
    let set_value = set_value(
        submit_code,
        whole_code_state.clone(),
        disabled_input.clone(),
    );

    html!(
        <div class={classes!("nice-digit-code-container-view", class.clone())} id={id} code_length={digit_count.to_string()}>
            <div class={classes!("digit-code-container")}>
            {
                (0..digit_count)
                .map(
                    |index| html!(<CodeSingleDigitElement<PROFILE> profile={profile.clone()} key={index} whole_code={whole_code_state.clone()} index={index} focus_next={focus_next.clone()} focus_prev={focus_prev.clone()} enter_hit={enter_hit.clone()} disabled={*disabled_input} set_value={set_value.clone()}/>)
                ).collect::<Vec<_>>()
             }
            </div>
        </div>
    )
}
