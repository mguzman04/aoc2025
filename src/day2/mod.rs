use regex::Regex;

/// Takes in a input of ranges thats are comma seperated.
/// Each range will be parsed and searched for invalid IDs
/// The sum of all invalid IDs will be returned
fn invalid_id_sum(input: String) -> i32 {
    // parse the string into comma seperated strings
    let mut sum = 0;
    let ranges: Vec<&str> = input.rsplit(',').collect();
    for range in ranges.iter() {
        let range_vec = range_to_ints(range);
        let invalid_ids = invalid_ids(range_vec);
        sum += invalid_ids.iter().sum::<i32>();
    }
    sum
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

/// Takes the range and spreads it out to a comma seperated full range
/// For example if the range is 11-22, the return string slice
/// is 11,12,13,14,15,16,17,18,19,20,21,22
fn spread_range(range: &str) -> &str {
    "uimplemented!"
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_invalid_sum() {
        let input = "11-22".to_string();
        assert_eq!(invalid_id_sum(input), 33);
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

    #[test]
    fn test_spread_range() {
        let range = "11-22";
        let spread_out = "11,12,13,14,15,16,17,18,19,20,21,22";
        assert_eq!(spread_range(range), spread_out);
    }
}
