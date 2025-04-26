pub mod matrix;
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

    #[test]
    fn matrix_mul() {
        let a1 = [1.0, 2.0, 3.0];
        let a2 = [4.0, 5.0, 6.0];
        let b1 = [10.0, 11.0];
        let b2 = [20.0, 21.0];
        let b3 = [30.0, 31.0];

        let a = [&a1[..], &a2[..]];
        let b = [&b1[..], &b2[..], &b3[..]];

        let matrix::MatrixResult::Ok(result) = matrix::mul(&a, &b) else {
            panic!("Failure");
        };

        assert_eq!(result, vec![vec![140.0, 146.0], vec![320.0, 335.0]]);
    }
}
