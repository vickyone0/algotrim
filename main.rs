fn main() {

    let mut arr = [5, 2, 9, 1, 5, 6];
    println!("Before sorting: {:?}", arr);
    insertion_sort(&mut arr);
    println!("After sorting: {:?}", arr);

}

pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
    // Start from the second element (index 1)
    for i in 1..arr.len() {
        let mut j = i;
        
        // Move elements of arr[0..i-1] that are greater than arr[i]
        // to one position ahead of their current position
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}