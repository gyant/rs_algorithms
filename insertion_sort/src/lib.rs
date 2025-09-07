use std::cmp::Ord;

pub fn insertion_sort<T: Ord>(list: &mut Vec<T>) {
    for i in 1..list.len() {
        let mut j = i;
        while j > 0 && list[j - 1] > list[j] {
            list.swap(j - 1, j);
            j -= 1;
        }
    }
}

pub fn insertion_sort_partial<T: PartialOrd>(list: &mut Vec<T>) {
    for i in 1..list.len() {
        let mut j = i;
        while j > 0 && list[j - 1] > list[j] {
            list.swap(j - 1, j);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i32_short() {
        let mut list: Vec<i32> = vec![3, 1, 2];
        insertion_sort(&mut list);
        assert_eq!(list, vec![1, 2, 3]);
    }

    #[test]
    fn test_i32_long() {
        let mut list: Vec<i32> = vec![3, 1, 2, 7, 4, 9, 5, 10, 6, 8];
        insertion_sort(&mut list);
        assert_eq!(list, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_i64_short() {
        let mut list: Vec<i64> = vec![3, 1, 2];
        insertion_sort(&mut list);
        assert_eq!(list, vec![1, 2, 3]);
    }

    #[test]
    fn test_i64_long() {
        let mut list: Vec<i64> = vec![3, 1, 2, 7, 4, 9, 5, 10, 6, 8];
        insertion_sort(&mut list);
        assert_eq!(list, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_char_short() {
        let mut list: Vec<char> = vec!['b', 'c', 'a'];
        insertion_sort(&mut list);
        assert_eq!(list, vec!['a', 'b', 'c']);
    }

    #[test]
    fn test_f32_short() {
        let mut list: Vec<f32> = vec![0.32, 0.007, 0.24];
        insertion_sort_partial(&mut list);
        assert_eq!(list, vec![0.007, 0.24, 0.32]);
    }
}
