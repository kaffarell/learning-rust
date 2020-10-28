struct Difference {
    elements: Vec<i32>,
    max_difference: i32,
}

impl Difference {
    fn new(elements: Vec<i32>) -> Difference {
        return Difference{elements: elements, max_difference: 0};
    }

    fn compute_difference(&mut self) {
        for i in 0..self.elements.len() {
            for a in 0..self.elements.len() {
                if self.elements[a] - self.elements[i] > self.max_difference {
                    self.max_difference = self.elements[a] - self.elements[i];
                }
            }
        }
    }
}

pub fn run() {
    let array = vec![1, 2, 5];
    let mut d: Difference = Difference::new(array);
    d.compute_difference();
    println!("{}", d.max_difference);
}