pub fn insertion(arr: &mut [i32]) {
    if arr.len() < 2 {
        return;
    }
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

pub fn selection(arr: &mut [i32]) {
    if arr.len() < 2 {
        return;
    }
    for i in 0..arr.len() {
        let mut min = i;
        for j in i + 1..arr.len() {
            if arr[j] < arr[min] {
                min = j;
            }
        }
        arr.swap(i, min);
    }
}

pub fn bubble(arr: &mut [i32]) {
    if arr.len() < 2 {
        return;
    }
    for i in 0..arr.len() {
        for j in 1..arr.len() - i {
            if arr[j] < arr[j - 1] {
                arr.swap(j - 1, j)
            }
        }
    }
}

pub fn quicksort(arr: &mut [i32]) {
    let len = arr.len();
    if len < 2 {
        return;
    }
    let p = arr[len - 1];
    let mut i = 0;
    let mut j = len - 1;
    loop {
        while arr[i] < p {
            i += 1;
        }
        while j > 0 && arr[j] > p {
            j -= 1;
        }
        if i >= j {
            break;
        }
        arr.swap(i, j);
    }
    quicksort(&mut arr[0..j]);
    quicksort(&mut arr[j + 1..len]);
}

#[cfg(test)]
mod sort_tests {
    use super::*;

    #[test]
    fn sort_test() {
        let sol = [0, 1, 2, 3, 4, 5, 6];

        let mut arr = [6, 5, 4, 3, 2, 1, 0];
        insertion(&mut arr);
        assert_eq!(arr, sol);

        let mut arr = [6, 5, 4, 3, 2, 1, 0];
        selection(&mut arr);
        assert_eq!(arr, sol);

        let mut arr = [6, 5, 4, 3, 2, 1, 0];
        bubble(&mut arr);
        assert_eq!(arr, sol);

        let mut arr = [6, 5, 4, 3, 2, 1, 0];
        quicksort(&mut arr);
        assert_eq!(arr, sol);

        let mut arr = [305086552, 2123194479, 738412784, -1237237207, -1056745339];
        quicksort(&mut arr);
        assert_eq!(
            arr,
            [-1237237207, -1056745339, 305086552, 738412784, 2123194479]
        );
    }
}
