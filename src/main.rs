use blockchainlib::*;

fn main () {
    // 初始化一条链
    let mut blockchain = Blockchain::new();


    // 区块难度值
    let difficulty = 0x000fffffffffffffffffffffffffffff;

    //创建初始块
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
    //执行挖矿
    genesis_block.mine();

    println!("Mined genesis block {:?}", &genesis_block);
    let last_hash = genesis_block.hash.clone();
    // 上链
    blockchain.update_with_block(genesis_block).expect("Failed to add genesis block");

    // 基于上一个块hash,创建一个新区块
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
                    value: 360,
                },
                transaction::Output {
                    to_addr: "Bob".to_owned(),
                    value: 12,
                },
            ],
        },
    ], difficulty);
    //执行挖矿
    block.mine();

    println!("Mined block {:?}", &block);
    // last_hash = block.hash.clone();
    //上链
    blockchain.update_with_block(block).expect("Failed to add block");
}
