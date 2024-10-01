/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: std::cmp::PartialOrd>(array: &mut [T]) {
    fn quick_sort<T: std::cmp::PartialOrd>(array: &mut [T], left: usize, right: usize) {
        if right <= left {
            return;
        }
        let p = right;
        let mut i = left;
        for j in left..right {
            if &array[j] < &array[p] {
                array.swap(i, j);
                i += 1;
            }
        }
        array.swap(i, p);
        quick_sort(array, left, i.saturating_sub(1));
        quick_sort(array, i + 1, right);
    }
    quick_sort(array, 0, array.len().saturating_sub(1))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}