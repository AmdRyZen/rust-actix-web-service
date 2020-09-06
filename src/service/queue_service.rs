use std::result::Result;

#[derive(Debug)]
pub struct Queue<T> {
    pub qdata: Vec<T>,
    pub size: usize,
}

impl<T> Queue<T> {
    pub async fn new(size: usize) -> Self {
        Queue {
            qdata: Vec::with_capacity(size),
            size: size,
        }
    }

    pub async fn enqueue(&mut self, item: T) -> Result<(), String> {
        if self.qdata.len() >= self.size {
            return Err("exceed size".to_string());
        }
        self.qdata.push(item);
        Ok(())
    }

    pub async fn dequeue(&mut self) -> Option<T> {
        let size = self.qdata.len();
        if size > 0 {
            let v = self.qdata.remove(0);
            Some(v)
        } else {
            None
        }
    }

    pub async fn size(&mut self) -> usize {
        self.qdata.len()
    }
}
