#[cfg(test)]
mod coin_bank_tests {
    use crate::CoinBank;

    const COIN_BANK: CoinBank = CoinBank { quarters: 2, dimes: 5, nickels: 10, pennies: 15 };

    #[test]
    fn test_add_quarters() {
        let additional_quarters = 4;
        let original_quarter_count = COIN_BANK.quarters;
        assert_eq!(COIN_BANK.add_quarters(additional_quarters).quarters,
                   original_quarter_count + additional_quarters);
    }

    #[test]
    fn test_add_dimes() {
        let additional_dimes = 4;
        let original_dime_count = COIN_BANK.dimes;
        assert_eq!(COIN_BANK.add_dimes(additional_dimes).dimes,
                   original_dime_count + additional_dimes);
    }

    #[test]
    fn test_add_nickels() {
        let additional_nickels = 4;
        let original_nickel_count = COIN_BANK.nickels;
        assert_eq!(COIN_BANK.add_nickels(additional_nickels).nickels,
                   original_nickel_count + additional_nickels);
    }

    #[test]
    fn test_add_pennies() {
        let additional_pennies = 4;
        let original_penny_count = COIN_BANK.pennies;
        assert_eq!(COIN_BANK.add_pennies(additional_pennies).pennies,
                   original_penny_count + additional_pennies);
    }
}
