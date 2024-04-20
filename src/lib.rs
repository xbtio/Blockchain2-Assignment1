pub mod sorting {
    use std::cmp::Ordering;

    pub fn quick_sort<T, F>(arr: &mut [T], compare: &F)
    where
        F: Fn(&T, &T) -> Ordering,
    {
        if arr.len() > 1 {
            let pivot_index = partition(arr, compare);
            quick_sort(&mut arr[0..pivot_index], compare);
            quick_sort(&mut arr[pivot_index + 1..], compare);
        }
    }

    fn partition<T, F>(arr: &mut [T], compare: &F) -> usize
    where
        F: Fn(&T, &T) -> Ordering,
    {
        let pivot_index = arr.len() - 1;
        let mut i = 0;
        for j in 0..pivot_index {
            if compare(&arr[j], &arr[pivot_index]) == Ordering::Less {
                arr.swap(i, j);
                i += 1;
            }
        }
        arr.swap(i, pivot_index);
        i
    }

    pub fn selection_sort<T, F>(arr: &mut [T], compare: &F)
    where
        F: Fn(&T, &T) -> Ordering,
    {
        let len = arr.len();
        for i in 0..len {
            let mut min_idx = i;
            for j in i + 1..len {
                if compare(&arr[j], &arr[min_idx]) == Ordering::Less {
                    min_idx = j;
                }
            }
            arr.swap(min_idx, i);
        }
    }

    pub fn insertion_sort<T, F>(arr: &mut [T], compare: &F)
    where
        F: Fn(&T, &T) -> Ordering,
    {
        let len = arr.len();
        for i in 1..len {
            let mut j = i;
            while j > 0 && compare(&arr[j], &arr[j - 1]) == Ordering::Less {
                arr.swap(j, j - 1);
                j -= 1;
            }
        }
    }

    pub fn merge_sort<T, F>(arr: &mut [T], compare: &F)
    where
        T: Clone,  
        F: Fn(&T, &T) -> Ordering,
    {
        let len = arr.len();
        if len > 1 {
            let mid = len / 2;
            merge_sort(&mut arr[0..mid], compare);
            merge_sort(&mut arr[mid..], compare);
            merge(arr, mid, compare);
        }
    }

    fn merge<T, F>(arr: &mut [T], mid: usize, compare: &F)
    where
        T: Clone,  
        F: Fn(&T, &T) -> Ordering,
    {
        let left = arr[..mid].to_vec();
        let right = arr[mid..].to_vec();
        let (mut i, mut j, mut k) = (0, 0, 0);
        while i < left.len() && j < right.len() {
            if compare(&left[i], &right[j]) == Ordering::Less {
                arr[k] = left[i].clone();
                i += 1;
            } else {
                arr[k] = right[j].clone();
                j += 1;
            }
            k += 1;
        }
        while i < left.len() {
            arr[k] = left[i].clone();
            i += 1;
            k += 1;
        }
        while j < right.len() {
            arr[k] = right[j].clone();
            j += 1;
            k += 1;
        }
    }
}
