mod fraction;

#[cfg(test)]
mod tests {
    use crate::fraction::Fraction;
    use std::str::FromStr;

    #[test]
    fn test_inverse() {
        assert_eq!(
            crate::fraction::Fraction::new(1, 2).inverse_probability(),
            crate::fraction::Fraction::new(1, 2)
        )
    }

    #[test]
    fn test_parse() {
        let s: Fraction<i32> = Fraction::from_str("1/2").unwrap();
        assert_eq!(s, crate::fraction::Fraction::new(1, 2))
    }
}
