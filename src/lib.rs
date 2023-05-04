type Matrix = Vec<Vec<f64>>;

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

#[macro_export]
macro_rules! matrix {
    ($($($x:expr),*);*) => {
        {
            let mut temp_matrix = Vec::new();

            $(
                temp_matrix.push(Vec::new());
                $(
                    temp_matrix.last_mut().unwrap().push($x as f64);
                )*
                
            )*

            temp_matrix
        }
    };
}