/// This struct stores all flags that can be used to send commands to the component
///
/// With `change` you can create a builder and edit flags.
///
/// # Examples
/// Focus first digit and clear all digits when code gets submitted (using standard TOTP input)
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
#[derive(Debug, PartialEq, Default, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ControlFlags {
    pub(crate) focus: Option<usize>,
    pub(crate) clear: Option<()>,
}

/// Builds the `ControlFlags` struct
#[derive(Debug, PartialEq, Eq)]
pub struct ControlFlagsBuilder {
    pub(crate) focus: Option<usize>,
    pub(crate) clear: Option<()>,
}

impl ControlFlags {
    /// Create a builder with the current flag configuration
    pub fn change(&self) -> ControlFlagsBuilder {
        ControlFlagsBuilder {
            focus: self.focus,
            clear: self.clear,
        }
    }
}

impl ControlFlagsBuilder {
    /// Set the flag to focus first digit
    pub fn focus_first(mut self) -> Self {
        self.focus = Some(0);
        self
    }
    /// Unset focus flag
    pub fn unset_focus(mut self) -> Self {
        self.focus = None;
        self
    }
    /// Set flag to clear all digits
    pub fn clear(mut self) -> Self {
        self.clear = Some(());
        self
    }
    /// Unset flag to clear all digits
    pub fn unset_clear(mut self) -> Self {
        self.clear = None;
        self
    }
    /// Create `ControlFlags` for the current flag configuration
    pub fn apply(self) -> ControlFlags {
        ControlFlags {
            focus: self.focus,
            clear: self.clear,
        }
    }
}
