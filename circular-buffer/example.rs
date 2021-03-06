#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

pub struct CircularBuffer {
    buffer: Vec<char>,
    size: usize,
    start: usize,
    end: usize,
}

impl CircularBuffer {
    // this circular buffer keeps an unallocated slot between the start and the end
    // when the buffer is full. 
    pub fn new(size: usize) -> CircularBuffer {
        CircularBuffer { 
            buffer: vec!['0'; size + 1], 
            size: size + 1, 
            start: 0, 
            end: 0 
        }
    }

    pub fn read(&mut self) -> Result<char, Error> {
        if self.is_empty() {
            return Err(Error::EmptyBuffer);
        }

        let v = *self.buffer.get(self.start).unwrap();
        self.advance_start();
        Ok(v)
    }

    pub fn write(&mut self, byte: char) -> Result<(), Error> {
        if self.is_full() {
            return Err(Error::FullBuffer);
        } 
        
        self.buffer[self.end] = byte;
        self.advance_end();
        Ok(())
        
    }

    pub fn overwrite(&mut self, byte: char) {
        if self.is_full() {
            self.advance_start();
        }

        self.buffer[self.end] = byte;
        self.advance_end();
    }

    pub fn clear(&mut self) {
        self.start = 0;
        self.end = 0;
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    pub fn is_full(&self) -> bool {
        (self.end + 1) % self.size == self.start
    }

    fn advance_start(&mut self) {
        self.start = (self.start + 1) % self.size;
    }

    fn advance_end(&mut self) {
        self.end = (self.end + 1) % self.size;
    }

}
