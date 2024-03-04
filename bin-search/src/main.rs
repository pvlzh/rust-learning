use std::cmp::Ordering;

fn main() {
    let arr: [i32; 10] = [-5, -2, 3, 5, 9, 16, 24, 51, 66, 92];
    let searched_value = 5;
    let result = bin_search(&arr, searched_value);   

    match result {
        Some((value, inx)) => println!("Searched value: {value} Index: {inx}"),
        None => println!("Value {searched_value} is not found.")
    }
}


/// Бинарный поиск по отсортированной коллекции чисел.
fn bin_search(arr: &[i32], searched_value: i32) -> Option<(i32, usize)>{
    let mut low_bound: usize = 0;
    let mut up_bound: usize = arr.len() - 1;

    let mut inx: usize = 0;

    while low_bound <= up_bound {
        inx += 1;

        let mid_inx = (up_bound + low_bound) / 2;
        let mid_value = arr[mid_inx];

        match mid_value.cmp(&searched_value){
            Ordering::Equal => return Some((mid_value, mid_inx)),
            Ordering::Greater => {
                up_bound = match mid_inx.checked_sub(1) {
                    Some(result) => result,
                    None => return None,
                }
            },
            Ordering::Less => low_bound = mid_inx + 1,
        }
        println!("Step {inx}");
    }
    None
}


#[cfg(test)]
mod tests{
    use super::*;

    const ARR: [i32; 10] = [-30, -10, -1, 2, 16, 38, 71, 129, 268, 514];

    #[test]
    fn element_found_1() {
        assert_eq!((-10, 1), bin_search(&ARR, -10).unwrap())
    }
    #[test]
    fn element_found_2() {
        assert_eq!((2, 3), bin_search(&ARR, 2).unwrap())
    }

    #[test]
    fn element_found_3() {
        assert_eq!((268, 8), bin_search(&ARR, 268).unwrap())
    }

    #[test]
    fn element_not_found() {
        assert!(bin_search(&ARR, 3).is_none())
    }

    #[test]
    fn smalles_element_not_found() {
        assert!(bin_search(&ARR, -10000).is_none())
    }
}