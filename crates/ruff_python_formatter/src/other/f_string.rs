use super::interpolated_string_element::FormatInterpolatedStringElement;
use crate::other::interpolated_string::{InterpolatedStringContext, InterpolatedStringLayout};
use crate::prelude::*;
use crate::string::{StringNormalizer, StringQuotes};
use ruff_formatter::write;
use ruff_python_ast::{FString, StringFlags};

/// Formats an f-string which is part of a larger f-string expression.
///
/// For example, this would be used to format the f-string part in `"foo" f"bar {x}"`
/// or the standalone f-string in `f"foo {x} bar"`.
#[derive(Default)]
pub struct FormatFString;

impl FormatNodeRule<FString> for FormatFString {
    fn fmt_fields(&self, item: &FString, f: &mut PyFormatter) -> FormatResult<()> {
        let normalizer = StringNormalizer::from_context(f.context());

        let string_kind = normalizer.choose_quotes(item.into()).flags();

        let context = InterpolatedStringContext::new(
            string_kind,
            InterpolatedStringLayout::from_interpolated_string_elements(
                &item.elements,
                f.context().source(),
            ),
        );

        // Starting prefix and quote
        let quotes = StringQuotes::from(string_kind);
        write!(f, [string_kind.prefix(), quotes])?;

        for element in &item.elements {
            FormatInterpolatedStringElement::new(element, context).fmt(f)?;
        }

        // Ending quote
        quotes.fmt(f)
    }
}
