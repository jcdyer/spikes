//use std::i64;

use std::cmp::Ordering;

/// ZoneMap maintains a Vec of data along with maintaining pre-computed
/// information about that data.
#[derive(Clone, Default)]
pub struct ZoneMap {
    data: Vec<i64>,
    min: i64,
    max: i64,
    mincount: usize,
    maxcount: usize,
    sum: i64,
}

impl ZoneMap {
    pub fn new(data: Vec<i64>) -> ZoneMap {
        let mut min = i64::MAX;
        let mut max = i64::MIN;
        let mut sum = 0;
        let mut mincount = 0;
        let mut maxcount = 0;

        for &i in &data {
            sum += i;
            match i.cmp(&min) {
                Ordering::Less => {
                    min = i;
                    mincount = 1;
                }
                Ordering::Equal => mincount += 1,
                _ => {}
            }
            match i.cmp(&max) {
                Ordering::Greater => {
                    max = i;
                    maxcount = 1;
                }
                Ordering::Equal => maxcount += 1,
                _ => {}
            }
        }

        ZoneMap {
            data,
            min,
            max,
            sum,
            mincount,
            maxcount,
        }
    }

    pub fn append(&mut self, value: i64) {
        self.sum += value;
        self.data.push(value);

        match value.cmp(&self.min) {
            Ordering::Less => {
                self.min = value;
                self.mincount = 1;
            }
            Ordering::Equal => self.mincount += 1,
            _ => {}
        }
        match value.cmp(&self.max) {
            Ordering::Greater => {
                self.max = value;
                self.maxcount = 1;
            }
            Ordering::Equal => self.maxcount += 1,
            _ => {}
        }
    }

    /// panics if index is out of bounds.
    pub fn remove(&mut self, index: usize) {
        let oldvalue = self.data.remove(index);

        self.sum -= oldvalue;

        if oldvalue == self.min {
            self.mincount -= 1;

            // If removing the last min, recalculate

            if self.mincount == 0 {
                self.min = i64::MAX;
                for &value in &self.data {
                    match value.cmp(&self.min) {
                        Ordering::Less => {
                            self.min = value;
                            self.mincount = 1;
                        }
                        Ordering::Equal => self.mincount += 1,
                        _ => {}
                    }
                }
            }
        }

        if oldvalue == self.max {
            self.maxcount -= 1;

            // If removing the last max, recalculate

            if self.maxcount == 0 {
                self.max = i64::MIN;
                for &value in &self.data {
                    match value.cmp(&self.max) {
                        Ordering::Greater => {
                            self.max = value;
                            self.maxcount = 1;
                        }
                        Ordering::Equal => self.maxcount += 1,
                        _ => {}
                    }
                }
            }
        }
    }

    pub fn replace(&mut self, index: usize, value: i64) {
        let oldvalue = self.data[index];
        self.data[index] = value;

        self.sum += value - oldvalue;

        // If the old value was a min, decrement the count,
        // but do not yet reset self.min if no longer valid.
        if oldvalue == self.min {
            self.mincount -= 1;
        }

        // Compare against the old min, even if it is being removed
        match value.cmp(&self.min) {
            Ordering::Less => {
                self.min = value;
                self.mincount = 1;
            }
            Ordering::Equal => self.mincount += 1,
            _ => {}
        }

        if self.mincount == 0 {
            // We no longer have enough information to calculate the min
            // without iterating over our whole dataset.
            self.min = i64::MAX;
            for &value in &self.data {
                match value.cmp(&self.min) {
                    Ordering::Less => {
                        self.min = value;
                        self.mincount = 1;
                    }
                    Ordering::Equal => {
                        self.mincount += 1;
                    }
                    _ => {}
                }
            }
        }

        if oldvalue == self.max {
            self.maxcount -= 1;
        }
        match value.cmp(&self.max) {
            Ordering::Greater => {
                self.max = value;
                self.maxcount = 1;
            }
            Ordering::Equal => self.maxcount += 1,
            _ => {}
        }

        if self.maxcount == 0 {
            // We no longer have enough information to calculate the max
            // without iterating over our whole dataset.
            self.max = i64::MIN;
            for &value in &self.data {
                match value.cmp(&self.max) {
                    Ordering::Greater => {
                        self.max = value;
                        self.maxcount = 1;
                    }
                    Ordering::Equal => {
                        self.maxcount += 1;
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn count(&self) -> usize {
        self.data.len()
    }

    pub fn avg(&self) -> i64 {
        self.sum / self.count() as i64
    }

    pub fn min(&self) -> i64 {
        self.min
    }

    pub fn max(&self) -> i64 {
        self.max
    }

    pub fn sum(&self) -> i64 {
        self.sum
    }
}

#[cfg(test)]
mod tests {
    use super::ZoneMap;
    #[test]
    fn creation() {
        let z = ZoneMap::new(vec![100, 200, 300, 400, 400]);
        assert_eq!(z.min(), 100);
        assert_eq!(z.max(), 400);
        assert_eq!(z.sum(), 1400);
        assert_eq!(z.count(), 5);
        assert_eq!(z.avg(), 280);
    }

    #[test]
    fn append() {
        let mut z = ZoneMap::new(vec![]);
        z.append(100);
        z.append(200);
        z.append(300);
        z.append(400);
        z.append(400);
        assert_eq!(z.min(), 100);
        assert_eq!(z.max(), 400);
        assert_eq!(z.sum(), 1400);
        assert_eq!(z.count(), 5);
        assert_eq!(z.avg(), 280);
    }

    #[test]
    fn replace_new_min() {
        let mut z = ZoneMap::new(vec![100, 200, 300, 400, 400]);
        z.replace(2, 0);
        assert_eq!(z.min(), 0);
        assert_eq!(z.max(), 400);
        assert_eq!(z.sum(), 1100);
        assert_eq!(z.count(), 5);
        assert_eq!(z.avg(), 220);
    }

    #[test]
    fn replace_max() {
        let mut z = ZoneMap::new(vec![100, 200, 300, 400, 400]);
        z.replace(3, 350);
        assert_eq!(z.max(), 400);
        z.replace(4, 50);
        assert_eq!(z.max(), 350);
    }
}

#[cfg(test)]
mod property_tests {
    use super::ZoneMap;
    use quickcheck::quickcheck;

    quickcheck! {
        #[test]
        fn max(data: Vec<i64>) -> bool {
            let max = data.iter().copied().max().unwrap_or(i64::MIN);
            let zm = ZoneMap::new(data);
            zm.max() == max
        }

        #[test]
        fn sum(data: Vec<i64>) -> bool {
            let sum = data.iter().sum();
            let zm = ZoneMap::new(data);
            zm.sum() == sum
        }

        #[test]
        fn modify_min(v1: Vec<i64>, v2: Vec<i64>) -> bool {
            let (data, mods) = if v1.len() > v2.len() {
                (v1, v2)
            } else {
                (v2, v1)
            };
            let mut control = data.clone();
            let mut zm = ZoneMap::new(data);
            for (i, val) in mods.into_iter().enumerate() {
                zm.replace(i, val);
                control[i] = val;
            }
            zm.min() == control.into_iter().min().unwrap_or(i64::MAX)
        }

        #[test]
        fn modify_max(v1: Vec<i64>, v2: Vec<i64>) -> bool {
            let (data, mods) = if v1.len() > v2.len() {
                (v1, v2)
            } else {
                (v2, v1)
            };
            let mut control = data.clone();
            let mut zm = ZoneMap::new(data);
            for (i, val) in mods.into_iter().enumerate() {
                zm.replace(i, val);
                control[i] = val;
            }
            zm.max() == control.into_iter().max().unwrap_or(i64::MIN)
        }

        fn modify_sum(v1: Vec<i64>, v2: Vec<i64>) -> bool {
            let (data, mods) = if v1.len() > v2.len() {
                (v1, v2)
            } else {
                (v2, v1)
            };
            let mut control = data.clone();
            let mut zm = ZoneMap::new(data);
            for (i, val) in mods.into_iter().enumerate() {
                zm.replace(i, val);
                control[i] = val;
            }
            zm.sum() == control.into_iter().sum::<i64>()
        }
    }

    #[test]
    fn regress_modify_min_1() {
        let mut zm = ZoneMap::new(vec![-1, 1]);
        zm.replace(0, 0);
        assert_eq!(zm.min(), 0);
    }
}
