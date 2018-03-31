#[derive(Debug, PartialEq)]
pub struct CustomSet<T> {
    data: Vec<T>
}


impl<T> CustomSet<T> {
    pub fn new(data: Vec<T>) -> CustomSet<T> {
        CustomSet { data: data }
    }

    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    pub fn contains(&self, c: &T) -> bool
        where T: PartialEq {
        self.data.iter().find(|&x| x == c).is_some()
    }
}