pub struct BinarySearch<T: Ord> {
    _marker: std::marker::PhantomData<T>
}

pub trait BSearch<T: Ord> {
    fn search(self, elem: &T) -> Option<usize>;
}

impl<T: Ord> BSearch<T> for &[T] {
    fn search(self, elem: &T) -> Option<usize> {
        BinarySearch::search(self, elem)
    }
}

impl<T: Ord> BinarySearch<T> {
    pub fn search(list: &[T], elem: &T) -> Option<usize> {
        let left = 0;
        let right = list.len();
        
        Self::search_in(list, elem, left, right)
    }

    fn search_in(list: &[T], elem: &T, left: usize, right: usize) -> Option<usize> {
        let mid = left + (right - left) / 2;

        if left >= right {
            return None;
        }

        match elem.partial_cmp(&list[mid]).unwrap() {
            std::cmp::Ordering::Less => Self::search_in(list, elem, left, mid),
            std::cmp::Ordering::Greater => Self::search_in(list, elem, mid+1, right),
            std::cmp::Ordering::Equal => Some(mid),
        }
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn bs_test() {
        use super::BSearch;
        let bs = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(bs.search(&1), Some(0));
        assert_eq!(bs.search(&2), Some(1));
        assert_eq!(bs.search(&3), Some(2));
        assert_eq!(bs.search(&4), Some(3));
        assert_eq!(bs.search(&5), Some(4));
        assert_eq!(bs.search(&6), Some(5));
        assert_eq!(bs.search(&7), Some(6));
        assert_eq!(bs.search(&8), Some(7));
        assert_eq!(bs.search(&9), Some(8));
        assert_eq!(bs.search(&10), None);
    }

    #[test]
    // #[ignore]
    fn bs_random_test() {
        use super::BinarySearch;
        let big_vec = (0..10000).collect::<Vec<u32>>();

        assert!(BinarySearch::search(&big_vec, &1337).is_some());
    }


}
