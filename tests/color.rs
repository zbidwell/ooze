#[cfg(test)]
mod tests {
    use ooze::color::*;

    #[test]
    fn test_normfloat() {
        assert_eq!(NormFloat::from(0.6f32), NormFloat::from(0.6f64));
        assert_eq!(NormFloat::from(3.0), NormFloat::from(1.0));
        assert_eq!(NormFloat::from(-1.0), NormFloat::from(0.0));
    }

    #[test]
    fn test_color_new() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        let c1 = Color::new(0.5, 0.5, 0.5, 1.0);
        let c2 = Color::new(NormFloat::from(0.5), NormFloat::from(0.5), NormFloat::from(0.5), NormFloat::from(1.0));
        assert_eq!(c1, c2);
    }

    #[test]
    fn test_is_opaque() {
        let c1 = Color::new(0.1, 0.2, 0.3, 1.0);
        assert_eq!(c1.is_opaque(), true);
        let c2 = Color::new(0.1, 0.2, 0.3, 0.5);
        assert_eq!(c2.is_opaque(), false);
    }

    #[test]
    fn test_as_array() {
        let c1 = Color::new(0.1, 0.2, 0.3, 1.0);
        let arr: [f32; 4] = [0.1, 0.2, 0.3, 1.0];
        assert_eq!(c1.as_array(), arr);
    }

    #[test]
    fn test_from_array() {
        let arr1 = [0.1, 0.2, 0.3];
        let arr2 = [0.1, 0.2, 0.3, 0.4];
        let c1: Color = arr1.into();
        let c2 = Color::from(arr2);
        assert_eq!(c1, Color::new(0.1, 0.2, 0.3, 1.0));
        assert_eq!(c2, Color::new(0.1, 0.2, 0.3, 0.4));
    }
}