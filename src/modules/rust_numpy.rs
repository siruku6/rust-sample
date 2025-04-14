use ndarray::{arr2, array, Array2, ArrayD, Axis};
// use std::panic;

pub fn ndarray_sample() -> Array2<f32> {
    let a: Array2<f32> = Array2::eye(3);
    assert_eq!(&a, array![[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);

    let b: Array2<f32> = arr2(&[[1., 2., 3.], [3., 4., 5.], [4., 5., 6.]]);

    let res_adamar = &a * &b;

    // dot product
    let res_dot: Array2<f32> = a.dot(&b);

    println!("res_adamar:\n{res_adamar:?}");
    println!("res_dot:\n{res_dot:?}");

    // 列方向に話をとったとき（Axis(1)）の合計 (行ごとの合計値)
    let col_sums = res_dot.sum_axis(Axis(1));
    println!("col_sums:\n{}", col_sums);

    return a;
}
