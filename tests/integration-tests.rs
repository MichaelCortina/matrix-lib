use matrix_lib::{identity, rref, matrix, normalize, dup};

#[test]
fn init_identity_3x3() {
    let expected = 
        matrix![1, 0, 0;
                0, 1, 0;
                0, 0, 1];
    
    let actual = identity(3usize);

    assert_eq!(expected, actual);
}

#[test]
fn rref_independent_3x3() {

    let expected = identity(3);

    let mut actual = 
        matrix![2,  3,  4 ;
                3,  1,  5 ;
                7,  11, 22];
    rref(&mut actual);

    assert_eq!(expected, actual);
}

#[test]
fn rref_dependent_3x3() {
    let expected = 
        matrix![1, 0, 0;
                0, 1, 1;
                0, 0, 0];
    
    let mut actual = 
        matrix![1, 1, 1;
                0, 1, 1;
                1, 1, 1];
    rref(&mut actual);
    
    assert_eq!(expected, actual);
}


#[test]
fn rref_dependent_3x2() {
    let expected = 
        matrix![1, 0;
                0, 1;
                0, 0];

    let mut actual = 
        matrix![5,  4;
                0,  3;
                10, 8];
    rref(&mut actual);

    assert_eq!(expected, actual);    
}

#[test]
fn normalize_3x3() {
    let expected =
        matrix![1f64/f64::sqrt(2f64), 1f64/f64::sqrt(5f64), 1;
                1f64/f64::sqrt(2f64),                    0, 0;
                                   0, 2f64/f64::sqrt(5f64), 0];
    
    let mut actual = 
        matrix![1, 1, 1;
                1, 0, 0;
                0, 2, 0];
    normalize(&mut actual);

    assert_eq!(expected, actual);
}

#[test]
fn dup_rref_3x3() {
    let expected = matrix!
        [1, 0, 0;
         0, 1, 0;
         0, 0, 1];
    
    let input = matrix! 
        [5, 4, 3;
         2, 1, 9;
         8, 7, 6];

    let actual = dup(rref, &input);

    assert_eq!(expected, actual);
}