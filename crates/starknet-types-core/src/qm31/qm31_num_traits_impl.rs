use super::QM31;
use num_bigint::{ToBigInt, ToBigUint};
use num_traits::{FromPrimitive, Inv, One, ToPrimitive, Zero};

impl ToBigInt for QM31 {
    /// Converts the value of `self` to a [`BigInt`].
    ///
    /// Safe to unwrap, will always return `Some`.
    fn to_bigint(&self) -> Option<num_bigint::BigInt> {
        Some(self.to_bigint())
    }
}

impl ToBigUint for QM31 {
    /// Converts the value of `self` to a [`BigUint`].
    ///
    /// Safe to unwrap, will always return `Some`.
    fn to_biguint(&self) -> Option<num_bigint::BigUint> {
        Some(self.to_biguint())
    }
}

impl FromPrimitive for QM31 {
    fn from_i64(value: i64) -> Option<Self> {
        todo!();
        // Some(value.into())
    }

    fn from_u64(value: u64) -> Option<Self> {
        Some(value.into())
    }

    fn from_i128(value: i128) -> Option<Self> {
        todo!();
        // Some(value.into())
    }

    fn from_u128(value: u128) -> Option<Self> {
        Some(value.into())
    }
}

// TODO: we need to decide whether we want conversions to signed primitives
// will support converting the high end of the field to negative.
impl ToPrimitive for QM31 {
    fn to_u64(&self) -> Option<u64> {
        self.to_u128().and_then(|x| u64::try_from(x).ok())
    }

    fn to_i64(&self) -> Option<i64> {
        self.to_u128().and_then(|x| i64::try_from(x).ok())
    }

    fn to_u128(&self) -> Option<u128> {
        match self.inner() {
            [l4, l3, l2, l1] if l4 <= 1 << 20 => Some(
                (l1 as u128) | ((l2 as u128) << 36) | ((l3 as u128) << 72) | ((l4 as u128) << 108),
            ),
            _ => None,
        }
    }

    fn to_i128(&self) -> Option<i128> {
        self.to_u128().and_then(|x| i128::try_from(x).ok())
    }
}

impl Zero for QM31 {
    fn is_zero(&self) -> bool {
        *self == QM31::ZERO
    }

    fn zero() -> QM31 {
        QM31::ZERO
    }
}

impl One for QM31 {
    fn one() -> Self {
        QM31::ONE
    }
}

impl Inv for QM31 {
    type Output = Option<Self>;

    fn inv(self) -> Self::Output {
        (&self).inverse().ok()
    }
}

#[cfg(test)]
mod tests {
    use core::u128;

    use num_bigint::{BigInt, BigUint};

    use super::*;

    #[test]
    fn zero_is_zero() {
        assert!(QM31::ZERO.is_zero());
    }

    #[test]
    fn one_is_one() {
        assert!(QM31::ONE.is_one());
    }

    #[test]
    fn default_is_zero() {
        assert!(QM31::default().is_zero());
    }

    #[test]
    fn qm31_to_primitive() {
        let qm31 = QM31::from(0u64);
        assert_eq!(qm31.to_u64().unwrap(), 0u64);
        let qm31 = QM31::from(10u64);
        assert_eq!(qm31.to_u64().unwrap(), 10u64);
        let qm31 = QM31::from(u64::MAX);
        assert_eq!(qm31.to_u64().unwrap(), u64::MAX);
        
        let qm31 = QM31::from(0u128);
        assert_eq!(qm31.to_u128().unwrap(), 0u128);
        let qm31 = QM31::from(10u128);
        dbg!(qm31);
        assert_eq!(qm31.to_u128().unwrap(), 10u128);
        let qm31 = QM31::from(u128::MAX);
        assert_eq!(qm31.to_u128().unwrap(), u128::MAX);

        let qm31 = QM31::from(0u64);
        assert_eq!(qm31.to_biguint(), BigUint::from(0u64));
        let qm31 = QM31::from(10u64);
        assert_eq!(qm31.to_biguint(), BigUint::from(10u64));
        let qm31 = QM31::from(u128::MAX);
        assert_eq!(qm31.to_biguint(), BigUint::from(u128::MAX));

        let qm31 = QM31::from(0u64);
        assert_eq!(qm31.to_bigint(), BigInt::from(0u64));
        let qm31 = QM31::from(10u64);
        assert_eq!(qm31.to_bigint(), BigInt::from(10u64));
        let qm31 = QM31::from(u128::MAX);
        assert_eq!(qm31.to_bigint(), BigInt::from(u128::MAX));
    }
}
