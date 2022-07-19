# slicedisplay - lightweight `Display` for Vecs and slices

`slicedisplay` is a tiny `no-std` crate which supplies the `SliceDisplay` trait.

This trait extends `&[T]` and `Vec<T>` with the `display` method, which allows formatting without heap allocations.

## Usage

```rust
use slicedisplay::SliceDisplay;

let empty: Vec<u8> = Vec::new();
assert_eq!(empty.display().to_string(), "[]");

let single = Vec::from([1]);
assert_eq!(single.display().to_string(), "[1]");

let numbers = Vec::from([1, 2, 3, 4, 5]);
assert_eq!(numbers.display().to_string(), "[1, 2, 3, 4, 5]");
```

It's also possible to slightly customize the display.


```rust
use slicedisplay::SliceDisplay;

let hello: Vec<_> = "Hello".chars().collect();
assert_eq!(
    hello.display().delimiter(';').to_string(),
    "[H; e; l; l; o]"
);
assert_eq!(
    hello.display().terminator('{', '}').to_string(),
    "{H, e, l, l, o}"
);
assert_eq!(
    hello
        .display()
        .terminator('(', ')')
        .delimiter(';')
        .to_string(),
    "(H; e; l; l; o)"
);
```
