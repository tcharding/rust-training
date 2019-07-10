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
    fn quick_sort_works() {
        let mut data = [7, 5, 3, 1, 9, 0, 4, 2, 6, 8];
        let want = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        quick_sort(&mut data, &|x, y| x < y);

        for (i, d) in data.iter().enumerate() {
            if d != &want[i] {
                panic!("Failed at index: {} with value: {}", i, d);
            }
        }
    }
}
