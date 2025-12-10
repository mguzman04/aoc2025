fn invalid_id_sum(input: String) -> i32 {
    // parse the string into comma seperated strings
    // let ranges: Vec<&str> = input.rsplit(',').map(|substring| range_to_ints(substring));
    0
}

fn invalid_ids(range: Vec<&str>) -> Vec<i32> {
    vec![0]
}

fn range_to_ints(string_range: &str) -> Vec<i32> {
    let (min_str, max_str) = string_range.split_once('-').unwrap();
    let min = min_str.parse::<i32>().unwrap();
    let max = max_str.parse::<i32>().unwrap();
    (min..=max).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_invalid_sum() {
        let input = "11-22".to_string();
        assert_eq!(invalid_id_sum(input), 2);
    }

    #[test]
    fn invalid_ids_test() {
        unimplemented!()
    }
}
