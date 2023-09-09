use ethers::abi::AbiEncode;
use ethers::core::utils::hex;
use ethers::prelude::*;
use rayon::prelude::*;
use sha2::Digest;

abigen!(IKey, "../curta/out/Puzzle17.sol/IKey.json");

fn to_bytes(i: u64) -> [u8; 32] {
    let mut res = [0u8; 32];
    res[0..8].copy_from_slice(&i.to_le_bytes());
    res
}

fn abi_encode(owner: Address, password: [u8; 32]) -> Vec<u8> {
    SolveThePuzzleOfCoastWithImpressionInNightAndSquallOnCayAndEndToVictoryCall { owner, password }.encode()
}

fn main() {
    let owner = "7714F5E0C26F10584180515FC704C06d4c17d4F0".parse().unwrap();
    let sig = hex::decode("00000000").unwrap();

    // let res = (0..u64::MAX).into_par_iter().find_any(|i| {        
    //     let data = abi_encode(owner, to_bytes(*i));
    //     let digest = sha2::Sha256::digest(&data);
    //     digest.starts_with(&sig)
    // });

    let i: u64 = 12105675798531350165;

    // if let Some(i) = res {
        let password = to_bytes(i);
        let data = abi_encode(owner, password);
        println!("i: {}, data: {:?}, bytes_i: {}", i, data, hex::encode(password));
    // } else {
    //     println!("Crunching failed. Bigger range?");
    // }
}
