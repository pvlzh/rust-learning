use std::{default::Default, iter::Sum, marker::Copy, ops::Add};

/// Сложение чисел перечисления.
pub fn sum1<T>(numbers: &[T]) -> T
    where T : Add<Output = T> + Copy + Default
{
    let mut result = T::default();
    for number in numbers {
        result = result + (*number);
    }
    result
}

/// Сложение чисел перечисления.
pub fn sum2<'a, T>(numbers: &'a [T]) -> T 
    where T : Sum<&'a T>
{
    numbers.iter().sum()
}
