pub struct Que {
    pub q_id: u8,
    q_width: u8
}

impl Que {
    pub fn new(width: u8) -> Que {
        Que{
            q_id: 0,
            q_width: width
        }
    }
    pub fn next_in_que(&mut self) {
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
        que.next_in_que();
        que.next_in_que();
        que.next_in_que();
        que.next_in_que();
        assert_eq!(0, que.q_id);
    }
}
