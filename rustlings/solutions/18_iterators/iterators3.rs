#[derive(Debug, PartialEq, Eq)]
enum DivisionError {
    // 例: 42 / 0
    DivideByZero,
    // `i64`の場合のみ: `i64::MIN / -1`を実行すると`i64::MAX + 1`になってしまうためエラーとなる。
    IntegerOverflow,
    // 例: 5 / 2 = 2.5
    NotDivisible,
}

fn divide(a: i64, b: i64) -> Result<i64, DivisionError> {
    if b == 0 {
        return Err(DivisionError::DivideByZero);
    }

    if a == i64::MIN && b == -1 {
        return Err(DivisionError::IntegerOverflow);
    }

    if a % b != 0 {
        return Err(DivisionError::NotDivisible);
    }

    Ok(a / b)
}

fn result_with_list() -> Result<Vec<i64>, DivisionError> {
    //                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    let numbers = [27, 297, 38502, 81];
    let division_results = numbers.into_iter().map(|n| divide(n, 27));
    // 推定された返り値の集合体。割り算の結果を最初のエラーとして返します。
    division_results.collect()
}

fn list_of_results() -> Vec<Result<i64, DivisionError>> {
    //               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    let numbers = [27, 297, 38502, 81];
    let division_results = numbers.into_iter().map(|n| divide(n, 27));
    // 推定された返り値の集合体。
    division_results.collect()
}

fn main() {
    // この行で関数のテストができます。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_integer_overflow() {
        assert_eq!(divide(i64::MIN, -1), Err(DivisionError::IntegerOverflow));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(divide(81, 6), Err(DivisionError::NotDivisible));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        assert_eq!(result_with_list().unwrap(), [1, 11, 1426, 3]);
    }

    #[test]
    fn test_list_of_results() {
        assert_eq!(list_of_results(), [Ok(1), Ok(11), Ok(1426), Ok(3)]);
    }
}
