// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

// pub fn naive_factorial(num: u64) -> u64 {
//     let mut fact: u64 = 1;
//     let mut n = num.clone();
//     while n > 0 {
//         fact = fact * n;
//         println!("{:?}", n);
//         n = n - 1;
//     }
//     return fact;
// }

// pub fn recursion_factorial(num: u64) -> u64 {
//     if num < 1 {
//         return 1;
//     } else {
//         return num * factorial(num - 1);
//     }
// }

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - early returns (using the `return` keyword explicitly)
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    (1u64..=num).rev().fold(1, |acc, x| acc * x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
