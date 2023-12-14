use std::fmt;

#[derive(Clone)]
pub struct BigInteger {
    digits: Vec<BigDigit>,
}

#[derive(Clone)]
pub struct BigDigit {
    pub value: usize,
}

impl BigDigit {
    pub fn from(value: usize) -> BigDigit {
        return BigDigit { value };
    }

    pub fn zero() -> BigDigit {
        return BigDigit { value: 0usize };
    }

    pub fn one() -> BigDigit {
        return BigDigit { value: 1usize };
    }

    pub fn is_zero(&self) -> bool {
        return self.value == 0;
    }

    pub fn is_not_zero(&self) -> bool {
        return self.value != 0;
    }

    /**
     * Returns: (wrapped value, overflow)
     */
    pub fn add(&self, rhs: &BigDigit) -> (BigDigit, bool) {
        let (value, overflow) = self.value.overflowing_add(rhs.value);
        return (BigDigit::from(value), overflow);
    }

    /**
     * Returns: (wrapped value, overflow)
     */
    pub fn overflowing_add_assign(&mut self, rhs: &BigDigit) -> bool {
        let (value, overflow) = self.value.overflowing_add(rhs.value);
        self.value = value;
        return overflow;
    }
}

impl BigInteger {
    pub fn from(value: usize) -> BigInteger {
        return BigInteger {
            digits: vec![BigDigit::from(value)],
        };
    }

    pub fn zeroes(size: usize) -> BigInteger {
        return BigInteger {
            digits: vec![BigDigit::zero(); size],
        };
    }

    pub fn add(self, rhs: &BigInteger) -> BigInteger {
        // TODO: Make smaller side loop
        let length = std::cmp::max(self.digits.len(), rhs.digits.len());
        let mut result = BigInteger::zeroes(length);
        let mut carry = BigDigit::from(0);
        let one = BigDigit::one();

        for i in 0..length {
            let mut new_carry = BigDigit::zero();

            // Do the addition
            let left_digit_opt = self.digits.get(0);
            let right_digit_opt = rhs.digits.get(0);
            // No matching left
            if left_digit_opt.is_some() {
                let overflow = result.digits[i].overflowing_add_assign(left_digit_opt.unwrap());
                if overflow {
                    new_carry.overflowing_add_assign(&one);
                }
            }
            if right_digit_opt.is_some() {
                let overflow = result.digits[i].overflowing_add_assign(right_digit_opt.unwrap());
                if overflow {
                    new_carry.overflowing_add_assign(&one);
                }
            }
            let overflow = result.digits[i].overflowing_add_assign(&carry);
            if overflow {
                new_carry.overflowing_add_assign(&one);
            }
            
            carry = new_carry;
        }

        if carry.is_not_zero() {
            result.digits.push(carry);
        }

        return result;
    }
}

impl fmt::Display for BigInteger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (idx, digit) in self.digits.iter().enumerate().rev() {
            write!(f, "2^{} * {}", usize::BITS*(idx as u32), digit.value)?;
            if idx != 0 {write!(f, " + ")?;}
        };
        Ok(())
    }
}
