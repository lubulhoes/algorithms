pub fn binary_search<T: Ord>(arr: &[T], target: T) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + ((right - left) / 2);
        match target.cmp(&arr[mid]) {
            std::cmp::Ordering::Less => right = left,
            std::cmp::Ordering::Greater => left += 1,
            std::cmp::Ordering::Equal => return Some(left),
            
        }
    }
    None
}