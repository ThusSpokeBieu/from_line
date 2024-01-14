[LEIA EM PT-BR](README_PT-BR.md)

This is a dummy lib as study case.

This crate provides a procedural macro `FromLine` for deriving the `from_line::traits::from_string::FromString` trait implementation for a struct. The `FromLine` trait allows you to easily convert a line of text into a struct instance by parsing the fields from the line.

# Usage

To use this crate, you need to add the `from_line_derive` dependency to your `Cargo.toml` file:

```toml
[dependencies]
from_line_derive = "0.1"
```

Then, you can use the `FromLine` derive macro on your struct:

```rust
use from_line_derive::FromLine;

#[derive(FromLine)]
struct Person {
    #[from_line(0..5)]
    name: String,

    #[from_line(6..10)]
    age: u32,

    #[from_line(11..50)]
    address: String,
}
```

The `#[from_line(X..Y)]` attribute specifies the range of characters in the line that should be used to populate the corresponding field. The range is inclusive of the start index `X` and exclusive of the end index `Y`.

Finally, you can use the `from_line` method to create an instance of the struct from a line of text:

```rust
let line = "John  25  123 Main St";
let person = Person::from_line(line);

assert_eq!(person.name, "John");
assert_eq!(person.age, 25);
assert_eq!(person.address, "123 Main St");
```

You can also do your custom implementation from the trait FromString to create your types conversions from a string text.
