//! Provides functionality related to computing the Levenshtein edit distance
//! metric.

/// Compute the Levenshtein distance between `str1` and `str2`.
///
/// #Parameters
///
/// * `str1` - first string
/// * `str2` - second string
pub fn distance(str1: &[char], str2: &[char]) -> usize {
    let mut cost = 0;
    if str1.len() == 0 {
        return str2.len();
    }
    if str2.len() == 0 {
        return str1.len();
    }

    cost = if str1.last() == str2.last() {
        0
    } else {
        1
    };

    let options = vec![
        distance(&str1[..str1.len() - 1], str2) + 1,
        distance(str1,  &str2[..str2.len() - 1]) + 1,
        distance(&str1[..str1.len() - 1],  &str2[..str2.len() - 1]) + cost,
    ];

     *options.iter().min().unwrap()
}
