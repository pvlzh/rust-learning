fn main() {
    let mut arr = [5, 41, 2, 67, 9, -42, -5, 4, 15, 92];
    println!("{:?}", &arr);

    bubble_sort(&mut arr);
    println!("{:?}", &arr);
}

fn bubble_sort(arr: &mut [i32]) {
    let length = arr.len();

    for _ in 0..length {

        for inx in 0..length - 1{

            if arr[inx] < arr[inx + 1] { 
                continue; 
            }

            let temp = arr[inx];
            arr[inx] = arr[inx + 1];
            arr[inx + 1] = temp;
        }
    }
}
