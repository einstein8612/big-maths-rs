extern crate big_maths;

#[cfg(test)]
mod tests {

    use big_maths::*;

    #[test]
    fn it_works() {

        let lhs = BigDigit::from(usize::MAX);
        let rhs = BigDigit::from(10);

        println!("{}", lhs.add(&rhs).0.value);
    }

    #[test]
    fn test() {
        let lhs = BigInteger::from(usize::MAX);
        let rhs = BigInteger::from(usize::MAX);

        println!("{}", lhs.add(&rhs));
    }
}
