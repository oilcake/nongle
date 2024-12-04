#[derive(Copy, Clone, Debug)]
enum QueMode {
    Down,
    Up,
    // Random
}
#[derive(Copy, Clone, Debug)]
pub struct Que {
    id: usize,
    width: usize,
    mode : QueMode
}

impl Que {
    pub fn new(width: usize, mode: QueMode) -> Self {
        Que{
            id: 0,
            width,
            mode: mode
        }
    }
    pub fn next(&mut self) {
        self.id += 1;
        self.id = self.id % self.width;
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

    #[test]
    fn next_index_in_que_up() {
        let mut que = Que::new(4, QueMode::Up);
        que.next();
        que.next();
        que.next();
        que.next();
        assert_eq!(4, que.id);
    }
}
