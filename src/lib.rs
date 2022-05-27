use std::fmt::Display;

pub trait SliceDisplay<'a, T: Display> {
    fn display(&'a self) -> SliceDisplayImpl<'a, T>;
}

pub struct SliceDisplayImpl<'a, T: Display> {
    slice: &'a [T],
}

impl<'a, T: Display> SliceDisplay<'a, T> for &'a [T] {
    fn display(&'a self) -> SliceDisplayImpl<'a, T> {
        SliceDisplayImpl { slice: self }
    }
}

impl<T: Display> SliceDisplay<'_, T> for Vec<T> {
    fn display(&self) -> SliceDisplayImpl<'_, T> {
        SliceDisplayImpl { slice: self }
    }
}

impl<'a, T: Display> Display for SliceDisplayImpl<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let elems = self.slice;
        write!(f, "[")?;
        match elems.split_last() {
            Some((last, elems)) => {
                for elem in elems {
                    write!(f, "{elem}, ")?;
                }
                write!(f, "{last}")?;
            }
            None => {}
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use crate::SliceDisplay;

    #[test]
    fn slice_display_empty() {
        let empty: Vec<u8> = vec![];
        assert_eq!(empty.display().to_string(), "[]");
    }

    #[test]
    fn slice_display_single() {
        let single = vec![1];
        assert_eq!(single.display().to_string(), "[1]");
    }

    #[test]
    fn slice_display_multiple() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(numbers.display().to_string(), "[1, 2, 3, 4, 5]");
    }
}
