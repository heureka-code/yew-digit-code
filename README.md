This crate provides a `yew` component for digit based codes like TOTP where the user gets
a nice looking collection of LEN edits with length one to enter a code with a fixed alphabet.

Documentation: https://heureka-code.github.io/yew-digit-code/

A runnable example: https://github.com/heureka-code/yew-digit-code-example

This crate uses the `unicode-segmentation` crate as a dependency (behind a feature flag).
The signatures of this crate's methods depend highly on this flag so be aware of that.
`unicode-segmentation` is needed if your alphabet contains characters that can't be represented in a single rust char.
If your codes only use ASCII you can disable this feature.

# Features
- `log` _(default)_: adds few log messages using `log` crate
- `default-id` _(default)_: generates a random id (`rand` crate) where a unique html id is needed. When disabled the user has to choose one.
- `unicode-segmentation` _(default)_: needed for unicode alphabets (changes method signatures)
- `itertools`: no interface changes. Some implementations differ (joining strings and chars). Itertools is more efficient for sure
- `serde`: not needed but adds some implementations
- `yew-hooks` _(default)_: activates `yew-hooks` to provide a callback that's called after initialization of the component

# Style
An example SCSS style sheet is displayed below. It's best for digit codes with a length that's a multiple of 3

```scss
$primary-color: lime;

body {
    background-color: rgb(70, 70, 206);
}

.nice-digit-code-container-view {

    $digit-background: rgba(50, 50, 50);
    $digit-color: white;
    $digit-color-hover: rgba(80, 80, 80);
    $digit-focus-color: $primary-color;

    $digit-width: 3.39rem;
    $digit-padding: 0.39rem;
    $digit-separator: 0.5rem;
    $digit-big-separator: (
        1.25 * $digit-separator
    );
    $digit-font-size: 3em;

display: grid;
grid-template-columns: auto;

.digit-code-container {
    display: grid;
    row-gap: calc(1.5 * $digit-separator);
    column-gap: $digit-separator;
    justify-content: center;

    input {
        text-align: center;

        width: 1em;
        padding-top: $digit-padding;
        padding-bottom: $digit-padding;

        font-size: $digit-font-size;
        background-color: $digit-background;
        color: $digit-color;
        border: none;
        border-radius: 0.5rem;
        font-family: monospace;
    }

    input:focus {
        outline: 0.2rem solid $primary-color;
    }

    input:hover {
        background-color: $digit-color-hover;
    }

    grid-auto-flow: column;
    $media-min-width: calc(($digit-width + $digit-separator + $digit-big-separator) * 10);
    $media-max-width: calc($media-min-width - 0.001em);

    @media only screen and (min-width: $media-min-width) {
        input:nth-child(3n+0) {
            margin-right: $digit-big-separator;
        }

        input:last-child {
            margin: 0 0 0 0;
            padding: 0;
        }
    }

    @media only screen and (max-width: $media-max-width) {
        grid-template-columns: auto auto auto;
        grid-auto-flow: row;
    }

}
}
```
