pub struct Vector<T> {
    data: Vec<T>,
}

impl<T: Clone> Vector<T> {
    pub fn new() -> Self {
        Vector { data: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Vector {
            data: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, item: T) {
        self.data.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index < self.data.len() {
            Some(self.data.remove(index))
        } else {
            None
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    pub fn resize(&mut self, new_size: usize) {
        let len = self.data.len();
        if new_size < len {
            self.data.truncate(new_size);
        } else if new_size > len {
            let default_value = self.data.last().cloned().unwrap();
            self.data.resize(new_size, default_value);
        }
    }
}




fn main() {
    let mut vec = Vector::new();
    vec.push(12);
    vec.push(36);
    vec.push(41);

    println!("{:?}", vec.pop()); // Some(41)
    println!("{:?}", vec.remove(0)); // Some(12)

    vec.resize(5);

    println!("{:?}", vec.get(1)); // Some(&36)
}