trait Adder<T: num::Signed + num::NumCast + Debug> {
    fn add(&self, i: (T, T)) -> Option<(usize, usize)>;
}

impl<T: num::Signed + num::NumCast + Debug> Adder<T> for (usize, usize) {
    fn add(&self, i: (T, T)) -> Option<(usize, usize)> {
        let mut ans = (None, None);
        let &&u = &self;

        let i_0_abs = i.0.abs().to_usize().unwrap();
        let i_1_abs = i.1.abs().to_usize().unwrap();

        ans.0 = match i.0.is_negative() {
            true => u.0.checked_sub(i_0_abs),
            false => u.0.checked_add(i_0_abs),
        };
        ans.1 = match i.1.is_negative() {
            true => u.1.checked_sub(i_1_abs),
            false => u.1.checked_add(i_1_abs),
        };

        if ans.0.is_none() || ans.1.is_none() {
            None
        } else {
            Some((ans.0.unwrap(), ans.1.unwrap()))
        }
    }
}