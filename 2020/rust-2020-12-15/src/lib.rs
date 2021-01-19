use std::{borrow::Borrow, collections::HashMap, hash::Hash, rc::Rc};

#[derive(Default)]
pub struct ListMap<V> {
    map: HashMap<String, Rc<V>>,
    list: Vec<Rc<V>>,
}

pub fn rangecontains() {
    let r = 0..5;
    r.contains(&4);
}
impl<V> ListMap<V> {
    pub fn insert(&mut self, k: String, v: V) {
        let v = Rc::new(v);
        self.map.insert(k, v.clone());
        self.list.push(v);
    }

    pub fn get_by_key<Q>(&self, k: &Q) -> Option<&V>
    where
        String: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.map.get(k).map(Rc::borrow)
    }

    pub fn get_by_idx(&self, idx: usize) -> Option<&V> {
        self.list.get(idx).map(Rc::borrow)
    }
}


#[cfg(test)]
mod test {
    use super::ListMap;
    #[test]
    fn insert_and_get() {
        let mut lm = ListMap::default();
        lm.insert(String::from("HOME"), 127);
        assert_eq!(lm.get_by_key("HOME"), Some(&127));
        assert_eq!(lm.get_by_key("AWAY"), None);
        assert_eq!(lm.get_by_idx(0), Some(&127));
        assert_eq!(lm.get_by_idx(1), None);
    }
}
