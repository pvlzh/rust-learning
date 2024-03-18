fn main() {
    let mut input_array = [5, 41, 2, 67, 9, -42, -5, 4, 15, 92];
    println!("{:?}", &input_array);

    bubble_sort(&mut input_array);
    println!("{:?}", &input_array);
}

fn bubble_sort(input_array: &mut [i32]) {
    let array_length = input_array.len();

    for _ in 0..array_length {

        for inx in 0..array_length - 1{

            if input_array[inx] < input_array[inx + 1] { 
                continue; 
            }

            let temp = input_array[inx];
            input_array[inx] = input_array[inx + 1];
            input_array[inx + 1] = temp;
        }
    }
}