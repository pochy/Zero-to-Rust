fn binary_search(values: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = values.len();

    while left < right {
        let middle = left + (right - left) / 2;

        if values[middle] == target {
            return Some(middle);
        }

        if values[middle] < target {
            left = middle + 1;
        } else {
            right = middle;
        }
    }

    None
}

fn main() {
    let values = [1, 3, 5, 7, 9, 11, 13];

    for target in [1, 7, 13, 2] {
        println!("{target}: {:?}", binary_search(&values, target));
    }
}

#[cfg(test)]
mod tests {
    use super::binary_search;

    #[test]
    fn finds_positions() {
        let values = [1, 3, 5, 7, 9];
        assert_eq!(binary_search(&values, 1), Some(0));
        assert_eq!(binary_search(&values, 7), Some(3));
        assert_eq!(binary_search(&values, 9), Some(4));
    }

    #[test]
    fn handles_missing_and_empty_inputs() {
        let values = [1, 3, 5, 7, 9];
        assert_eq!(binary_search(&values, 2), None);
        assert_eq!(binary_search(&[], 2), None);
    }
}
