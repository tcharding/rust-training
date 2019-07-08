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

/// Searches data for target.  Data must be sorted in ascending order.
pub fn binary_search<T: Ord>(data: &[T], target: &T) -> Option<usize> {
    let mut low = 0;
    let mut high = data.len();
    let mut mid = high / 2;

    while low < high {
        match target.cmp(&data[mid]) {
            Less => high = mid,
            Greater => low = mid,
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
    fn linear_search_works() {
        let data = vec![2, 4, 6, 0, 1, 3, 9, 5];
        let target = 1;
        let want_index = 4;

        let got_index = linear_search(&data, &target).expect("false negative");
        assert_eq!(want_index, got_index);
    }

    #[test]
    fn binary_search_works() {
        let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let target = 7;
        let want_index = 7;

        let got_index = linear_search(&data, &target).expect("false negative");
        assert_eq!(want_index, got_index);
    }

    #[test]
    #[should_panic]
    fn binary_search_fails_correctly() {
        let data = vec![0, 1, 2];
        let target = 999;

        linear_search(&data, &target).unwrap();
    }
}
