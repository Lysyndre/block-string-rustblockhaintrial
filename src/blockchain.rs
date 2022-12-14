use super::*; //implementing the row 89 & 90 @ lib.rs
use std::collections::HashSet; //access to unspent outputs

#[derive(Debug)] //directly showing text as error in enum.
pub enum BlockValidationErr {
    MismatchedIndex,    //Indexes does not match
    InvalidHash,        // Hash is invalid (waht is hash)
    AchronologicalTimestamp, //Time order malfunction
    MismatchedPreviousHash, //Previous hash does not match
    InvalidGenesisBlockFormat,  //initial format is not valid
    InvalidInput,               //Invalid input
    InsufficientInputValue,     //Insufficient amount
    InvalidCoinbaseTransaction, //???
}

/*impl Blockchain { //entire verify function
    pub fn verify (&self) -> bool {
        for (i, block) in self.blocks.iter().enumerate() {
            if block.index != i as u32 {
                println!("Indec mismatch {} != {}", &block.index, &i);
                return false;
            }
            else if !block::check.difficulty(block.hash(), block.difficulty)
            {
                println!("Difficulty fail.");
            }
            else if i != 0 {
                //not genesis block
                let prev_block = &self.blocks[i - 1];
                if block.timestamp <= prev_block.timestamp {
                    println!("Time did not increase");
                    return false;
                }
                else if block.prev_block_hash != prev_block.hash {
                    println!("Hash mismatch");
                }
            }
            else {
                //Genesis block
                if block.prev_block_hash != vec![0; 32] {
                    println!("Genesis block prev_block_hash invalid");
                }
            }
        }
    }
}*/

pub struct Blockchain {
    pub blocks: Vec<Block>,
    unspent_outputs: HashSet<Hash>,
}

impl Blockchain {
    pub fn new () -> Self {
        Blockchain {
            blocks: vec![],
            unspent_outputs: HashSet::new(),
        }
    }

    pub fn update_with_block (&mut self, block: Block) -> Result<(), BlockValidationErr> {
        let i = self.blocks.len();

        if block.index != i as u32 {
            return Err(BlockValidationErr::MismatchedIndex);
        } else if !block::check_difficulty(&block.hash(), block.difficulty) {
            return Err(BlockValidationErr::InvalidHash);
        } else if i != 0 {
            // Not genesis block
            let prev_block = &self.blocks[i - 1];
            if block.timestamp <= prev_block.timestamp {
                return Err(BlockValidationErr::AchronologicalTimestamp);
            } else if block.prev_block_hash != prev_block.hash {
                return Err(BlockValidationErr::MismatchedPreviousHash);
            }
        } else {
            // Genesis block
            if block.prev_block_hash != vec![0; 32] {
                return Err(BlockValidationErr::InvalidGenesisBlockFormat);
            }
        }

        if let Some((coinbase, transactions)) = block.transactions.split_first() { //
            if !coinbase.is_coinbase() {
                return Err(BlockValidationErr::InvalidCoinbaseTransaction);
            }

            let mut block_spent: HashSet<Hash> = HashSet::new();
            let mut block_created: HashSet<Hash> = HashSet::new();
            let mut total_fee = 0;

            for transaction in transactions {
                let input_hashes = transaction.input_hashes();

                if
                    !(&input_hashes - &self.unspent_outputs).is_empty() ||  !(&input_hashes & &block_spent).is_empty()
                {
                    return Err(BlockValidationErr::InvalidInput);
                }

                let input_value = transaction.input_value();
                let output_value = transaction.output_value();

                if output_value > input_value {
                    return Err(BlockValidationErr::InsufficientInputValue);
                }

                let fee = input_value - output_value;

                total_fee += fee;

                block_spent.extend(input_hashes);
                block_created.extend(transaction.output_hashes());
            }

            if coinbase.output_value() < total_fee {
                return Err(BlockValidationErr::InvalidCoinbaseTransaction);
            } else {
                block_created.extend(coinbase.output_hashes());
            }

            self.unspent_outputs.retain(|output| !block_spent.contains(output));
            self.unspent_outputs.extend(block_created);
        }

        self.blocks.push(block);

        Ok(())
    }
}
