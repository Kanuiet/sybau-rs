use crate::evaluate;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(evaluate("40 + 30"), Ok(70.0));
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(evaluate("40 - 20"), Ok(20.0));
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(evaluate("5*20"), Ok(100.0));
    }

    #[test]
    fn test_division() {
        assert_eq!(evaluate("10/40"), Ok(0.25));
    }

    #[test]
    fn test_power() {
        assert_eq!(evaluate("2 ^ 3 ^ 2"), Ok(512.0));
    }

    #[test]
    fn test_unary_neg() {
        assert_eq!(evaluate("----10"), Ok(10.0));
        assert_eq!(evaluate("-(-(-(-(10))))"), Ok(10.0));
    }

    #[test]
    fn test_mixed_operators() {
        assert_eq!(evaluate("5(3/2)^2 * 6(10) / 2^2"), Ok(168.75));
        assert_eq!(evaluate("3 ^ 6 - (1.5 * 6 / 2 + 1) + 5 / 2 + 1"), Ok(727.0));
        assert_eq!(evaluate("--10 + -30 * 20^(1/2) - (3^4^(1/2)) / (2 * (2^2^100^(1/2)))"), Ok(-124.1640786499874));
    }
}
