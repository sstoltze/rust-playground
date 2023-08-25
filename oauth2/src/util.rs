use rand::Rng;

const TOKEN_CHARSET: &[u8] = b"01234567890ABCDEF";

pub fn generate_random_string(string_length: usize) -> String {
    let mut rng = rand::thread_rng();

    let token: String = (0..string_length)
        .map(|_| {
            let idx = rng.gen_range(0..TOKEN_CHARSET.len());
            TOKEN_CHARSET[idx] as char
        })
        .collect();

    token
}
