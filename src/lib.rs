#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use core::fmt::{Display, Write};

/// Configurable Display implementation for slices and Vecs.
pub trait SliceDisplay<'a, T: Display> {
    #[must_use = "this does not display the slice, \
                  it returns an object that can be displayed"]
    fn display(&'a self) -> SliceDisplayImpl<'a, T>;
}

/// Helper struct for printing Vecs and slices.
pub struct SliceDisplayImpl<'a, T: Display> {
    slice: &'a [T],
    terminators: (char, char),
    delimiter: char,
}

impl<'a, T: Display> SliceDisplayImpl<'a, T> {
    /// Configures the terminators to be used for the display.
    ///
    /// # Example
    ///
    /// ```rust
    /// use slicedisplay::SliceDisplay;
    ///
    /// let hello: Vec<_> = "Hello".chars().collect();
    ///
    /// assert_eq!(hello.display().terminator('{', '}').to_string(), "{H, e, l, l, o}");
    /// ```
    pub fn terminator(self, beginning: char, ending: char) -> Self {
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
    ///
    /// let hello: Vec<_> = "Hello".chars().collect();
    ///
    /// assert_eq!(hello.display().delimiter(';').to_string(), "[H; e; l; l; o]");
    /// ```
    pub fn delimiter(self, delimiter: char) -> Self {
        Self { delimiter, ..self }
    }
}

impl<'a, T: Display> SliceDisplay<'a, T> for &'a [T] {
    fn display(&'a self) -> SliceDisplayImpl<'a, T> {
        SliceDisplayImpl {
            slice: self,
            terminators: ('[', ']'),
            delimiter: ',',
        }
    }
}

impl<T: Display> SliceDisplay<'_, T> for Vec<T> {
    fn display(&self) -> SliceDisplayImpl<'_, T> {
        SliceDisplayImpl {
            slice: self,
            terminators: ('[', ']'),
            delimiter: ',',
        }
    }
}

impl<'a, T: Display> Display for SliceDisplayImpl<'a, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let (beginning, ending) = self.terminators;
        let delimiter = self.delimiter;

        f.write_char(beginning)?;
        if let Some((last, elems)) = self.slice.split_last() {
            for elem in elems {
                write!(f, "{elem}{delimiter} ")?;
            }
            write!(f, "{last}")?;
        }

        f.write_char(ending)
    }
}

#[cfg(test)]
mod tests {
    use alloc::{string::ToString, vec::Vec};

    use crate::SliceDisplay;

    extern crate alloc;

    #[test]
    fn slice_display_empty() {
        let empty: Vec<u8> = Vec::new();
        assert_eq!(empty.display().to_string(), "[]");
    }

    #[test]
    fn slice_display_single() {
        let single = Vec::from([1]);
        assert_eq!(single.display().to_string(), "[1]");
    }

    #[test]
    fn slice_display_multiple() {
        let numbers = Vec::from([1, 2, 3, 4, 5]);
        assert_eq!(numbers.display().to_string(), "[1, 2, 3, 4, 5]");
    }
}
