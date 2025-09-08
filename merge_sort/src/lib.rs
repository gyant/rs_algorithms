pub fn merge_sort<T: Ord + Copy>(list: &[T]) -> Vec<T> {
    if list.len() <= 1 {
        return list.to_vec();
    }

    let (left, right) = list.split_at(list.len() / 2);

    let merge_left = merge_sort(left);
    let merge_right = merge_sort(right);

    merge(&merge_left, &merge_right)
}

pub fn merge<T: Ord + Copy>(left: &[T], right: &[T]) -> Vec<T> {
    let len_left = left.len();
    let len_right = right.len();

    let mut results = vec![];
    let (mut i, mut j) = (0, 0);

    while i < len_left && j < len_right {
        if left[i] <= right[j] {
            results.push(left[i]);
            i += 1;
        } else {
            results.push(right[j]);
            j += 1;
        }
    }

    // Copy remainder
    while i < len_left {
        results.push(left[i]);
        i += 1;
    }

    while j < len_right {
        results.push(right[j]);
        j += 1;
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i32_short() {
        let mut list: Vec<i32> = vec![3, 1, 2];
        list = merge_sort(&list);
        assert_eq!(list, vec![1, 2, 3]);
    }

    #[test]
    fn test_i32_long() {
        let mut list: Vec<i32> = vec![3, 1, 2, 7, 4, 9, 5, 10, 6, 8];
        list = merge_sort(&list);
        assert_eq!(list, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_i64_short() {
        let mut list: Vec<i64> = vec![3, 1, 2];
        list = merge_sort(&list);
        assert_eq!(list, vec![1, 2, 3]);
    }

    #[test]
    fn test_i64_long() {
        let mut list: Vec<i64> = vec![3, 1, 2, 7, 4, 9, 5, 10, 6, 8];
        list = merge_sort(&list);
        assert_eq!(list, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_char_short() {
        let mut list: Vec<char> = vec!['b', 'c', 'a'];
        list = merge_sort(&list);
        assert_eq!(list, vec!['a', 'b', 'c']);
    }
}
