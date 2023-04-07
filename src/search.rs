pub fn binary_search(arr: &[i32], e: i32) -> bool {
    let mut l: i32 = 0;
    let mut r: i32 = arr.len() as i32 - 1;

    while l <= r {
        let m = (l + r) / 2;
        if e == arr[m as usize] {
            return true;
        } else if arr[m as usize] > e {
            r = m - 1;
        } else if arr[m as usize] < e {
            l = m + 1;
        }
    }
    false
}

#[cfg(test)]
mod search_tests {
    use super::*;
    #[test]
    fn test_found() {
        let arr = [1, 2, 3, 4, 5, 6];
        assert_eq!(true, binary_search(&arr, 4));
    }

    #[test]
    fn test_not_found() {
        let arr = [1, 2, 3, 4, 5, 6];
        println!("{}", binary_search(&arr, 0));
        assert_eq!(false, binary_search(&arr, 0));
    }
}
