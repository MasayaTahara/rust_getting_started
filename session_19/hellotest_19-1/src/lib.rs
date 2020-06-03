fn add(x: i32, y: i32) -> i32 {
    let ans = x + y;
    if ans < 0 {
        0
    } else {
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        assert_eq!(3, add(1, 2));
        assert_eq!(73, add(26, 47));
    }
    #[test]
    fn test_add_zero() {
        assert_eq!(0, add(0, 0));
    }
    #[test]
    fn test_add_under_zero() {
        assert_eq!(0, add(-1, -1));
    }
}
