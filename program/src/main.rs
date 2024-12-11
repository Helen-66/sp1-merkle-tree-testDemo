//! A simple program to be proven inside the zkVM.
//test

#![no_main]

use alloy_merkle_tree::tree::MerkleTree;
use alloy_primitives::{Uint, B256};
sp1_zkvm::entrypoint!(main);

pub fn main() {
    let count = sp1_zkvm::io::read::<u64>();

    let mut tree = MerkleTree::new();

    // 插入整数作为叶子节点
    let mut values = Vec::new(); // 用于存储所有整数值
    println!("Start reading transactions");

    for _ in 0..count {
        // //读取整数
        // let value: u64 = sp1_zkvm::io::read();  // 读取整数
        // values.push(value);  // 将读取的值存储到 `values` 向量中
        // tree.insert(B256::from(Uint::from(value)));

        // 直接读取比特币交易的字节数据
        let value: Vec<u8> = sp1_zkvm::io::read();  // 读取比特币交易字节数据

        // 将字节数据转换为 B256 类型，并插入 Merkle 树
        let leaf = B256::from_slice(&value);
        tree.insert(leaf);
        values.push(value);  // 存储交易数据（如果需要用于后续操作）
    }

    tree.finish();

    // 如果节点数量大于 1，获取第二个节点的证明
    if count > 1 {

        //整数使用
        // let second_value = values[1];  // 获取第二个节点的整数值
        // let proof = tree.create_proof(&B256::from(Uint::from(second_value))).unwrap();  // 创建第二个节点的证明

        //读取字节使用
        let second_value = &values[1];  // 获取第二个节点的值
        let proof = tree.create_proof(&B256::from_slice(&second_value)).unwrap();  // 创建第二个节点的证明

        // 验证证明
        assert!(MerkleTree::verify_proof(&proof));  // 验证该证明是否有效
        println!("Proof for second node (value: {:?}): verified", second_value);
    } else {
        println!("There is no second node in the Merkle tree.");
    }

    sp1_zkvm::io::write(&count);
}
