pub fn nrt(radicand: impl Into<f64>, root: impl Into<f64>) -> f64 {
    radicand.into().powf(1.0 / root.into())
}

pub fn sqrt(radicand: impl Into<f64>) -> f64 {
    radicand.into().sqrt()
}

pub fn cbrt(radicand: impl Into<f64>) -> f64 {
    nrt(radicand, 3)
}
