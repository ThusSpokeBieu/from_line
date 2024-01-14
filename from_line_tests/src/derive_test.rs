use from_line_derive::FromLine;

#[derive(Debug, PartialEq, Eq, FromLine)]
struct Person {
    #[from_line(0..15)]
    name: String,
    #[from_line(15..18)]
    age: u8,
    #[from_line(18..30)]
    city: String,
}

#[cfg(test)]
mod derive_tests {
    use crate::derive_test::Person;

    #[test]
    fn test_from_line() {
        let line1 = "      John Doe 30    New York";
        let expected1 = Person {
            name: "John Doe".to_string(),
            age: 30,
            city: "New York".to_string(),
        };

        assert_eq!(Person::from_line(line1), expected1);

        let line2 = "Jane Smith     25  Los Angeles";
        let expected2 = Person {
            name: "Jane Smith".to_string(),
            age: 25,
            city: "Los Angeles".to_string(),
        };
        assert_eq!(Person::from_line(line2), expected2);
    }
}
