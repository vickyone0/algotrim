fn main() {

    let mut arr = [5, 2, 9, 1, 5, 6];
    println!("Before sorting: {:?}", arr);
    bubble_sort(&mut arr);
    println!("After sorting: {:?}", arr);

}

fn bubble_sort(arr: &mut [i32]) {

    for i in 0..(arr.len()-1) {
        let mut swapped = false;

        for j in 0..(arr.len()-1-i) {
            if arr[j] > arr[j+1] {
                arr.swap(j, j+1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }

    }
}