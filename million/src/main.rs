/*
 Author: Henry Rausch
 Date: 15.04.2023
Description: This is a programm that implents binary search and linear search in rust
*/

// import libraries
use std::time::Instant;
use rand::Rng;

fn binary_search(arr: [i32; 100000], target: i32) -> bool {
    let mut low : usize = 0;
    let mut high: usize = arr.len() - 1;
    let mut mid : usize;

    while low <= high {
        mid = (low + high) / 2;
        if arr[mid] == target {
            return true;
        } else if arr[mid] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }
    return false;
}

fn linear_search(arr: [i32; 100000], target: i32) -> bool {
    for i in 0..arr.len() {
        if arr[i] == target {
            return true;
        }
    }
    return false;
}

fn random_number(max : i32) -> i32 {
    let mut rng = rand::thread_rng();
    let num : i32 = rng.gen_range(0..max);
    return num;
}

fn create_array(size: i32) -> [i32; 100000] {
    let mut arr: [i32; 100000] = [0; 100000];
    for i in 0..size {
    // Fill the array with numbers
        arr[i as usize] = i;
    }
    return arr;
}

fn main() {
    // set target
    let target: i32 = random_number(100000);
    println!("The Target is: {:?}", target);

    // create array
    let arr : [i32; 100000] = create_array(100000);

    // binary search
    let start = Instant::now();
    binary_search(arr, target);
    let duration = start.elapsed();
    println!("Binary search took: {:?}", duration);

    // linear search
    let start = Instant::now();
    linear_search(arr, target);
    let duration = start.elapsed();
    println!("Linear search took: {:?}", duration);
}
