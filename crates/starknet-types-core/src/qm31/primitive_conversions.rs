use crate::qm31::STWO_PRIME;

use super::QM31;

#[derive(Debug, Copy, Clone)]
pub struct PrimitiveFromQM31Error;

#[cfg(feature = "std")]
impl std::error::Error for PrimitiveFromQM31Error {}

impl core::fmt::Display for PrimitiveFromQM31Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Failed to convert `QM31` into primitive type")
    }
}

macro_rules! impl_from {
    ($from:ty, $with:ty) => {
        impl From<$from> for QM31 {
            fn from(value: $from) -> Self {
                (value as $with).into()
            }
        }
    };
}

impl_from!(u8, u128);
impl_from!(u16, u128);
impl_from!(u32, u128);
impl_from!(u64, u128);
impl_from!(usize, u128);
impl_from!(i8, i128);
impl_from!(i16, i128);
impl_from!(i32, i128);
impl_from!(i64, i128);
impl_from!(isize, i128);

impl From<u128> for QM31 {
    fn from(value: u128) -> QM31 {
        let mask_36 = (1 << 36) - 1;
        let mask_20 = (1 << 20) - 1;

        let limbs: [u64; 4] = [
            ((value >> 108) & mask_20) as u64,
            ((value >> 72) & mask_36) as u64,
            ((value >> 36) & mask_36) as u64,
            (value & mask_36) as u64,
        ];

        Self(limbs)
    }
}

impl From<i128> for QM31 {
    fn from(value: i128) -> QM31 {
        let unsigned_128 = value.abs() as u128;
        dbg!(unsigned_128.to_be_bytes());
        let qm31 = QM31::from(unsigned_128);

        if value.is_negative() {
            qm31.neg()
        } else {
            qm31
        }
    }
}

macro_rules! try_from_qm31_to_unsigned {
    ($into: ty) => {
        impl TryFrom<QM31> for $into {
            type Error = PrimitiveFromQM31Error;

            fn try_from(value: QM31) -> Result<Self, Self::Error> {
                let bytes = value.to_bytes_be();

                let (bytes_check, bytes_return) =
                    bytes.split_at(18 - core::mem::size_of::<$into>());

                // A QM31 follows a big-endian ordering. Since it can be represented with 18 bytes (144 bits), we
                // need to check that the first size_of::<QM31> - size_of::<$into> bytes are zero.
                if bytes_check.iter().all(|&b| b == 0) {
                    Ok(<$into>::from_be_bytes(bytes_return.try_into().unwrap()))
                } else {
                    Err(PrimitiveFromQM31Error)
                }
            }
        }
    };
}

try_from_qm31_to_unsigned!(u8);
try_from_qm31_to_unsigned!(u16);
try_from_qm31_to_unsigned!(u32);
try_from_qm31_to_unsigned!(u64);
try_from_qm31_to_unsigned!(u128);
try_from_qm31_to_unsigned!(usize);

impl TryFrom<QM31> for i128 {
    type Error = PrimitiveFromQM31Error;

    fn try_from(value: QM31) -> Result<Self, Self::Error> {
        // This convertion can be splited into three cases:
        // QM31 >= 0
        // QM31 < -1
        // QM31 == -1
        let bytes_be = value.to_bytes_be();
        let most_significant_byte = 18 - core::mem::size_of::<i128>();
        dbg!(bytes_be);
        // Case 1: QM31 >= 0
        // Out QM31 holds values up to 144 bits, encoded in 18 bytes, following a big-endian ordering.
        // Our target type can hold a size `size_of_type` (= { 1, 2, 4, 8, 16 }) bytes.
        // Since it is encoded with the big-endian notation, the most significant byte will be 18 - `size_of_type` + 1.
        // For a value to be positive, it should have its first 18 - `size_of_type` bytes equal to zero and MSB byte less
        // or equal to 0b0111111.
        // Remember we operating with signed values, so the MSB bit is used for the sign.
        if bytes_be[..most_significant_byte].iter().all(|&v| v == 0)
            && bytes_be[most_significant_byte] <= 0b01111111
        {
            Ok(i128::from_be_bytes(
                bytes_be[most_significant_byte..].try_into().unwrap(),
            ))
        } else if bytes_be[..most_significant_byte]
            == BIGGER_NEGATIVE_ONE_REPR[..most_significant_byte]
            && bytes_be[most_significant_byte] >= 0b10000000
        {
            let value_plus_one =
                i128::from_be_bytes(bytes_be[most_significant_byte..].try_into().unwrap());

            value_plus_one.checked_sub(1).ok_or(PrimitiveFromQM31Error)
        }
        // // else if {}
        else {
            Err(PrimitiveFromQM31Error)
        }
    }
}

const BIGGER_NEGATIVE_ONE_REPR: [u8; 18] = [
    7, 255, 255, 255, 240, 127, 255, 255, 255, 7, 255, 255, 255, 240, 127, 255, 255, 128,
];

macro_rules! impl_try_felt_into_signed {
    ($into: ty) => {
        impl TryFrom<QM31> for $into {
            type Error = PrimitiveFromQM31Error;

            fn try_from(value: QM31) -> Result<Self, Self::Error> {
                // This convertion can be splited into three cases:
                // QM31 >= 0
                // QM31 < -1
                // QM31 == -1
                let bytes_be = value.to_bytes_be();
                let most_significant_byte = 18 - core::mem::size_of::<$into>();
                dbg!(bytes_be);
                // Case 1: QM31 >= 0
                // Out QM31 holds values up to 144 bits, encoded in 18 bytes, following a big-endian ordering.
                // Our target type can hold a size `size_of_type` (= { 1, 2, 4, 8, 16 }) bytes.
                // Since it is encoded with the big-endian notation, the most significant byte will be 18 - `size_of_type` + 1.
                // For a value to be positive, it should have its first 18 - `size_of_type` bytes equal to zero and MSB byte less
                // or equal to 0b0111111.
                // Remember we operating with signed values, so the MSB bit is used for the sign.
                if bytes_be[..most_significant_byte].iter().all(|&v| v == 0)
                    && bytes_be[most_significant_byte] <= 0b01111111
                {
                    Ok(<$into>::from_be_bytes(
                        bytes_be[most_significant_byte..].try_into().unwrap(),
                    ))
                } else if bytes_be[..most_significant_byte]
                    == BIGGER_NEGATIVE_ONE_REPR[..most_significant_byte]
                    && bytes_be[most_significant_byte] >= 0b10000000
                {
                    let value_plus_one = <$into>::from_be_bytes(
                        bytes_be[most_significant_byte..].try_into().unwrap(),
                    );

                    value_plus_one.checked_add(1).ok_or(PrimitiveFromQM31Error)
                }
                // // else if {}
                else {
                    Err(PrimitiveFromQM31Error)
                }
            }
        }
    };
}

impl_try_felt_into_signed!(i8);
impl_try_felt_into_signed!(i16);
impl_try_felt_into_signed!(i32);
impl_try_felt_into_signed!(i64);
// impl_try_felt_into_signed!(i128);
impl_try_felt_into_signed!(isize);

#[cfg(test)]
mod tests {
    use core::{u128, u8};

    use crate::qm31::QM31;

    #[test]
    fn test_qm31_to_unsigned_values_to_qm31_valid_values() {
        let u8_max = u8::MAX;
        assert_eq!(u8_max, u8::try_from(QM31::from(u8_max)).unwrap());
        let u8_zero: u8 = 0;
        assert_eq!(u8_zero, u8::try_from(QM31::from(u8_zero)).unwrap());

        let u16_max = u16::MAX;
        assert_eq!(u16_max, u16::try_from(QM31::from(u16_max)).unwrap());
        let u16_zero: u16 = 0;
        assert_eq!(u16_zero, u16::try_from(QM31::from(u16_zero)).unwrap());

        let u32_max = u32::MAX;
        assert_eq!(u32_max, u32::try_from(QM31::from(u32_max)).unwrap());
        let u32_zero: u32 = 0;
        assert_eq!(u32_zero, u32::try_from(QM31::from(u32_zero)).unwrap());

        let u64_max = u64::MAX;
        assert_eq!(u64_max, u64::try_from(QM31::from(u64_max)).unwrap());
        let u64_zero: u64 = 0;
        assert_eq!(u64_zero, u64::try_from(QM31::from(u64_zero)).unwrap());

        let u128_max = u128::MAX;
        assert_eq!(u128_max, u128::try_from(QM31::from(u128_max)).unwrap());
        let u128_zero: u128 = 0;
        assert_eq!(u128_zero, u128::try_from(QM31::from(u128_zero)).unwrap());

        let usize_max_value = usize::MAX;
        assert_eq!(
            usize_max_value,
            usize::try_from(QM31::from(usize_max_value)).unwrap()
        );
        let usize_zero_value = 0usize;
        assert_eq!(
            usize_zero_value,
            usize::try_from(QM31::from(usize_zero_value)).unwrap()
        );
    }

    #[test]
    fn test_qm31_to_signed_values_to_qm31_valid_values() {
        let i8_max = i8::MAX;
        assert_eq!(i8_max, i8::try_from(QM31::from(i8_max)).unwrap());
        assert_eq!(-1i8, i8::try_from(QM31::from(-1i8)).unwrap());
        let i8_zero: i8 = 0;
        assert_eq!(i8_zero, i8::try_from(QM31::from(i8_zero)).unwrap());

        let i16_max = i16::MAX;
        assert_eq!(i16_max, i16::try_from(QM31::from(i16_max)).unwrap());
        assert_eq!(-i16_max, i16::try_from(QM31::from(-i16_max)).unwrap());
        let i16_zero: i16 = 0;
        assert_eq!(i16_zero, i16::try_from(QM31::from(i16_zero)).unwrap());

        // let i32_max = i32::MAX;
        // assert_eq!(i32_max, i32::try_from(QM31::from(i32_max)).unwrap());
        // assert_eq!(-i32_max, i32::try_from(QM31::from(-i32_max)).unwrap());
        // let i32_zero: i32 = 0;
        // assert_eq!(i32_zero, i32::try_from(QM31::from(i32_zero)).unwrap());

        // let i64_max = i64::MAX;
        // assert_eq!(i64_max, i64::try_from(QM31::from(i64_max)).unwrap());
        // assert_eq!(-i64_max, i64::try_from(QM31::from(-i64_max)).unwrap());
        // let i64_zero: i64 = 0;
        // assert_eq!(i64_zero, i64::try_from(QM31::from(i64_zero)).unwrap());

        let i128_max = i128::MAX;
        assert_eq!(i128_max, i128::try_from(QM31::from(i128_max)).unwrap());
        assert_eq!(-i128_max, i128::try_from(QM31::from(-i128_max)).unwrap());
        let i128_zero: i128 = 0;
        assert_eq!(i128_zero, i128::try_from(QM31::from(i128_zero)).unwrap());

        let isize_max_value = isize::MAX;
        assert_eq!(
            isize_max_value,
            isize::try_from(QM31::from(isize_max_value)).unwrap()
        );
        assert_eq!(
            -isize_max_value,
            isize::try_from(QM31::from(-isize_max_value)).unwrap()
        );
        let isize_zero_value: isize = 0;
        assert_eq!(
            isize_zero_value,
            isize::try_from(QM31::from(isize_zero_value)).unwrap()
        );
    }

    #[test]
    fn qm31_to_unsigned_values_out_of_bounds() {
        let qm31_u128_max = QM31::from(u128::MAX);
        assert!(u8::try_from(qm31_u128_max).is_err());
        assert!(u16::try_from(qm31_u128_max).is_err());
        assert!(u32::try_from(qm31_u128_max).is_err());
        assert!(u64::try_from(qm31_u128_max).is_err());
        assert!(usize::try_from(qm31_u128_max).is_err());
    }
}
