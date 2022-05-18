#[warn(non_snake_case)]
// This code does the basic Bubble Sorting of elements in an array.
// Best Case TC : O(n)
// Worst Case TC : O(n^2)

use std::time::Instant;

pub fn bubble_sort(arr: &mut [i32]) {
        let now = Instant::now();
        for i in 0..arr.len() {
            for j in 0..arr.len() - 1 - i {
                if arr[j] > arr[j+1] {
                    arr.swap(j, j+1);
                }
            }
        }
        let elapsed = now.elapsed();
        println!("Optimized Bubble Sort Time taken {:?}", elapsed);
    }


pub fn bubble_sort_optimized(arr: &mut [i32]) {
        let now = Instant::now();
        let mut is_sorted;
        for i in 0..arr.len() {
            is_sorted = false;
            for j in 0..arr.len() - 1 - i {
                if arr[j] > arr[j+1] {
                    arr.swap(j,j+1);
                    is_sorted = true;
                }
            }
                
            if is_sorted == false {
                break;
            }
            
        }
        let elapsed = now.elapsed();
        println!("Unoptimized Bubble Sort Time taken {:?}", elapsed);
    }


fn main() {
    let mut number_un = [8,4,5,6,1,2];
    let mut number = [8,4,5,6,1,2];
    bubble_sort(&mut number_un);
    println!("UnOptimized Function {:?}", number_un);
    bubble_sort_optimized(&mut number);
    println!("Optimized Function {:?}", number);

    println!("NOTE: Time taken may vary from system to system");
}