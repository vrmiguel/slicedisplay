#![no_std]
#![doc = include_str!("../README.md")]
extern crate alloc;

use core::fmt::Display;
#[cfg(not(feature = "multichar"))]
use core::fmt::Write;

use cfg_if::cfg_if;

/// Configurable Display implementation for slices and Vecs.
pub trait SliceDisplay<'a, T: Display> {
    #[must_use = "this does not display the slice, \
                  it returns an object that can be displayed"]
    fn display(&'a self) -> SliceDisplayImpl<'a, T>;
}

#[cfg(feature = "multichar")]
type ChromeLiteral = &'static str;

#[cfg(not(feature = "multichar"))]
type ChromeLiteral = char;

/// Helper struct for printing Vecs and slices.
#[derive(Clone, Copy)]
pub struct SliceDisplayImpl<'a, T: Display> {
    slice: &'a [T],
    terminators: (ChromeLiteral, ChromeLiteral),
    delimiter: ChromeLiteral,
    should_space: bool,
}

impl<'a, T: Display> SliceDisplayImpl<'a, T> {
    /// Configures the terminators to be used for the display.
    ///
    /// # Example
    ///
    /// ```rust
    /// use slicedisplay::SliceDisplay;
    /// use cfg_if::cfg_if;
    ///
    /// let hello: Vec<_> = "Hello".chars().collect();
    ///
    /// let (leftTerm, rightTerm);
    /// cfg_if! {
    ///     if #[cfg(feature = "multichar")] {
    ///         (leftTerm, rightTerm) = ("{", "}");
    ///     } else {
    ///         (leftTerm, rightTerm) = ('{', '}');
    ///     }
    /// }
    /// assert_eq!(hello.display().terminator(leftTerm, rightTerm).to_string(), "{H, e, l, l, o}");
    /// ```
    pub fn terminator(self, beginning: ChromeLiteral, ending: ChromeLiteral) -> Self {
        Self {
            terminators: (beginning, ending),
            ..self
        }
    }

    /// Configures the delimiter to be used for the display.
    ///
    /// # Example
    ///
    /// ```rust
    /// use slicedisplay::SliceDisplay;
    /// use cfg_if::cfg_if;
    ///
    /// let hello: Vec<_> = "Hello".chars().collect();
    ///
    /// let delim;
    /// cfg_if! {
    ///     if #[cfg(feature = "multichar")] {
    ///        delim = ";";
    ///     } else {
    ///        delim = ';';
    ///    }
    /// }
    /// assert_eq!(hello.display().delimiter(delim).to_string(), "[H; e; l; l; o]");
    /// ```
    pub fn delimiter(self, delimiter: ChromeLiteral) -> Self {
        Self { delimiter, ..self }
    }

    /// Sets whether additional spacing should be added between elements.
    ///
    /// True by default.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use slicedisplay::SliceDisplay;
    /// use cfg_if::cfg_if;
    ///
    /// let hello: Vec<_> = "Hello".chars().collect();
    ///
    /// let delim;
    /// cfg_if! {
    ///     if #[cfg(feature = "multichar")] {
    ///        delim = ";";
    ///     } else {
    ///        delim = ';';
    ///    }
    /// }
    /// assert_eq!(hello.display().delimiter(delim).to_string(), "[H; e; l; l; o]");
    /// assert_eq!(hello.display().delimiter(delim).should_space(false).to_string(), "[H;e;l;l;o]");
    /// ```
    pub fn should_space(self, should_space: bool) -> Self {
        Self {
            should_space,
            ..self
        }
    }
}

impl<T: Display, A> SliceDisplay<'_, T> for A
where
    A: AsRef<[T]>,
{
    #[cfg(feature = "multichar")]
    fn display(&self) -> SliceDisplayImpl<'_, T> {
        SliceDisplayImpl {
            slice: self.as_ref(),
            terminators: ("[", "]"),
            delimiter: ",",
            should_space: true,
        }
    }
    #[cfg(not(feature = "multichar"))]
    fn display(&self) -> SliceDisplayImpl<'_, T> {
        SliceDisplayImpl {
            slice: self.as_ref(),
            terminators: ('[', ']'),
            delimiter: ',',
            should_space: true,
        }
    }
}

impl<'a, T: Display> Display for SliceDisplayImpl<'a, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let (beginning, ending) = self.terminators;
        let delimiter = self.delimiter;
        let spacing = if self.should_space { " " } else { "" };

        cfg_if! {
            if #[cfg(feature = "multichar")] {
                f.write_str(beginning)?;
            } else {
                f.write_char(beginning)?;
            }
        }

        if let Some((last, elems)) = self.slice.split_last() {
            for elem in elems {
                write!(f, "{elem}{delimiter}{spacing}")?;
            }
            write!(f, "{last}")?;
        }

        cfg_if! {
            if #[cfg(feature = "multichar")] {
                f.write_str(ending)
            } else {
                f.write_char(ending)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::{string::ToString, vec::Vec};

    use crate::SliceDisplay;

    extern crate alloc;

    #[test]
    fn slice_display_empty() {
        // Slighly redundant in order to ensure that we can
        // call Display on any AsRef<T>
        let empty: Vec<u8> = Vec::new();
        let empty_array: [u8; 0] = [];
        let empty_slice: &[u8] = &[];

        assert_eq!(empty.display().to_string(), "[]");
        assert_eq!(empty_array.display().to_string(), "[]");
        assert_eq!(empty_slice.display().to_string(), "[]");
    }

    #[test]
    fn slice_display_single() {
        let single = [1];
        assert_eq!(single.display().to_string(), "[1]");
    }

    #[test]
    fn slice_display_multiple() {
        let numbers = [1, 2, 3, 4, 5];
        assert_eq!(numbers.display().to_string(), "[1, 2, 3, 4, 5]");
    }

    #[test]
    #[cfg(not(feature = "multichar"))]
    fn slice_display_custom_delimiter() {
        let numbers = [1, 2, 3, 4, 5];
        assert_eq!(
            numbers.display().delimiter(';').to_string(),
            "[1; 2; 3; 4; 5]"
        );
        assert_eq!(
            numbers
                .display()
                .delimiter('-')
                .should_space(false)
                .to_string(),
            "[1-2-3-4-5]"
        );
    }

    #[test]
    #[cfg(not(feature = "multichar"))]
    fn slice_display_custom_terminators() {
        let numbers = [1, 2, 3, 4, 5];
        assert_eq!(
            numbers.display().terminator('{', '}').to_string(),
            "{1, 2, 3, 4, 5}"
        );
        assert_eq!(
            numbers
                .display()
                .terminator('{', '}')
                .should_space(false)
                .delimiter(';')
                .to_string(),
            "{1;2;3;4;5}"
        );
    }

    #[test]
    #[cfg(feature = "multichar")]
    fn slice_display_custom_multichar_chrome() {
        let numbers = [1, 2, 3, 4, 5];
        assert_eq!(
            numbers
                .display()
                .terminator("{{", "}}")
                .should_space(false)
                .delimiter(" ... ")
                .to_string(),
            "{{1 ... 2 ... 3 ... 4 ... 5}}"
        );
    }
}
