use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha512, Digest};

/*
seed can get the bytes from a string
and give you certain RANDOM information like a random bool

it stores the bytes the index of the bytes vector 
and the current position bit
*/
pub struct Seed {
    bytes: Vec<u8>,
    index: usize,
    bit: u8,
}

impl Seed {
    // create a seed object from a string, sets default index and bit indizes
    pub fn from_str(seed: &str) -> Seed {
        let mut hasher = Sha512::new();
        hasher.update(seed);
        let bytes = hasher.finalize().to_vec();

        let index = 0;
        let bit = 0;

        Seed {bytes, index, bit}
    }

    // create a seed object from the current timestamp
    pub fn from_time() -> Result<Seed, String> {
        if let Ok(unix) = SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(Seed::from_str(&unix.as_secs().to_string()))
        } else {
            Err("Could not read system time.".to_string())
        }
    }
 
    /*
    check if the next bit in the system is a 1 
    then move to the next bit and or index in the byte vector
    */
    pub fn next_bool(&mut self) -> bool {
        if 7 < self.bit {
            self.bit = 0;
            self.index += 1;
        } 

        if self.bytes.len() - 1 < self.index {
            self.index = 0;
        }

        let byte = self.bytes[self.index];
        let shifted = byte >> self.bit;
        let new_byte = shifted & 1;

        self.bit += 1;
        new_byte == 1
    }

    /*
    get the next 8 bits in the seed as an u8
    from 0 to 255
    */
    pub fn next_u8(&mut self) -> u8 {
        let mut num: u8 = 0;
        
        for _ in 0..8 {
            // left shift by one place (00010) => (00100)
            num = num << 1;

            num = if self.next_bool() {
                num | 1
            } else {
                num | 0
            };
        } 

        num
    }
}