pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value)
    }

    pub fn remove(&mut self) -> Option<i32> {
        if let Some(value) = self.list.pop() {
            self.update_average();
            Some(value)
        } else {
            None
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let sum: i32 = self.list.iter().sum();
        self.average = sum as f64 / self.list.len() as f64;
    }
}

fn main() {}
