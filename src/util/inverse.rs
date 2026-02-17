use crate::util::positive_mod;

/// Returns `(gcd, coefficient_a, coefficient_b)` such that: `a * coefficient_a + b * coefficient_b = gcd`
fn extended_gcd(a: isize, b: isize) -> (isize, isize, isize) {
    if b == 0 {
        // Base case: gcd(a, 0) = a = a*1 + 0*0
        (a, 1, 0)
    } else {
        let (gcd, prev_coefficient_a, prev_coefficient_b) = extended_gcd(b, positive_mod(a, b));
        let current_coefficient_a = prev_coefficient_b;
        let current_coefficient_b = prev_coefficient_a - (a / b) * prev_coefficient_b;

        (gcd, current_coefficient_a, current_coefficient_b)
    }
}

pub fn mod_inverse(number: isize, modulus: isize) -> Option<isize> {
    let (gcd, coefficient_number, _) = extended_gcd(number, modulus);

    // Existence check
    if gcd != 1 {
        return None;
    }

    let inverse = positive_mod(coefficient_number, modulus);

    Some(inverse)
}
#[cfg(test)]
mod test {
    use log::info;
    use super::*;

    #[test]
    fn extended_gcd_two() {
        assert_eq!(extended_gcd(2, 5).0, 1);
        dbg!( mod_inverse(7, 26).unwrap());
    }

    #[test]
    fn extended_gcd_minus_three() {
        assert_eq!(extended_gcd(-3, 5).0, 1);
    }

    #[test]
    fn extended_gcd_tests() {
        assert_eq!(extended_gcd(-2, 5).0, 1);
        assert_eq!(extended_gcd(-3, 5).0, 1);
    }

    fn verify_inverse(initial: isize, inverse: Option<isize>, modulus: isize) -> bool {
        match inverse {
            Some(inverse) => {
                println!("{initial}**-1 mod {modulus} eq {inverse}");
                let v = (initial * inverse) % modulus;
                v == 1 || v == -modulus + 1
            }
            None => {
                println!("{initial} does not have an inverse mod {modulus}");
                dbg!(extended_gcd(initial, modulus).0) != 1
            },
        }

    }

    macro_rules! test_one {
        ($value:expr, $modulus:expr) => {
            let inverse = mod_inverse($value, $modulus);
            assert!(verify_inverse($value, inverse, $modulus));
        };
    }

    #[test]
    fn test_inverse() {
        test_one!(2, 5);
        test_one!(3, 5);
        test_one!(6, 5);
        test_one!(1, 5);
        test_one!(0, 5);
        test_one!(-5, 5);
        test_one!(-2, 5);
        test_one!(-3, 5);
    }

    #[test]
    fn test_batch_non_prime() {
        let modulus: isize = 24;

        for x in -100isize..100 {
            test_one!(x, modulus);
        }
    }

    #[test]
    fn test_batch_prime() {
        let prime: isize = 751;

        for x in -5000isize..5000 {
            test_one!(x, prime);
        }
    }
}