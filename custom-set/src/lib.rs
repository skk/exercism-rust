use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub struct CustomSet<T: Ord> {
    data: Vec<T>
}


impl<T: Ord> CustomSet<T>
    where T: PartialEq + Clone + Debug {
    pub fn new(data: Vec<T>) -> CustomSet<T> {
        let mut set = CustomSet { data: vec![] };
        for element in data {
            set._add(&element);
        }
        set
    }

    pub fn new_from_vec_of_refs(data: Vec<&T>) -> CustomSet<T> {
        let mut set = CustomSet { data: vec![] };
        for element in data {
            set._add(element);
        }
        set
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn contains(&self, c: &T) -> bool where T: PartialEq {
        self.data.iter().find(|&x| x == c).is_some()
    }

    fn _add(&mut self, c: &T) -> () where T: PartialEq + Clone + Debug {
        if !self.contains(&c) {
            self.data.push(c.clone());
            self.data.sort();
        }
    }

    pub fn add(&mut self, c: T) -> () where T: PartialEq + Clone + Debug {
        if !self.contains(&c) {
            self.data.push(c);
            self.data.sort();
        }
    }

    pub fn is_subset(&self, other: &CustomSet<T>) -> bool {
        self.data.iter().all(|data| other.contains(data))
    }

    pub fn is_disjoint(&self, other: &CustomSet<T>) -> bool {
        !self.data.iter().any(|data| other.contains(data))
    }

    pub fn intersection(&self, other: &CustomSet<T>) -> CustomSet<T> {
        let data = self.data.iter().filter(|x| other.contains(x)).collect::<Vec<_>>();
        CustomSet::new_from_vec_of_refs(data)
    }

    pub fn union(&self, other: &CustomSet<T>) -> CustomSet<T> {
        let data = self.data.iter().chain(other.data.iter()).collect::<Vec<_>>();
        CustomSet::new_from_vec_of_refs(data)
    }

    pub fn difference(&self, other: &CustomSet<T>) -> CustomSet<T> {
        let data = self.data.iter().filter(|x| !other.contains(x)).collect::<Vec<_>>();
        CustomSet::new_from_vec_of_refs(data)
    }
}