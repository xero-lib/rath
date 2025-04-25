pub fn nrt(radicand: f64, root: f64) -> f64 {
    radicand.powf(1.0 / root)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = nrt(8.0, 3.0);
        assert_eq!(result, 2.0);
    }
}
