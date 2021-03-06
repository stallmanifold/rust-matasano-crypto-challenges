use cryptanalysis::frequency_analysis::Histogram;
use num_rational::Ratio;
use std::iter::IntoIterator;
use std::hash::Hash;
use crypto::xor_cipher::SingleCharXorCipher;
use crypto::xor_cipher::BlockCipher;
use bitwise::bitwiseops;
use bitwise::transpose::TransposeChunks;


pub fn score_with<T>(model: &Histogram<T>, string: &[T]) -> Ratio<usize>
    where T: Eq + Hash + Copy {

    string.into_iter().fold(Ratio::from_integer(0), |score, ch| { 
        if model.contains_key(ch) {
            score + model.get(ch).unwrap()
        } else {
            score
        }
    })
}

pub fn max_char_with(model: &Histogram<u8>, charset: &[u8], string: &[u8]) -> (u8, Ratio<usize>) {
    let mut max_score = Ratio::from_integer(0);
    let mut max_char  = 0x00;
    
    for ch in charset {
        let cipher      = SingleCharXorCipher::new(*ch);
        let cipher_text = cipher.process_block(string);
        let score       = score_with(model, &cipher_text);

        if score >= max_score {
            max_score = score;
            max_char  = *ch;
        }
    }

    (max_char, max_score)
}

pub fn break_xor_char(model: &Histogram<u8>, charset: &[u8], string: &[u8]) -> (u8, Ratio<usize>) {
    max_char_with(model, charset, string)
}

fn guess_chunked_key_size(chunk_count: usize, keysizes: &[usize], buf: &[u8]) -> Option<usize> {
    let mut scores = Vec::new();

    for keysize in keysizes {
        let byte_count = chunk_count * keysize;
        
        if byte_count <= buf.len() {
            let chunks: Vec<&[u8]> = buf.chunks(*keysize)
                                        .take(chunk_count)
                                        .collect();
  
            let score = bitwiseops::mean_edit_distance(chunks.as_ref()).unwrap();
            
            scores.push((keysize, score))
        }
    }

    let best_key_size = *scores.into_iter()
                              .min_by_key(|&(_, score)| { score })
                              .unwrap_or((&0, Ratio::from_integer(0))).0;

    if best_key_size == 0 {
        None
    } else {
        Some(best_key_size)
    }

}

pub fn guess_key_size(keysizes: &[usize], buf: &[u8]) -> usize {
    guess_chunked_key_size(10, keysizes, buf).unwrap_or(0)
}

pub fn break_xor_key(model: &Histogram<u8>, charset: &[u8], keysize: usize, string: &[u8]) -> Vec<u8> {
    let chunks: Vec<Vec<u8>> = TransposeChunks::transpose(string, keysize).collect();
    
    let mut key = Vec::new();
    for chunk in chunks {
        let ch = break_xor_char(model, charset, chunk.as_ref()).0;
        key.push(ch);
    }

    key
}