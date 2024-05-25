use crate::inner_code_element::InnerCodeDigitInput;
use crate::{ControlFlags, DigitCode, DigitCodeProfile};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props<PROFILE: DigitCodeProfile + 'static> {
    #[cfg(feature = "default-id")]
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[cfg(not(feature = "default-id"))]
    pub id: AttrValue,
    #[prop_or_default]
    pub submit_code: Option<Callback<String>>,
    #[prop_or_default]
    pub profile: Option<PROFILE>,
    #[prop_or_default]
    pub flags: Option<UseStateHandle<ControlFlags>>,
    #[prop_or_default]
    pub class: Classes,
}

/// This is the general input component for a code of multiple digits.
///
/// - It needs a _UNIQUE_ id, if the `default-id` feature is enabled it will be selected automatically with `rand` crate
/// - Additionally you can provide the attribute `class` with extra html classes (yew classes!() macro)
/// - By default the generic profile type's default object will be used as `profile`.
///   If you want more control you can provide an instance as value of the `profile` attribute.
/// - The code the user wants to submit will be send to the `submit_code` callback
/// - If you want to perform additional actions like reset the whole edit you can provide a state with `ControlFlags` to influence the behaviour to `flags`.
///
/// # Examples
///
/// ```
/// use yew_digit_code::TotpInput;
/// use yew::prelude::*;
///
/// #[function_component(CodeComponent)]
/// fn code_component() -> Html {
///     let submit_code = Callback::from(move |code: String| {
///         log::info!("Submit: {code}");
///     });
///
///     html!(<TotpInput submit_code={submit_code}/>)
/// }
/// ```
///
/// To clear and focus first on submit
///
/// ```
/// use yew_digit_code::{ControlFlags, TotpInput};
/// use yew::prelude::*;
///
/// #[function_component(CodeComponentWR)]
/// fn code_component_with_reset() -> Html {
///     let flags = use_state_eq(|| ControlFlags::default());
///     let cloned_flags = flags.clone();
///     let submit_code = Callback::from(move |code| {
///         let flags = cloned_flags.clone();
///         flags.set(flags.change().focus_first().clear().apply());
///         log::info!("Submit: {code}");
///     });
///     html!(
///     <div style={"display: flex; height: 97vh"}>
///         <div style={"margin: auto;"}>
///             <TotpInput submit_code={submit_code} flags={flags}/>
///         </div>
///     </div>
///     )
/// }
/// ```
#[function_component(CodeDigitInput)]
pub fn code_digit_element<PROFILE: DigitCodeProfile + 'static>(
    Props {
        id,
        submit_code,
        flags,
        profile,
        class,
    }: &Props<PROFILE>,
) -> Html {
    let default_state_handle_flags = use_state_eq(ControlFlags::default);
    let flags = flags.clone().unwrap_or(default_state_handle_flags);

    #[cfg(feature = "default-id")]
    let id = {
        let id_state = use_state_eq(|| {
            use rand::{distributions::Alphanumeric, Rng};

            if let Some(id) = id {
                id.to_string()
            } else {
                let rng = rand::thread_rng();

                let random_alphanumeric: String = rng
                    .sample_iter(&Alphanumeric)
                    .take(50)
                    .map(char::from)
                    .collect();
                // must start with a letter
                format!("digit-code-edit-{random_alphanumeric}")
            }
        });
        (*id_state).clone()
    };

    let id = id.to_string();
    let id = if !id.starts_with(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']) {
        id
    } else {
        #[cfg(feature = "log")]
        log::warn!("Your choosen id isn't valid. It needs to start with a letter");
        format!("d-{id}")
    };

    let profile = profile.clone().unwrap_or_default();
    let flags = flags.clone();
    let submit_code = submit_code.clone();

    let whole_code_state = use_state(|| DigitCode::new(std::sync::Arc::new(profile.clone())));

    html!(<InnerCodeDigitInput<PROFILE> class={class} id={id} submit_code={submit_code} flags={flags} code={whole_code_state}/>)
}
