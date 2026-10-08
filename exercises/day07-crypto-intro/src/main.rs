use sha2::{Digest, Sha256};

fn main() {
    println!("Dia 7: bytes, hex, hashing - introducao a Rust para criptografia");

    // TODO 1: crie um arquivo de teste (ex: exercises/day07-crypto-intro/sample.txt)
    // e leia com std::fs::read("sample.txt") -> Vec<u8>

    // TODO 2: calcule o SHA-256 do conteudo lido e imprima em hex.
    // Dica (so a mecanica do hasher, o resto e seu):
    let mut hasher = Sha256::new();
    hasher.update(b"exemplo");
    let result = hasher.finalize();
    println!("sha256(\"exemplo\") = {}", hex::encode(result));

    // TODO 3: implemente fn xor_cipher(data: &[u8], key: &[u8]) -> Vec<u8>
    // que aplica XOR byte a byte, repetindo a chave ciclicamente.

    // TODO 4: prove que xor_cipher(xor_cipher(data, key), key) == data
}

// fn xor_cipher(data: &[u8], key: &[u8]) -> Vec<u8> {
//     todo!()
// }
