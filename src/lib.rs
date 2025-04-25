pub mod root;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nrt() {
        let result = root::nrt(243, 5);
        assert_eq!(result, 3_f64);
    }

    #[test]
    fn sqrt() {
        let result = root::sqrt(4);
        assert_eq!(result, 2_f64);
    }

    #[test]
    fn cbrt() {
        let result = root::cbrt(8);
        assert_eq!(result, 2_f64);
    }
}
