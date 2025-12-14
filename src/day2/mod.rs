use regex::Regex;

fn invalid_id_sum(input: String) -> i32 {
    // parse the string into comma seperated strings
    // let ranges: Vec<&str> = input.rsplit(',').map(|substring| range_to_ints(substring));
    0
}

/// Takes a vector of integers and returns only the integers that are invalid IDs
/// Example: Passing [11,12,13,14,15,16,17,18,19,20,21,22] will return [11,22]
fn invalid_ids(range: Vec<i32>) -> Vec<i32> {
    // TODO: Use regex (?<!\d)((.)\2+)(?!\d)
    let regex = Regex::new(r"(?<!\d)((.)\2+)(?!\d)");
    vec![0]
}

/// Takes a range of integers in the form of strings with the range
/// seperated with with a '-' and returns a vector of integers in the range (inclusive)
/// Example: Passing "10-15", will return [10,11,12,13,14,15]
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
    fn test_invalid_ids() {
        let invalid_id_check = vec![11, 22];
        let range = (11..=22).collect();
        assert_eq!(
            invalid_ids(range),
            invalid_id_check,
            "Expected invalid IDs to be 11,22."
        );
    }

    #[test]
    fn test_range_to_ints() {
        let vec_range = vec![10, 11, 12, 13, 14, 15];
        let returned_range = range_to_ints("10-15");
        assert_eq!(
            returned_range, vec_range,
            "Expected range to be [10,11,12,13,14,15]"
        );
    }
}
