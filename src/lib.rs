//! # Matrix-MC
//!
//! 'matrix_mc' is a library which provides useful functions for
//! operating on and analyzing matrices of f64s

/// 2D Vector of f64s
pub type Matrix = Vec<Vec<f64>>;

pub fn identity(dim: usize) -> Matrix {
    let mut result = Vec::with_capacity(dim);

    for i in 0..dim {
        result.push(Vec::with_capacity(dim));
        for j in 0..dim {
            let a = if i == j { 1f64 } else { 0f64 };
            result[i].push(a);
        }
    }

    result
}

pub fn rref(a: &mut Matrix) {
    let mut i = 0usize;
    while i < a.len() && i < a[0].len() {
        let divisor = a[i][i];
        if divisor != 0f64 {
            for j in (0..a.len()).filter(|j| *j != i) {
                let dividend = a[j][i];
                for k in 0..a[j].len() {
                    a[j][k] -= a[i][k] * (dividend / divisor);
                }
            }
        }
        i += 1;
    }

    i = 0;
    while i < a.len() && i < a[0].len() {
        let divisor = a[i][i];
        if divisor != 0f64 {
            for j in 0..a[i].len() {
                a[i][j] /= divisor;
            }
        }
        i += 1;
    }
}

pub fn normalize(a: &mut Matrix) {
    for i in 0..a.len() {
        let mut sum = 0f64;

        for j in 0..a[0].len() {
            sum += a[j][i] * a[j][i];
        }
        let length = f64::sqrt(sum);

        for j in 0..a[0].len() {
            a[j][i] /= length;
        }
    }
}

/// Applies the provided funtion f to the matrix a,
/// returning a new matrix with the result of the
/// computation, leaving the original matrix unchanged
///
/// # Examples
///
/// ```
/// use matrix_mc::{rref, identity, dup, matrix, Matrix};
///
/// let arg: Matrix = matrix![2, 3, 4;
///                           5, 6, 7;
///                           9, 2, 0];
///
/// let reduced: Matrix = dup(rref, &arg);
///
/// assert_eq!(reduced, identity(3usize));
/// assert_ne!(arg, reduced);
/// ```
pub fn dup<F>(f: F, a: &Matrix) -> Matrix
where
    F: FnOnce(&mut Matrix),
{
    let mut temp = a.clone();
    f(&mut temp);
    temp
}

/// Provides a concise syntax for creating Matrices of f64s,
/// Creates a new Matrix, and adds every number seperated by a
/// comma as a f64, and adds a new row for each semicolon. if
/// rows are of unequal size, 0s are added to make up for the
/// difference
///
/// #Examples
///
/// ```
/// use matrix_mc::{matrix, Matrix};
///
/// let macro_matrix: Matrix = matrix!
///     [0, 1, 2;
///      1, 1;
///      2, 3, 4];
///
/// let manual_matrix: Matrix = {
///     let mut temp_matrix: Matrix = Vec::with_capacity(3);
///     
///     let mut temp_vector: Vec<f64> = Vec::with_capacity(3);
///     temp_vector.push(0f64);
///     temp_vector.push(1f64);
///     temp_vector.push(2f64);
///
///     temp_matrix.push(temp_vector);
///     
///     let mut temp_vector: Vec<f64> = Vec::with_capacity(3);
///     temp_vector.push(1f64);
///     temp_vector.push(1f64);
///     temp_vector.push(0f64);
///     
///     temp_matrix.push(temp_vector);
///     
///     let mut temp_vector: Vec<f64> = Vec::with_capacity(3);
///     temp_vector.push(2f64);
///     temp_vector.push(3f64);
///     temp_vector.push(4f64);
///
///     temp_matrix.push(temp_vector);
///
///     temp_matrix
/// };
///
/// assert_eq!(macro_matrix, manual_matrix);
/// ```
#[macro_export(local_inner_macros)]
macro_rules! matrix {
    ($( $( $x:expr ),* );* ) => {
        {
            let (row_max, col_max)  = {
                let mut temp_vec: Vec<usize> = Vec::with_capacity( count_tts!($( $( $x )* )*) );

                $(
                    let row_max = count_tts!($( $x )* );
                    temp_vec.push(row_max);
                )*

                (*temp_vec.iter().max().unwrap(), temp_vec.len())
            };

            let mut temp_matrix = Vec::with_capacity(col_max);

            $(
                let mut temp_vec = Vec::with_capacity(row_max);

                $(
                    temp_vec.push($x as f64);
                )*

                while temp_vec.len() < row_max {
                    temp_vec.push(0f64);
                }

                temp_matrix.push(temp_vec);
            )*

            temp_matrix
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {
        $sub
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! count_tts {
    ($($tts:tt)*) => {
        0usize $(+ matrix_mc::replace_expr!($tts 1usize))*
    };
}