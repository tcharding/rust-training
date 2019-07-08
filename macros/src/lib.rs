/// Example macro creation

/// A macro definition that mirrors the stdlib vec! macro.
#[macro_export]
macro_rules! myvec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn myvec_works() {
        let v = myvec!(1, 4, 7);
        let w = vec![1, 4, 7];

        for (i, _) in v.iter().enumerate() {
            assert_eq!(v[i], w[i]);
        }
    }

    #[test]
    fn myvec_is_iter_and_enumerate() {
        let v = myvec!(1, 4, 7);
        let w = vec![1, 4, 7];

        for (i, _) in w.iter().enumerate() {
            assert_eq!(v[i], w[i]);
        }
    }
}
