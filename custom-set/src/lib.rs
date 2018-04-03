use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub struct CustomSet<T: Ord> {
    data: Vec<T>
}


impl<T: Ord> CustomSet<T>
 where T: PartialEq + Clone + Debug {
    pub fn new(data: Vec<T>) -> CustomSet<T> {
        let mut set = CustomSet{data: vec![]};
        for element in data {
            set.add(element);
        }
        set
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn contains(&self, c: &T) -> bool where T: PartialEq {
        self.data.iter().find(|&x| x == c).is_some()
    }

    pub fn add(&mut self, c: T) -> () where T: PartialEq + Clone + Debug {
        if ! self.contains(&c) {
            self.data.push(c);
            self.data.sort();
        }
    }
}