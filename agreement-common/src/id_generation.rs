use rand::distr::Alphanumeric;

pub fn agreement_id() -> String {
    use rand::Rng;
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(24)
        .map(char::from)
        .collect()
}
