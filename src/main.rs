use blockchainlib::*; //implementing defined library at cargo.toml row 8

fn main () {
    println!("Hello Patika!");

    /*
    let block = Block::new(0, 0, vec![0;32], 0, "Genesis block".to_owned());
    println!("{:?}", &block); //displays empty block

    let h = block.hash();

    println!("{:?}", &h); //by using debug form header &h works!
    //if you change nonce to any value and compare the before and after result you can see there is no resemblance in between, avalanching achieved!!!

    block.hash = h;
     println!("{:?}", &block); 
     ///End of first video///
    */

    /*
    let mut block = Block::new(0, 0, vec![0;32], 69420, "Genesis block!".to_owned(), 0x0000ffffffffffffffffffffffffffff);
    block.hash = block.hash();
    println!("{:?}", &block);
    block.mine();
    println!("{:?}", &block);
    ///End of second video///
     */

     /*
    let difficulty = 0x00ffffffffffffffffffffffffffffff;
    let mut block = Block::new(0, now(), vec![0;32], 0, "Genesis block!".to_owned(), difficulty);
    blockd.mine();
    println!("Mined genesis block{:?}", &block);
    let mut last_hash = block.hash.clone();         //keeping track of the most fresh hash
    let mut blockchain = Blockchain {
        blocks: vec![block],
    };
    blockchain.blocks[3].index = 4; //distrupting the chain on purpose, we shall see "index mismatch" error on the compiler
    blockchain.blocks[3].hash[8] += 1; //distrupting the chain on purpose, we shall see "hash mismatch" error on the compiler
    blockchain.blocks[3].payload = "Bullocks".to_owned(); //distrupting the chain on purpose, we shall see "difficulty fail" error on the compiler
    blockchain.blocks[3].prev_block_hash[18] = 8; //distrupting the chain on purpose, we shall see "difficulty fail" error on the compiler
    println!("Verify: {}", &blockchain.verify());
    return;
    for i in 1..=10 {
        let mut block = Block::new(i, now(), last hash, 0, "Another block!".to_owned(), difficulty);
        block.mine();
        last_hash = block.hach.clone(); //always lagging one behind
        println!("Mined block{:?}", &block);
        blockchain.blocks.push(block);
        println!("Verify: {}", &blockchain.verify());
        ///End of third video///
    }
     */

     /*
    asdads
     */

    
    let difficulty = 0x00ffffffffffffffffffffffffffffff;    /*most significant byte of block must be zero in order to be this block considered mined 
                                                                    (otherwise it would require heavier computational power), aslo same nonce gives the same hash*/

    let mut genesis_block = Block::new(0, now(), vec![0; 32], vec![
        Transaction {
            inputs: vec![ ],
            outputs: vec![
                transaction::Output {
                    to_addr: "Alice".to_owned(),
                    value: 50,
                },
                transaction::Output {
                    to_addr: "Bob".to_owned(),
                    value: 7,
                },
            ],
        },
    ], difficulty);

    genesis_block.mine();

    println!("Mined genesis block {:?}", &genesis_block);

    let mut last_hash = genesis_block.hash.clone();

    let mut blockchain = Blockchain::new();

    blockchain.update_with_block(genesis_block).expect("Failed to add genesis block");

    let mut block = Block::new(1, now(), last_hash, vec![
        Transaction {
            inputs: vec![ ],
            outputs: vec![
                transaction::Output {
                    to_addr: "Chris".to_owned(),
                    value: 536,
                },
            ],
        },
        Transaction {
            inputs: vec![
                blockchain.blocks[0].transactions[0].outputs[0].clone(),
            ],
            outputs: vec![
                transaction::Output {
                    to_addr: "Alice".to_owned(),
                    value: 33,
                },
                transaction::Output {
                    to_addr: "Bob".to_owned(),
                    value: 17,
                },
            ],
        },
    ], difficulty);

    block.mine();

    println!("Mined block {:?}", &block);

    last_hash = block.hash.clone();

    blockchain.update_with_block(block).expect("Failed to add block");
}
