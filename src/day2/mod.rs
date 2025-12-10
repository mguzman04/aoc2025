fn invalid_id_sum(input: String) -> i32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_invalid_sum() {
        let input = "11-22".to_string();
        assert_eq!(invalid_id_sum(input), 2);
    }
}
