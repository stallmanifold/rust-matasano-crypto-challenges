use bitwise::bitwiseops;
use challengeinfo::challenge::{Challenge, ChallengeInfo};


pub const INFO5: ChallengeInfo<'static> = ChallengeInfo {
    set_number: 5,
    challenge_number: 5,
    title: "Implement repeating-key XOR",
    description: "",
    url: "http://cryptopals.com/sets/1/challenges/5",
};

pub const CHALLENGE5: Challenge<'static> = Challenge {
    info: INFO5,
    func: execute,
};

fn execute() -> String {
    let plaintext  = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let key        = "ICE";
    let result     = bitwiseops::xor_with_key(key.as_ref(), plaintext.as_ref());

    String::from_utf8(result).unwrap()
}