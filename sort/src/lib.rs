/// Based on http://rosettacode.org/wiki/Sorting_algorithms/Quicksort#Rust

pub fn quick_sort<T, F>(v: &mut [T], f: &F)
where
    F: Fn(&T, &T) -> bool,
{
    if v.len() > 1 {
        let pivot = partition(v, f);
        quick_sort(&mut v[0..pivot], f);
        quick_sort(&mut v[pivot + 1..], f);
    }
}

fn partition<T, F>(v: &mut [T], f: &F) -> usize
where
    F: Fn(&T, &T) -> bool,
{
    let len = v.len();
    let pivot = len / 2;
    let last = len - 1;

    v.swap(pivot, last);

    let mut store = 0;
    for i in 0..len - 1 {
        if f(&v[i], &v[last]) {
            v.swap(i, store);
            store += 1;
        }
    }

    v.swap(store, last);
    store
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_single_element_array() {
        let mut v = [6];
        quick_sort(&mut v, &|x, y| x < y);
    }

    #[test]
    fn sorts_five_element_array() {
        let mut v = [0, 3, 4, 2, 1];
        quick_sort(&mut v, &|x, y| x < y);
        for i in 0..v.len() {
            assert_eq!(v[i], i);
        }
    }
}
