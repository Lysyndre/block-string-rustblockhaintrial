use std::fmt::{ self, Debug, Formatter };
use super::*; //lib.rs importer

pub struct Block {
    pub index: u32,             //unsigned 32 bit integer that implements where does this lie in the blockchain
    pub timestamp: u128,        //for time verification
    pub hash: Hash,             //at lib
    pub prev_block_hash: Hash,  //lib.rs row 1
    pub nonce: u64,             //lib.rs row 1
    //pub payload: String,        //temporary payload
    pub transactions: Vec<Transaction>, 
    pub difficulty: u128,  
}

impl Debug for Block { //deriving debug is more ideal for production level code.
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Block[{}]: {} at: {} with: {} nonce: {}", /*block, hash , at time , with payload data */
            &self.index,
            &hex::encode(&self.hash), //hex implemented from crates.io into the cargo.toml row 16, reading the documentation is advised before use.
            &self.timestamp,
            &self.transactions.len(),
            &self.nonce,
        )
    }
}

impl Block {
    pub fn new (index: u32, timestamp: u128, prev_block_hash: Hash, transactions: Vec<Transaction>, difficulty: u128) ->  /* is return*/ Self /*is the block */ {
        Block {
            index,
            timestamp,
            hash: vec![0; 32], //unsigned 8 bit integers vector must be assigned hence rust doesn't use null
            prev_block_hash,
            nonce: 0,
            transactions,
            difficulty,
        }
    }

    pub fn mine (&mut self) { //self IMmutable reference taker function. Hence the name entire mining function.
        for nonce_attempt in 0..(u64::max_value()) { //looping trough until hash matches
            self.nonce = nonce_attempt;
            let hash = self.hash();
            if check_difficulty(&hash, self.difficulty) { //does it matches difficulty?
                self.hash = hash;
                return;
            }
        }
    }
}

impl Hashable for Block {   //hashable implemented!
    fn bytes (&self) -> Vec<u8> {   //
        let mut bytes = vec![]; //in rust everything is IMmutable by input so we add mut to the beginning to be able to extend.

        bytes.extend(&u32_bytes(&self.index)); //extend with ...
        bytes.extend(&u128_bytes(&self.timestamp));
        bytes.extend(&self.prev_block_hash);
        bytes.extend(&u64_bytes(&self.nonce));
        //bytes.extend(self.payload.as_bytes());
        bytes.extend(
            self.transactions
                .iter()
                .flat_map(|transaction| transaction.bytes())
                .collect::<Vec<u8>>()
        );
        bytes.extend(&u128_bytes(&self.difficulty));

        bytes
    }
}

pub fn check_difficulty (hash: &Hash, difficulty: u128) -> bool { //checking if difficulty values match
    difficulty > difficulty_bytes_as_u128(&hash)                  //needs to be implemented to = Block struct, pub fn new & Block, imp Hashable.
}
