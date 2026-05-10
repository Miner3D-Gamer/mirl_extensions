#[cfg(test)]
mod tests {
    #![allow(trivial_numeric_casts)]
    use crate::*;

    /// Macro to test all applicable traits for a given numeric type
    macro_rules! test_all_traits {
        // For signed integer types
        (signed_int: $type:ty, $test_val:expr) => {
            pastey::paste! {
                #[test]
                fn [<test_ $type _sign>]() {
                    let pos: $type = $test_val;
                    let neg: $type = -$test_val;
                    let zero: $type = 0;

                    assert_eq!(pos.sign(), 1);
                    assert_eq!(neg.sign(), -1);
                    assert_eq!(zero.sign(), 0);
                }

                #[test]
                fn [<test_ $type _abs>]() {
                    let val: $type = -$test_val;
                    assert_eq!(val.abs(), $test_val);
                }

                #[test]
                fn [<test_ $type _saturating_ops>]() {
                    let a: $type = $test_val;
                    let b: $type = 2;

                    let _ = a.saturating_add(b);
                    let _ = a.saturating_sub(b);
                    let _ = a.saturating_mul(b);
                    let _ = a.saturating_abs();
                    let _ = a.saturating_neg();

                    // Test saturation at bounds
                    let max_val = <$type>::MAX;
                    assert_eq!(max_val.saturating_add(1), max_val);

                    let min_val = <$type>::MIN;
                    assert_eq!(min_val.saturating_sub(1), min_val);
                }

                #[test]
                fn [<test_ $type _next_power_of_two>]() {
                    let val: $type = $test_val;
                    let next_pow = val.next_power_of_two();
                    assert!(next_pow >= val);
                    assert!(next_pow.is_power_of_two());
                }

                #[test]
                fn [<test_ $type _next_power_of_two_exponent>]() {
                    let val: $type = $test_val;
                    let exp = val.next_power_of_two_exponent();
                    let (pow, exp2) = val.next_power_of_two_with_exponent();
                    assert_eq!(exp, exp2);
                    assert_eq!(pow, 2_i32.pow(exp as u32) as $type);
                }

                #[test]
                fn [<test_ $type _sign_mapping>]() {
                    let signed_val: $type = $test_val;
                    let unsigned_val = signed_val.map_sign_to_non_sign();
                    let back_to_signed = unsigned_val.map_non_sign_to_sign();
                    assert_eq!(signed_val, back_to_signed);
                }

                #[test]
                fn [<test_ $type _sqrt>]() {
                    let val: $type = $test_val;
                    let _ = val.sqrt();
                }
            }
        };

        // For unsigned integer types
        (unsigned_int: $type:ty, $test_val:expr) => {
            pastey::paste! {
                #[test]
                fn [<test_ $type _saturating_ops>]() {
                    let a: $type = $test_val;
                    let b: $type = 2;

                    let _ = a.saturating_add(b);
                    let _ = a.saturating_sub(b);
                    let _ = a.saturating_mul(b);

                    // Test saturation at bounds
                    let max_val = <$type>::MAX;
                    assert_eq!(max_val.saturating_add(1), max_val);

                    let min_val = <$type>::MIN;
                    assert_eq!(min_val.saturating_sub(1), min_val);
                }

                #[test]
                fn [<test_ $type _next_power_of_two>]() {
                    let val: $type = $test_val;
                    let next_pow = val.next_power_of_two();
                    assert!(next_pow >= val);
                    assert!(next_pow.is_power_of_two());
                }

                #[test]
                fn [<test_ $type _next_power_of_two_exponent>]() {
                    let val: $type = $test_val;
                    let exp = val.next_power_of_two_exponent();
                    let (pow, exp2) = val.next_power_of_two_with_exponent();
                    assert_eq!(exp, exp2);
                    assert_eq!(pow, 2_u32.pow(exp as u32) as $type);
                }

                #[test]
                fn [<test_ $type _sign_mapping>]() {
                    let unsigned_val: $type = $test_val;
                    let signed_val = unsigned_val.map_non_sign_to_sign();
                    let back_to_unsigned = signed_val.map_sign_to_non_sign();
                    assert_eq!(unsigned_val, back_to_unsigned);
                }

                #[test]
                fn [<test_ $type _sqrt>]() {
                    let val: $type = $test_val;
                    let _ = val.sqrt();
                }

                #[test]
                fn [<test_ $type _add_sign_logic>]() {
                    let unsigned_val: $type = $test_val;

                    // Test adding positive signed value
                    let result = unsigned_val.add_sign(5_i32);
                    assert_eq!(result, unsigned_val.wrapping_add($test_val + 5));

                    // Test adding negative signed value
                    let result = unsigned_val.saturated_add_sign(-2_i32);
                    assert!(result <= unsigned_val);
                }
            }
        };

        // For floating point types
        (float: $type:ty, $test_val:expr) => {
            pastey::paste! {
                #[test]
                fn [<test_ $type _sign>]() {
                    let pos: $type = $test_val;
                    let neg: $type = -$test_val;
                    let zero: $type = 0.0;

                    assert_eq!(pos.sign(), 1.0);
                    assert_eq!(neg.sign(), -1.0);
                    assert_eq!(zero.sign(), 0.0);
                }

                #[test]
                fn [<test_ $type _abs>]() {
                    let val: $type = -$test_val;
                    assert_eq!(val.abs(), $test_val);
                }

                #[test]
                fn [<test_ $type _round>]() {
                    let val: $type = 3.7;
                    assert_eq!(val.round(), 4.0);

                    let val2: $type = 3.3;
                    assert_eq!(val2.round(), 3.0);
                }

                #[test]
                fn [<test_ $type _sqrt>]() {
                    let val: $type = 16.0;
                    assert_eq!(val.sqrt(), 4.0);
                }

                #[test]
                fn [<test_ $type _floor_ceil>]() {
                    let val: $type = 3.7;
                    assert_eq!(val.floor(), 3.0);
                    assert_eq!(val.ceil(), 4.0);
                }

                #[test]
                fn [<test_ $type _next_power_of_two>]() {
                    let val: $type = 7.0;
                    let next_pow = val.next_power_of_two();
                    assert_eq!(next_pow, 8.0);

                    let val2: $type = 0.5;
                    assert_eq!(val2.next_power_of_two(), 1.0);
                }

                #[test]
                fn [<test_ $type _next_power_of_two_with_exponent>]() {
                    let val: $type = 7.0;
                    let (pow, exp) = val.next_power_of_two_with_exponent();
                    assert_eq!(pow, 8.0);
                    assert_eq!(exp, 3.0);
                }

                #[test]
                fn [<test_ $type _log_functions>]() {
                    let val: $type = 8.0;
                    assert_eq!(val.log2(), 3.0);

                    let val2: $type = 100.0;
                    assert_eq!(val2.log10(), 2.0);

                    let val3: $type = std::$type::consts::E;
                    let log_result = val3.log(100.0);
                    assert!((log_result - 1.0).abs() < 0.0001);
                }

                #[test]
                fn [<test_ $type _exp2>]() {
                    let val: $type = 3.0;
                    assert_eq!(val.exp2(), 8.0);
                }

                #[test]
                fn [<test_ $type _fract_trunc>]() {
                    let val: $type = 3.7;
                    assert_eq!(val.tunc(), 3.0);
                    assert!((val.fract() - 0.7).abs() < 0.0001);
                }
            }
        };
    }

    // Generate tests for all signed integer types
    test_all_traits!(signed_int: i8, 5);
    test_all_traits!(signed_int: i16, 100);
    test_all_traits!(signed_int: i32, 1000);
    test_all_traits!(signed_int: i64, 10000);
    test_all_traits!(signed_int: i128, 100000);
    test_all_traits!(signed_int: isize, 5000);

    // Generate tests for all unsigned integer types
    test_all_traits!(unsigned_int: u8, 10);
    test_all_traits!(unsigned_int: u16, 200);
    test_all_traits!(unsigned_int: u32, 2000);
    test_all_traits!(unsigned_int: u64, 20000);
    test_all_traits!(unsigned_int: u128, 200000);
    test_all_traits!(unsigned_int: usize, 10000);

    // Generate tests for all floating point types
    test_all_traits!(float: f16, 5.5);
    test_all_traits!(float: f32, 10.5);
    test_all_traits!(float: f64, 20.5);
    //test_all_traits!(float: f128, 30.5);

    // Additional edge case tests
    #[test]
    fn test_saturation_edge_cases() {
        // Test i8 min saturation
        assert_eq!(i8::MIN.saturating_abs(), i8::MAX);
        assert_eq!(i8::MIN.saturating_neg(), i8::MAX);

        // Test overflow saturation
        assert_eq!(u8::MAX.saturating_add(1), u8::MAX);
        assert_eq!(u8::MIN.saturating_sub(1), u8::MIN);
    }

    #[test]
    fn test_power_of_two_edge_cases() {
        assert_eq!(1_u32.next_power_of_two(), 1);
        assert_eq!(0_u32.next_power_of_two(), 1);

        assert_eq!(1_f32.next_power_of_two(), 1.0);
        assert_eq!(0.5_f32.next_power_of_two(), 1.0);
    }

    #[test]
    fn test_sign_mapping_roundtrip() {
        // Test various values round-trip correctly
        for val in [i8::MIN, -100, -1, 0, 1, 100, i8::MAX] {
            let unsigned = val.map_sign_to_non_sign();
            let back = unsigned.map_non_sign_to_sign();
            assert_eq!(val, back, "Failed round-trip for {}", val);
        }
    }
}
