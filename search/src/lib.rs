use std::cmp::Ordering::*;

/// A couple of simple search algorithms.

/// Searches data for target.  (This is intentionally trivial.)
pub fn linear_search<T: PartialEq>(data: &[T], target: &T) -> Option<usize> {
    for (i, d) in data.iter().enumerate() {
        if d == target {
            return Some(i);
        }
    }
    None
}

pub fn iter_search<T: PartialEq>(data: impl Iterator<Item = T>, target: T) -> Option<usize> {
    for (i, d) in data.enumerate() {
        if d == target {
            return Some(i);
        }
    }
    None
}

/// Searches data for target.  Data must be sorted in ascending order.
pub fn binary_search<T: Ord>(data: &[T], target: &T) -> Option<usize> {
    let mut low = 0;
    let mut high = data.len();
    let mut mid = high / 2;

    while low < high {
        match target.cmp(&data[mid]) {
            Less => high = mid - 1,
            Greater => low = mid + 1,
            Equal => return Some(mid),
        }
        mid = (high + low) / 2;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_search_returns_none_for_empty_list() {
        let v = vec![];

        let index = linear_search(&v, &1);
        assert!(index.is_none());
    }

    #[test]
    fn linear_search_returns_none_for_non_present_value() {
        let v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let value = 99;

        let found = linear_search(&v, &value);
        assert!(found.is_none());
    }

    #[test]
    fn linear_search_finds_present_value() {
        let v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let index = 3;
        let value = 3;

        let found = linear_search(&v, &value).expect("false negative");
        assert_eq!(found, index);
    }

    #[test]
    fn iter_search_returns_none_for_empty_list() {
        let v = vec![];

        let index = iter_search(v.iter(), &1);
        assert!(index.is_none());
    }

    #[test]
    fn iter_search_returns_none_for_non_present_value() {
        let v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let value = 99;

        let found = iter_search(v.iter(), &value);
        assert!(found.is_none());
    }

    #[test]
    fn iter_search_finds_present_value() {
        let v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let index = 3;
        let value = 3;

        let found = iter_search(v.iter(), &value).expect("false negative");
        assert_eq!(found, index);
    }

    #[test]
    fn binary_search_finds_preset_value() {
        let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let target = 7;
        let want_index = 7;

        let got_index = linear_search(&data, &target).expect("false negative");
        assert_eq!(want_index, got_index);
    }

    #[test]
    fn binary_search_returns_none_for_external_missing_value() {
        let data = vec![1, 2, 3];
        let target = 10;
        assert_eq!(binary_search(&data, &target), None);
    }

    #[test]
    fn binary_search_returns_none_for_internal_missing_value() {
        let data = vec![0, 1, 2, 4, 5];
        let target = 3;
        assert_eq!(binary_search(&data, &target), None);
    }
}
