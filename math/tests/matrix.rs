#[cfg(test)]
mod test {
    use yagt_math::matrix::*;

    #[test]
    fn constants() {
        let m = Matrix::<f32, 3, 2>::zeros();
        for x in 0..3 {
            for y in 0..2 {
                assert_eq!(m.get(x, y), 0.0);
            }
        }

        let m = Matrix::<f32, 3, 2>::ones();
        for x in 0..3 {
            for y in 0..2 {
                assert_eq!(m.get(x, y), 1.0);
            }
        }

        let m = Matrix::<f32, 2, 2>::identity();
        assert_eq!(m.get(0, 0), 1.0);
        assert_eq!(m.get(1, 1), 1.0);
        assert_eq!(m.get(1, 0), 0.0);
        assert_eq!(m.get(0, 1), 0.0);
    }

    #[test]
    #[rustfmt::skip]
    fn elem_access() {
        let m = Mat22::from_row(&[
                                        1.0, 2.0,
                                        3.0, 4.0]);
        assert_eq!(m.get(0, 0), 1.0);
        assert_eq!(m.get(1, 0), 2.0);
        assert_eq!(m.get(0, 1), 3.0);
        assert_eq!(m.get(1, 1), 4.0);

        let m = Mat22::from_col(&[
                                        1.0, 2.0,
                                        3.0, 4.0]);
        assert_eq!(m.get(0, 0), 1.0);
        assert_eq!(m.get(1, 0), 3.0);
        assert_eq!(m.get(0, 1), 2.0);
        assert_eq!(m.get(1, 1), 4.0);

        let m = Matrix::<f32, 3, 2>::from_row(&[
                                    1.0, 2.0, 3.0,
                                    4.0, 5.0, 6.0]);
        assert_eq!(m.get(0, 0), 1.0);
        assert_eq!(m.get(1, 0), 2.0);
        assert_eq!(m.get(2, 0), 3.0);
        assert_eq!(m.get(0, 1), 4.0);
        assert_eq!(m.get(1, 1), 5.0);
        assert_eq!(m.get(2, 1), 6.0);
    }

    #[test]
    #[rustfmt::skip]
    fn arithmetics() {
        let m1 = Matrix::<f32, 3, 2>::from_row(&[
                                        1.0, 2.0, 3.0,
                                        4.0, 5.0, 6.0]);
        let m2 = Matrix::<f32, 3, 2>::from_row(&[
                                        -1.0, -2.0, -3.0,
                                        -4.0, -5.0, -6.0]);
        let result = m1 + m2;
        assert_eq!(result, Matrix::<f32, 3, 2>::zeros());

        let result = m1 - m2;
        assert_eq!(result, Matrix::<f32, 3, 2>::from_row(&[
            2.0, 4.0, 6.0,
            8.0, 10.0, 12.0,
        ]));

        let result = m1 * 3.0f32;
        assert_eq!(result.get(0, 0), 3.0);
        assert_eq!(result.get(1, 0), 6.0);
        assert_eq!(result.get(2, 0), 9.0);
        assert_eq!(result.get(0, 1), 12.0);
        assert_eq!(result.get(1, 1), 15.0);
        assert_eq!(result.get(2, 1), 18.0);

        let result = m1 / 2.0f32;
        assert_eq!(result.get(0, 0), 0.5);
        assert_eq!(result.get(1, 0), 1.0);
        assert_eq!(result.get(2, 0), 1.5);
        assert_eq!(result.get(0, 1), 2.0);
        assert_eq!(result.get(1, 1), 2.5);
        assert_eq!(result.get(2, 1), 3.0);

        let result = m1.mul_each(m2);
        assert_eq!(result.get(0, 0), -1.0);
        assert_eq!(result.get(1, 0), -4.0);
        assert_eq!(result.get(2, 0), -9.0);
        assert_eq!(result.get(0, 1), -16.0);
        assert_eq!(result.get(1, 1), -25.0);
        assert_eq!(result.get(2, 1), -36.0);

        let m2 = m2.transpose();
        assert_eq!(m2, Matrix::<f32, 2, 3>::from_col(&[
                                        -1.0, -2.0, -3.0,
                                        -4.0, -5.0, -6.0]));

        let result = m1 * m2;
        assert_eq!(result, Matrix::<f32, 2, 2>::from_row(&[
                                        -14.0, -32.0,
                                        -32.0, -77.0]));

    }

    #[test]
    fn vector_init() {
        let v = Vec3::from_xyz(1.0, 2.0, 3.0);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
    }

    #[test]
    fn vector_special_arithmetic() {
        let v1 = Vec3::from_xyz(1.0, 2.0, 3.0);
        let v2 = Vec3::from_xyz(3.0, -2.0, 3.0);
        assert_eq!(v1.dot(&v2), 8.0);

        let v = v1.cross(&v2);
        assert_eq!(v, Vec3::from_xyz(12.0, 6.0, -8.0));

        let v1 = Vec2::from_xy(1.0, 2.0);
        let v2 = Vec2::from_xy(3.0, 4.0);
        assert_eq!(v1.dot(&v2), 11.0);
        assert_eq!(v1.cross(&v2), -2.0);
    }

    #[test]
    fn det() {
        let m = Mat22::from_row(&[1.0, 2.0, 3.0, 4.0]);
        assert_eq!(m.det(), -2.0);
    }
}
