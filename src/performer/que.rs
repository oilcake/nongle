#[derive(Copy, Clone, Debug)]
pub struct Que {
    pub q_id: usize,
    pub q_width: usize
}

impl Que {
    pub fn new(width: usize) -> Self {
        Que{
            q_id: 0,
            q_width: width
        }
    }
    pub fn next(&mut self) {
        self.q_id += 1;
        self.q_id = self.q_id % self.q_width;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_index_in_que() {
        let mut que = Que::new(2);
        que.next();
        que.next();
        que.next();
        que.next();
        assert_eq!(0, que.q_id);
    }
}
