pub struct Memory {
    max_capacity: u16,
    data: Vec<u8>
}
impl Memory{
    pub fn new(cap: u16) -> Memory {
        Memory{
            max_capacity: cap,
            data: Vec::new()
        }
    }
    pub fn reset(&mut self) {
        self.data.resize(self.max_capacity as usize, 0)
    }

    pub fn set(&mut self, adress: usize, value: u8) {
        self.data[adress] = value;
    }

    pub fn get(&self, adress:usize) -> u8 {
        self.data[adress]
    }
}

get();