pub fn nrt(radicand: impl Into<f64>, root: impl Into<f64>) -> f64 {
    radicand.into().powf(1.0 / root.into())
}

pub fn sqrt(radicand: impl Into<f64>) -> f64 {
    radicand.into().sqrt()
}

pub fn cbrt(radicand: impl Into<f64>) -> f64 {
    nrt(radicand, 3)
}

#[cfg(test)]
mod tests {
    #[test]
    fn nrt() {
        let result = super::nrt(243, 5);
        assert_eq!(result, 3_f64);
    }

    #[test]
    fn sqrt() {
        let result = super::sqrt(4);
        assert_eq!(result, 2_f64);
    }

    #[test]
    fn cbrt() {
        let result = super::cbrt(8);
        assert_eq!(result, 2_f64);
    }
}
