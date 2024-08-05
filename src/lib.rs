use ::std::cmp::min;

/// # Levenshtein Distance
///
/// # Example
/// ```
/// use levenshtein_distance::levenshtein_distance;
/// assert_eq!(levenshtein_distance("playwright", "playright"), 1);
/// ```
pub fn levenshtein_distance<'a>(a: &'a str, b: &'a str) -> usize {
    if a == b {
        return 0;
    }
    if a.is_empty() {
        return b.len();
    }
    if b.is_empty() {
        return a.len();
    }

    let a_index = a.len() + 1;
    let b_index = b.len() + 1;

    let mut d: Vec<Vec<usize>> = (0..a_index)
        .map(|i| {
            (0..b_index)
                .map(|x| {
                    if x == 0 {
                        i
                    } else if i == 0 {
                        x
                    } else {
                        0
                    }
                })
                .collect()
        })
        .collect();

    for i in 1..a_index {
        for x in 1..b_index {
            if a.as_bytes()[i - 1] == b.as_bytes()[x - 1] {
                d[i][x] = d[i - 1][x - 1];
            } else {
                d[i][x] = 1 + min(d[i - 1][x], min(d[i][x - 1], d[i - 1][x - 1]));
            }
        }
    }

    d[a.len()][b.len()]
}

#[cfg(test)]
mod tests {
    use super::levenshtein_distance;

    #[test]
    fn identical_strings() {
        let result = levenshtein_distance("identical", "identical");
        assert_eq!(result, 0); // No changes
    }

    #[test]
    fn single_character_difference() {
        let result = levenshtein_distance("kitten", "sitten");
        assert_eq!(result, 1); // One substitution
    }

    #[test]
    fn single_insertion() {
        let result = levenshtein_distance("kitten", "kittens");
        assert_eq!(result, 1); // One insertion
    }

    #[test]
    fn single_deletion() {
        let result = levenshtein_distance("kitten", "kitte");
        assert_eq!(result, 1); // One insertion
    }

    #[test]
    fn insertion_deletion() {
        let result = levenshtein_distance("flaw", "lawn");
        assert_eq!(result, 2); // One insertion and one deletion
    }

    #[test]
    fn multiple_edits() {
        let result = levenshtein_distance("sitting", "kitten");
        assert_eq!(result, 3); // Three edits (substitution, deletion, insertion)
    }

    #[test]
    fn empty_string() {
        let result = levenshtein_distance("", "nonempty");
        assert_eq!(result, 8); // All characters need to be inserted
    }

    #[test]
    fn both_empty_strings() {
        let result = levenshtein_distance("", "");
        assert_eq!(result, 0); // No changes
    }

    #[test]
    fn right_empty_string() {
        let result = levenshtein_distance("nonempty", "");
        assert_eq!(result, 8); // All characters need to be deleted
    }

    #[test]
    fn long_strings_with_common_prefix() {
        let result = levenshtein_distance("abcdefghij", "abcxyzghij");
        assert_eq!(result, 3); // Three substitutions (d->x, e->y, f->z)
    }

    #[test]
    fn long_strings_with_large_difference() {
        let result = levenshtein_distance("abcdefghij", "zyxwvutsrq");
        assert_eq!(result, 10); // All characters are different
    }

    #[test]
    fn mixed_edits() {
        let result = levenshtein_distance("algorithm", "altrorithm");
        assert_eq!(result, 2); // One substitution and one insertion
    }
}
