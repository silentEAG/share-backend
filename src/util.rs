use rand::{distributions::Alphanumeric, Rng};

pub fn generate_unique_name(prefix: &str) -> String {
    format!(
        "{}-{}",
        prefix.to_ascii_lowercase(),
        generate_rand_str(20, true)
    )
}

pub fn generate_rand_str(len: usize, to_lowercase: bool) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .map(|c| match to_lowercase {
            true => c.to_ascii_lowercase(),
            false => c,
        })
        .collect::<String>()
}
