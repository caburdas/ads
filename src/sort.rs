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

pub fn quicksort_hoare(arr: &mut [i32]) {
    quicksort_hoare2(arr, 0, arr.len()-1)
}

fn quicksort_hoare2(arr: &mut [i32], l:usize, h:usize) {
    if l < h {
        let idx = partition_hoare( arr, l, h);
        quicksort_hoare2(arr, l, idx);
        quicksort_hoare2(arr, idx + 1, h );
    }
}

fn partition_hoare(arr: &mut [i32], l:usize, h:usize) -> usize{

    let p = arr[l]; // CAUTION !!!  Avoid to use as a pivot the last element, it can cause infinite
    // loop, you can use inital position l, or the middle position (l+h)/2 to avoid gits.
    let mut i:i32 = l as i32 - 1;
    let mut j:i32 = h as i32 + 1;
    loop {
        i += 1;
        while arr[i as usize] < p {
            i += 1;
        }
        j -= 1;
        while j > 0 && arr[j as usize] > p {
            j -= 1;
        }
        if i >= j {
            return j as usize;
        }
        arr.swap(i as usize, j as usize);
    }
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
        quicksort_hoare(&mut arr);
        assert_eq!(arr, sol);

        let mut arr = [305086552, 2123194479, 738412784, -1237237207, -1056745339];
        quicksort_hoare(&mut arr);
        assert_eq!(
            arr,
            [-1237237207, -1056745339, 305086552, 738412784, 2123194479]
        );
    }
}
