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
    // index of currtntly active layer
    id: usize,
    // width is a width of moving window
    width: usize,
    // depth is a number of layers in corresponding note
    depth: usize,
    mode : QueMode
}

impl Que {
    /// If witdth is not passed to constructor, it will be set to default
    pub fn new(width: usize, depth: usize, mode: QueMode) -> Self {
        Que{
            id: 0,
            width,
            depth,
            mode
        }
    }
    pub fn next(&mut self) {
        self.id += 1;
        self.id %= self.width;
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
    pub fn depth(&self) -> usize {
        self.depth
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn set_width(&mut self, width: usize) {
        self.width = width.min(self.depth);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_index_in_que_down() {
        let mut que = Que::new(2, 4, QueMode::Down);
        que.next();
        que.next();
        que.next();
        que.next();
        assert_eq!(0, que.id);
    }
}
