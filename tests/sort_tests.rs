use ads::sort::{bubble, insertion,  selection, quicksort, quicksort_hoare};
use std::time::Instant;

#[test]
fn benchmarks() {
    let v = random_vector(1000);

    let mut vec = v.clone();
    let start = Instant::now();
    insertion(&mut vec);
    println!("insertion {:?}", start.elapsed());
    assert_sorted(&vec);

    let mut vec = v.clone();
    let start = Instant::now();
    selection(&mut vec);
    println!("selection {:?}", start.elapsed());
    assert_sorted(&vec);

    let mut vec = v.clone();
    let start = Instant::now();
    bubble(&mut vec);
    println!("bubble {:?}", start.elapsed());
    assert_sorted(&vec);

    let mut vec = v.clone();
    let start = Instant::now();
    quicksort_hoare(&mut vec);
    println!("quicksort hoare {:?}", start.elapsed());
    assert_sorted(&vec);

    let mut vec = v.clone();
    let start = Instant::now();
    quicksort(&mut vec);
    println!("quicksort {:?}", start.elapsed());
    assert_sorted(&vec);
}

fn assert_sorted(slice: &[i32]) {
    for i in 1..slice.len() {
        assert!(slice[i - 1] <= slice[i])
    }
}

fn random_vector(size: u32) -> Vec<i32> {
    let mut slice: Vec<i32> = Vec::new();
    for _ in 0..size {
        slice.push(rand::random::<i32>());
    }
    slice
}
