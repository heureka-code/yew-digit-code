//! This crate provides a `yew` component for digit based codes like TOTP where the user gets
//! a nice looking collection of LEN edits – each with length one – to enter a code with a fixed alphabet.
//!
//! Documentation: <https://heureka-code.github.io/yew-digit-code/>
//!
//! A runnable example: <https://github.com/heureka-code/yew-digit-code-example>
//!
//! This crate uses the `unicode-segmentation` crate as a dependency (behind a feature flag).
//! The signatures of this crate's methods depend highly on this flag so be aware of that.
//! `unicode-segmentation` is needed if your alphabet contains characters that can't be represented in a single rust char.
//! If your codes only use ASCII you can disable this feature.
//!
//! # Features
//! - `log` _(default)_: adds few log messages using `log` crate
//! - `default-id` _(default)_: generates a random id (`rand` crate) where a unique html id is needed. When disabled the user has to choose one.
//! - `unicode-segmentation`: needed for unicode alphabets (changes method signatures)
//! - `itertools`: no interface changes. Some implementations differ (joining strings and chars). Itertools is more efficient for sure
//! - `serde`: not needed but adds some implementations
//! - `yew-hooks` _(default)_: activates `yew-hooks` to provide a callback that's called after initialization of the component

mod code_element;
mod control_flags;
mod digit_code_status;
pub mod extra;
mod focus_offset;
mod inner_code_element;
mod predefined;
mod profile;
mod single_digit_element;

use digit_code_status::ArcStrOrChar;
use digit_code_status::DigitCode;
use profile::RefStrOrChar;

pub use code_element::CodeDigitInput;
pub use control_flags::ControlFlags;
pub use predefined::TotpCodeProfile;
pub use profile::DigitCodeProfile;

/// This is a type alias for a `CodeDigitInput` with a `TotpCodeProfile` of the given length.
/// `LENGTH` defaults to 6
///
/// # Examples
///
/// ```
/// use yew::prelude::*;
/// use yew_digit_code::TotpInput;
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
/// With explicit length
/// ```
/// use yew::prelude::*;
/// use yew_digit_code::TotpInput;
///
/// #[function_component(CodeComponentWel)]
/// fn code_component_wel() -> Html {
///     let submit_code = Callback::from(move |code: String| {
///         log::info!("Submit: {code}");
///     });
///
///     html!(<TotpInput<8> submit_code={submit_code}/>)
/// }
/// ```
pub type TotpInput<const LENGTH: usize = 6> = CodeDigitInput<TotpCodeProfile<LENGTH>>;
