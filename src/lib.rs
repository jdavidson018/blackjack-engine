pub mod card;
mod shoe;
mod player;
mod hand;

pub fn add_one(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_one(2, 2);
        assert_eq!(result, 4);
    }
}
