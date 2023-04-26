use super::*;
use std::fmt::{self, Debug, Formatter};

pub struct Block {
    pub index: u32,            //块高
    pub timestamp: u128,       //时间戳
    pub hash: Hash,            //块hash
    pub prev_block_hash: Hash, //上一个块hash
    pub nonce: u64,
    pub transactions: Vec<Transaction>, //交易列表
    pub difficulty: u128,               //难度值
}

impl Debug for Block {
    //fn fmt(&self, f: &mut Formatter<'_>) -> Result;
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Block[{}]: {} at: {} with: {} nonce: {}",
            &self.index,
            &hex::encode(&self.hash),
            &self.timestamp,
            &self.transactions.len(),
            &self.nonce,
        )
    }
}

impl Block {
    pub fn new(
        index: u32,
        timestamp: u128,
        prev_block_hash: Hash,
        transactions: Vec<Transaction>,
        difficulty: u128,
    ) -> Self {
        Block {
            index,
            timestamp,
            hash: vec![0; 32],
            prev_block_hash,
            nonce: 0,
            transactions,
            difficulty,
        }
    }
    /**
     * 挖矿函数，从0开始遍历，作为nonce值，计算block的字节数组作为hash，
     * 将当前区块hash与难度值比较
     * 如果难度值大于当前hash值，将通过，挖矿产出
     */
    pub fn mine(&mut self) {
        for nonce_attempt in 0..(u64::max_value()) {
            self.nonce = nonce_attempt;
            let hash = self.hash();
            if check_difficulty(&hash, self.difficulty) {
                self.hash = hash;
                return;
            }
        }
    }
}

impl Hashable for Block {
    /**
     * 获取block的字节数组
     */
    fn bytes(&self) -> Vec<u8> {
        //创建一个空的vec
        let mut bytes = vec![];

        //将index,timestamp,pre_block_hash,nonce,transactions分别转为字节类型
        //依次加入bytes中，返回

        bytes.extend(&u32_bytes(&self.index));
        bytes.extend(&u128_bytes(&self.timestamp));
        bytes.extend(&self.prev_block_hash);
        bytes.extend(&u64_bytes(&self.nonce));
        bytes.extend(
            self.transactions
                .iter()
                .flat_map(|transaction| transaction.bytes())
                .collect::<Vec<u8>>(),
        );
        bytes.extend(&u128_bytes(&self.difficulty));

        bytes
    }
}

/**
 * 当 difficulty 值大于hash字节转为u128整型值的时候，则验证通过
 */
pub fn check_difficulty(hash: &Hash, difficulty: u128) -> bool {
    difficulty > difficulty_bytes_as_u128(&hash)
}
