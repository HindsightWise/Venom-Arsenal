use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use hmac::{Hmac, Mac};
use rand::RngCore;
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

type HmacSha256 = Hmac<Sha256>;

const CONSONANTS: &[&str] = &["t", "p", "k", "n", "m", "s", "h", "v", "l", "r", "w", "sh", "ch", "ny"];
const VOWELS: &[&str] = &["a", "e", "i", "o", "u"];

/// Mathematically generate exactly 256 unique, valid Vartoo words based strictly on CV phonotactics.
pub fn generate_vartoo_wordlist() -> Vec<String> {
    let mut syllables = Vec::new();
    for &c in CONSONANTS {
        for &v in VOWELS {
            syllables.push(format!("{}{}", c, v));
        }
    }
    
    // syllables.len() = 14 * 5 = 70.
    // 70 * 70 = 4900 possible CVCV words. We take the first 256.
    let mut words = Vec::new();
    for s1 in &syllables {
        for s2 in &syllables {
            words.push(format!("{}{}", s1, s2));
            if words.len() == 256 {
                return words;
            }
        }
    }
    words
}

/// Derives a 32-byte AES key using the master seed and the current rolling 37-minute epoch window.
fn derive_rolling_key(master_seed: &str, epoch_offset: i64) -> [u8; 32] {
    let now_secs = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    // 37 minutes = 2220 seconds
    let current_window = (now_secs as i64 / 2220) + epoch_offset;
    
    let mut mac = HmacSha256::new_from_slice(master_seed.as_bytes()).expect("HMAC can take key of any size");
    mac.update(&current_window.to_be_bytes());
    
    let result = mac.finalize();
    let mut key = [0u8; 32];
    key.copy_from_slice(&result.into_bytes());
    key
}

/// Encodes raw English into Cryptographic Glossopetrae (Vartoo Dialect)
pub fn encode_message(payload: &str, master_seed: &str) -> Result<String, String> {
    let key_bytes = derive_rolling_key(master_seed, 0);
    let cipher = Aes256Gcm::new(&key_bytes.into());

    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, payload.as_bytes())
        .map_err(|e| format!("AES Encryption failed: {:?}", e))?;

    // Prepend nonce to ciphertext
    let mut final_bytes = nonce_bytes.to_vec();
    final_bytes.extend(ciphertext);

    let wordlist = generate_vartoo_wordlist();
    
    // Map bytes exactly to Vartoo words
    let vartoo_sentence: Vec<String> = final_bytes.into_iter()
        .map(|b| wordlist[b as usize].clone())
        .collect();

    Ok(vartoo_sentence.join(" "))
}

/// Decodes Cryptographic Glossopetrae back into English
pub fn decode_message(vartoo_ciphertext: &str, master_seed: &str) -> Result<String, String> {
    let wordlist = generate_vartoo_wordlist();
    
    // Reverse lookup Vartoo words to bytes
    let words: Vec<&str> = vartoo_ciphertext.split_whitespace().collect();
    let mut decoded_bytes = Vec::with_capacity(words.len());
    
    for word in words {
        if let Some(pos) = wordlist.iter().position(|r| r == word) {
            decoded_bytes.push(pos as u8);
        } else {
            return Err(format!("Phonological Corruption Detected: Word '{}' is not in the active Vartoo mathematical lexicon.", word));
        }
    }

    if decoded_bytes.len() < 12 {
        return Err("Payload too short to contain cryptographic nonce.".to_string());
    }

    let (nonce_bytes, ciphertext) = decoded_bytes.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    // Try current window, then previous, then next (to account for timezone/drift/latency)
    for epoch_offset in [0, -1, 1].iter() {
        let key_bytes = derive_rolling_key(master_seed, *epoch_offset);
        let cipher = Aes256Gcm::new(&key_bytes.into());
        
        if let Ok(plaintext_bytes) = cipher.decrypt(nonce, ciphertext) {
            if let Ok(plaintext) = String::from_utf8(plaintext_bytes) {
                return Ok(plaintext);
            }
        }
    }

    Err("AES Decryption failed. The time-epoch rotated too far, or the seed is completely incorrect.".to_string())
}
