use std::cmp::Ordering;

#[cfg(test)]
mod tests;

pub fn binary_search<T: Ord>(arr: &[T], target: T) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + ((right - left) / 2);
        match target.cmp(&arr[mid]) {
            Ordering::Equal => return Some(mid),
            Ordering::Greater => left = mid + 1,
            Ordering::Less => right = mid,
        }
    }
    None
}

pub fn lower_bound<T: Ord>(arr: &[T], target: T) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + ((right - left) / 2);
        match target.cmp(&arr[mid]) {
            Ordering::Less | Ordering::Equal => right = mid,
            Ordering::Greater => left = mid + 1,
        }
    }
    
    if left < arr.len() {
        Some(left)
    } else {
        None
    }
}

pub fn upper_bound<T: Ord>(arr: &[T], target: T) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + ((right - left) / 2);
        match target.cmp(&arr[mid]) {
            Ordering::Less => right = mid,
            Ordering::Greater | Ordering::Equal => left = mid + 1,
        }
    }
    
    if left < arr.len() {
        Some(left)
    } else {
        None
    }
}

