//! A simple script to generate and verify the proof of a given program.

use sp1_core::{SP1Prover, SP1Stdin, SP1Verifier};
//use serde::{Serialize};  // 导入 Serialize trait
//use serde_json;          // 导入 serde_json 库

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    let start = std::time::Instant::now();
    // Generate proof.

    // //整数使用
    // //示例整数数据，假设我们有6个整数
    // let integers: Vec<u8> = (0..6).collect::<Vec<u8>>(); // 生成6个整数

    // // 创建标准输入并写入整数数量
    // let mut stdin = SP1Stdin::new();
    // let count: u64 = integers.len() as u64;  // 整数数量
    // println!("count: {}", count);
    // stdin.write(&count);

    // // 将整数数据作为输入传递给 zkVM 程序
    // for &value in &integers {
    //     stdin.write(&value); // 每个整数写入标准输入
    //     println!("value: {}", value);
    // }

    // 示例比特币交易数据（这里只是示例，实际应用时需要提供真实的交易字节）
    let transactions: Vec<Vec<u8>> = vec![
        vec![0x01, 0x02, 0x03, 0x04], // 交易 1 字节数据
         vec![0x05, 0x06, 0x07, 0x08], // 交易 2 字节数据
        // vec![0x09, 0x0A, 0x0B, 0x0C], // 交易 3 字节数据
        // vec![0x02, 0x0A, 0x01, 0x08], // 交易 4 字节数据
    ];

    // 创建标准输入并写入交易数量
    let mut stdin: SP1Stdin = SP1Stdin::new();
    let count: u64 = transactions.len() as u64;
    println!("count: {}", count);
    stdin.write(&count);

    //将交易字节数据作为输入传递给 zkVM 程序
    for tx in &transactions {

        stdin.write_slice(tx);  // 用切片将每个交易的字节数据传入 zkVM 程序
        // stdin.write(tx);  // 将每个交易的字节数据传入 zkVM 程序

        //转换成json，同样也序列化失败
        //let json = serde_json::to_string(&tx).expect("Failed to serialize vector to JSON");
        //stdin.write(&json);  // 将每个交易的字节数据传入 zkVM 程序

        println!("Transaction data: {:?}", tx);
    }

    let mut proof = SP1Prover::prove(ELF, stdin).expect("proving failed");

    // Read output.
    let count = proof.stdout.read::<u64>();
    println!("count: {}", count);
    let end = std::time::Instant::now();
    println!("Proof generation time: {:?}", end.duration_since(start));

    // Verify proof.
    SP1Verifier::verify(ELF, &proof).expect("verification failed");

    // Save proof.
    proof
        .save("proof-with-io.json")
        .expect("saving proof failed");

    println!("succesfully generated and verified proof for the program!")
}
