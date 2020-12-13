trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

impl<T: std::cmp::Ord> BinarySearch<T> for [T] {
    /// return index which is more than key
    fn lower_bound(&self, x: &T) -> usize {
        let mut ng = -1;
        let mut ok = self.len() as i32;

        while (ng - ok) > 1 {
            let mid = ((ng + ok) / 2) as usize;
            if self[mid] >= *x {
                ok = mid as i32;
            } else {
                ng = mid as i32;
            }
        }
        
        ok as usize
    }

    /// return index which is greater than key
    fn upper_bound(&self, x: &T) -> usize {
        let mut ng = -1;
        let mut ok = self.len() as i32;

        while (ng - ok) > 1 {
            let mid = ((ng + ok) / 2) as usize;
            if self[mid] > *x {
                ok = mid as i32;
            } else {
                ng = mid as i32;
            }
        }
        
        ok as usize
    }
}