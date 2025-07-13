#![allow(dead_code)]
#[derive(Copy, Clone, Debug)]
// Down mode will always reduce index at each new iteration
// until it reaches the lowest point of que
// Up is the opposite
pub enum QueMode {
    Down,
    Up,
    // Random
}
// id represents an actual index in the queue
// width is a size of cycle in which we can switch notes
#[derive(Copy, Clone, Debug)]
pub struct Que {
    id: usize,
    pub width: usize,
    mode : QueMode
}

impl Que {
    pub fn new(width: usize, mode: QueMode) -> Self {
        Que{
            id: 0,
            width,
            mode
        }
    }
    pub fn next(&mut self) {
        self.id += 1;
        self.id = self.id % self.width;
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_index_in_que_down() {
        let mut que = Que::new(2, QueMode::Down);
        que.next();
        que.next();
        que.next();
        que.next();
        assert_eq!(0, que.id);
    }
}
