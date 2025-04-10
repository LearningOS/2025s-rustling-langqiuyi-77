// iterators3.rs
//
// This is a bigger exercise than most of the others! You can do it! Here is
// your mission, should you choose to accept it:
// 1. Complete the divide function to get the first four tests to pass.
// 2. Get the remaining tests to pass by completing the result_with_list and
//    list_of_results functions.
//
// Execute `rustlings hint iterators3` or use the `hint` watch subcommand for a
// hint.


// 这个就是错误类型
#[derive(Debug, PartialEq, Eq)]
pub enum DivisionError {
    NotDivisible(NotDivisibleError),    // NotDivisible 是元组结构体
    DivideByZero,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotDivisibleError {  // NotDivisibleError 是结构体
    dividend: i32,
    divisor: i32,
}

// Calculate `a` divided by `b` if `a` is evenly divisible by `b`.
// Otherwise, return a suitable error.
// 丢异常就是 Err()
pub fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    // todo!();
    if b == 0 {
        return Err(DivisionError::DivideByZero);
    }

    if a % b != 0 {
        // NotDivisible 是元组结构体 要使用 （）创
        // NotDivisibleError 是结构体 要用 {}
        return Err(DivisionError::NotDivisible(NotDivisibleError{
            dividend: a,
            divisor: b,
        }));
    }

    Ok(a / b)
}

// Complete the function and return a value of the correct type so the test
// passes.
// Desired output: Ok([1, 11, 1426, 3])
fn result_with_list() -> Result<Vec<i32>, DivisionError> {
    // let numbers = vec![27, 297, 38502, 81];
    // let division_results: Result<Vec<i32>, DivisionError> = numbers.into_iter().map(|n| divide(n, 27)?).collect();
    // 上面的是我的写法，可以 .collect() 确实可以自动推断类型，但你用了 ?，整个 map(...) 是可能失败的操作
    // （因为 divide(...) -> Result<i32, DivisionError>），
    // 所以 .collect() 返回的也不是 Vec<i32>，而是：Result<Vec<i32>, DivisionError>
    // 所以你的 division_results 其实已经是一个 Result 了
    // 所以要指定 division_results: Result<Vec<i32>, DivisionError>
    vec![27, 297, 38502, 81]
        .into_iter()
        .map(|n| divide(n, 27))
        .collect()
}

// Complete the function and return a value of the correct type so the test
// passes.
// Desired output: [Ok(1), Ok(11), Ok(1426), Ok(3)]
fn list_of_results() -> Vec<Result<i32, DivisionError>> {
    let numbers = vec![27, 297, 38502, 81];
    // 在 Rust 里，.collect() 有“自动魔法”，它会根据目标类型来决定怎么收集。
    // 如果你 collect 到 Vec<Result<T, E>>，那它会把每个 Result 放进去，不管是 Ok 还是 Err。
    // 所以你这个代码是不会短路的，它会继续 collect，即使中间有 Err。
    let division_results: Vec<Result<i32, DivisionError>> = numbers.into_iter().map(|n| divide(n, 27)).collect();
    division_results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(
            divide(81, 6),
            Err(DivisionError::NotDivisible(NotDivisibleError {
                dividend: 81,
                divisor: 6
            }))
        );
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        assert_eq!(format!("{:?}", result_with_list()), "Ok([1, 11, 1426, 3])");
    }

    #[test]
    fn test_list_of_results() {
        assert_eq!(
            format!("{:?}", list_of_results()),
            "[Ok(1), Ok(11), Ok(1426), Ok(3)]"
        );
    }
}
