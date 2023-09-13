use serde::{Serialize, Deserialize};

/**
 * A collection of coins.
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct CoinBank {
    pub quarters: u16,
    pub dimes: u16,
    pub nickels: u16,
    pub pennies: u16,
}

impl CoinBank {
    /**
     * Returns the value of this CoinBank in cents.
     */
    pub fn cents(&self) -> u16 {
        (self.quarters * 25) + (self.dimes * 10) + (self.nickels * 5) + (self.pennies * 1)
    }

    /**
     * Converts the value of this CoinBank to dollars and cents.
     * The first value in the returned tuple is dollars.  The other value is
     * cents.
     */
    pub fn dollars_and_cents(&self) -> (u16, u8) {
        let cents = self.cents();
        let dollars = cents / 100;
        (dollars, (cents % 100) as u8)
    }

    /**
     * Adds quarters to this CoinBank, returning a new, updated CoinBank.
     */
    pub fn add_quarters(&self, quarters: u16) -> CoinBank {
        CoinBank { quarters: self.quarters + quarters, ..*self }
    }

    /**
     * Adds dimes to this CoinBank, returning a new, updated CoinBank.
     */
    pub fn add_dimes(&self, dimes: u16) -> CoinBank {
        CoinBank { dimes: self.dimes + dimes, ..*self }
    }

    /**
     * Adds nickels to this CoinBank, returning a new, updated CoinBank.
     */
    pub fn add_nickels(&self, nickels: u16) -> CoinBank {
        CoinBank { nickels: self.nickels + nickels, ..*self }
    }

    /**
     * Adds pennies to this CoinBank, returning a new, updated CoinBank.
     */
    pub fn add_pennies(&self, pennies: u16) -> CoinBank {
        CoinBank { pennies: self.pennies + pennies, ..*self }
    }
}
