use wasm_bindgen::JsCast;
use web_sys::{window, Document, HtmlInputElement};

pub fn document() -> Option<Document> {
    window()?.document()
}

#[derive(Debug, Clone)]
pub enum FocusOffset {
    Next,
    Previous,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FocusResult<T = ()> {
    Ok(T),
    TooBig,
    TooLow,
    NoDocument,
}

impl FocusOffset {
    /// Adds or subtracts 1 to the current index
    ///
    /// maximum is the last index that exists
    pub fn process(&self, current: usize, maximum: usize) -> FocusResult<usize> {
        match self {
            FocusOffset::Next => {
                if current == maximum {
                    FocusResult::TooBig
                } else {
                    FocusResult::Ok(current + 1)
                }
            }
            FocusOffset::Previous => {
                if current == 0 {
                    FocusResult::TooLow
                } else {
                    FocusResult::Ok(current - 1)
                }
            }
        }
    }
}

pub fn focus_offset(
    id: String,
    element_count: usize,
    offset: FocusOffset,
) -> impl Fn(usize) -> FocusResult<()> {
    move |i: usize| {
        let index = offset.process(i, element_count - 1);

        #[cfg(feature = "log")]
        log::trace!("Focus offset is called: {offset:?}, index={i} total={element_count}");

        let index = match index {
            FocusResult::Ok(index) => index,
            FocusResult::TooBig => return FocusResult::TooBig,
            FocusResult::TooLow => return FocusResult::TooLow,
            // can't occur at this point
            FocusResult::NoDocument => return FocusResult::NoDocument,
        };

        if let Some(document) = document() {
            let node = document.query_selector(&format!("#{} input[data-index=\"{}\"]", id, index));
            if let Ok(Some(node)) = node {
                let node = node.dyn_into::<HtmlInputElement>().ok();
                if let Some(node) = node {
                    let _ = node.focus();
                }
            } else {
                #[cfg(feature = "log")]
                log::error!("An error occured while focussing a node: {node:?}");
                return FocusResult::NoDocument;
            }
            FocusResult::Ok(())
        } else {
            #[cfg(feature = "log")]
            log::error!("The focus method was called before a document was ready. Therefore the call will be ignored and this could lead to unexpected behaviour.");
            FocusResult::NoDocument
        }
    }
}
