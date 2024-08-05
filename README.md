# Levenshtein Distance

A rust implementation of the levenshtein distance algorithm.

```rs
use levenshtein_distance::levenshtein_distance;
assert_eq!(levenshtein_distance("playwright", "playright"), 1);
```
