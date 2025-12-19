/// Takes in a input of ranges thats are comma seperated.
/// Each range will be parsed and searched for invalid IDs
/// The sum of all invalid IDs will be returned
pub fn invalid_id_sum(input: String) -> i64 {
    // parse the string into comma seperated strings
    let mut sum = 0;
    let ranges: Vec<&str> = input.rsplit(',').collect();
    for range in ranges.iter() {
        let invalid_id_vec = invalid_ids(range);
        sum += invalid_id_vec.iter().sum::<i64>();
    }
    sum
}

/// Takes a string slice of the range and returns only the integers that are invalid IDs
/// Example: Passing "11-22" will return [11,22]
fn invalid_ids(range: &str) -> Vec<i64> {
    let spread = spread_range(range);
    spread
        .split(',')
        .filter(|s| {
            let s = s.trim();
            if s.len() % 2 != 0 {
                return false;
            }
            let mid = s.len() / 2;
            &s[..mid] == &s[mid..]
        })
        .filter_map(|s| s.parse::<i64>().ok())
        .collect()
}

/// Takes a range of integers in the form of strings with the range
/// seperated with with a '-' and returns a vector of integers in the range (inclusive)
/// Example: Passing "10-15", will return [10,11,12,13,14,15]
fn range_to_ints(string_range: &str) -> Vec<i64> {
    let (min_str, max_str) = string_range.split_once('-').unwrap();
    let min = min_str.parse::<i64>().unwrap();
    let max = max_str.parse::<i64>().unwrap();
    (min..=max).collect()
}

/// Takes the range and spreads it out to a comma seperated full range
/// For example if the range is 11-22, the return string slice
/// is 11,12,13,14,15,16,17,18,19,20,21,22
fn spread_range(range: &str) -> String {
    let spread_int = range_to_ints(range);
    spread_int
        .into_iter()
        .map(|num| num.to_string())
        .collect::<Vec<String>>()
        .join(",")
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
    fn test_invalid_sum_sample() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124".to_string();
        assert_eq!(invalid_id_sum(input), 1227775554);
    }

    #[test]
    fn test_invalid_ids() {
        let invalid_id_check = vec![11, 22];
        let range = "11-22";
        assert_eq!(
            invalid_ids(range),
            invalid_id_check,
            "Expected invalid IDs to be 11,22."
        );
    }

    #[test]
    fn test_invalid_ids_non_adjecent() {
        let invalid_id_values = vec![38593859];
        let range = "38593856-38593862";
        assert_eq!(
            invalid_ids(range),
            invalid_id_values,
            "Expected invalid IDs is 38593859"
        );
    }

    #[test]
    fn test_invalid_ids_repeated_sequence() {
        let invalid_id_values = vec![1188511885];
        let range = "1188511880-1188511890";
        assert_eq!(
            invalid_ids(range),
            invalid_id_values,
            "Expected invalid IDs is 1188511885"
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
