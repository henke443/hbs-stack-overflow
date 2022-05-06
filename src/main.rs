use hbs_lms::*;
use rand_chacha::ChaCha20Rng;
use rand::prelude::*;

fn main() {
    let mut rng = ChaCha20Rng::from_entropy();

    let parameter = HssParameter::<Shake256>::new(LmotsAlgorithm::LmotsW2, LmsAlgorithm::LmsH10);
    
    let mut seed = [0u8; 32];
    
    rng.fill(&mut seed);

    // aux data
    let mut aux_data = vec![0u8; 100_000_000u64 as usize];
    let aux_slice: &mut &mut [u8] = &mut &mut aux_data[..];


    // seed is 32 random bytes [u8; 32]
    let (signing_key, verifying_key) = keygen(&[parameter], &seed, None)
    .unwrap_or_else(|_| panic!("Could not generate keys"));
}