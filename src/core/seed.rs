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

/*
enum used as a return to the Seed::roll method
holds the result of the roll and the number that was used
*/
pub enum Roll {
    HIT(u8),
    MISS(u8),
}

impl Seed {
    // create a seed object from a string, sets default index and bit indizes
    pub fn from_str(seed: &str) -> Seed {
        let bytes = Seed::hash(seed);
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
            self.next_hash_iteration();
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

    /*
    roll a u8 number. if its below max return true, 
    the closer to 255 the more likely true is  
    */
    pub fn roll(&mut self, max: u8) -> Roll {
        let roll = self.next_u8();
        if roll < max {
            Roll::HIT(roll)
        } else {
            Roll::MISS(roll)
        }
    }

    // hash a string and return it as a vector of u8
    fn hash(seed: &str) -> Vec<u8> {
        let mut hasher = Sha512::new();
        hasher.update(seed);
        hasher.finalize().to_vec()
    }

    /* 
    take the hash and hash it, 
    this way we have unlimited different randomness
    */
    fn next_hash_iteration(&mut self) {
        let mut iteration = String::from("");

        for byte in &self.bytes {
            iteration.push_str(&byte.to_string());
        }

        self.bytes = Seed::hash(&iteration);
    } 
}