pub struct Matrix {}

pub fn mul<'a>(a: &'a [&'a [f64]], b: &'a [&'a [f64]]) -> MatrixResult<Vec<Vec<f64>>> {
    use MatrixResult::*;
    if a.first().unwrap().len() != b.len() {
        return Mismatch;
    }

    if a.is_empty() ^ b.is_empty() {
        return Empty;
    } else if a.is_empty() || b.is_empty() {
        return Ok(a
            .iter()
            .map(|i| i.iter().map(ToOwned::to_owned).collect())
            .collect());
    }

    //    match check_matrix(a) {
    //        e @ (Empty | Mismatch | Invalid) => return e,
    //        _ => {}
    //    }

    let mut out = Vec::<Vec<f64>>::new();

    for i in 0..a.len() {
        out.push(vec![f64::default(); a.len()]); // adds vec, making i a valid index
        for j in 0..a.len() {
            let mut sum = 0.0;
            for b_idx in 0..a[0].len() {
                sum += a[i][b_idx] * b[b_idx][j];
            }

            out[i][j] = sum;
        }
    }

    MatrixResult::Ok(out)
}

pub enum MatrixResult<T> {
    Ok(T),
    Empty,
    Mismatch,
    Invalid,
}

// ugly
// fn check_matrix<'a>(
//     matrix: impl Iterator<Item = impl Iterator> + Clone,
// ) -> MatrixResult<impl Iterator<Item = impl Iterator>> {
//     return if matrix.clone().count() == 0 {
//         MatrixResult::Empty
//     } else if matrix
//         .clone()
//         .all(|i| i.count() == matrix.clone().next().unwrap().count())
//     {
//         MatrixResult::Ok(matrix)
//     } else {
//         MatrixResult::Invalid
//     };
// }

fn check_matrix<'a>(
    mut matrix: impl Iterator<Item = impl Iterator> + Clone,
) -> MatrixResult<impl Iterator<Item = impl Iterator>> {
    let Some(first) = matrix.next() else {
        return MatrixResult::Empty;
    };
    let len = first.count();
    if matrix.all(|i| i.count() == len) {
        MatrixResult::Ok(matrix)
    } else {
        MatrixResult::Invalid
    }
}
