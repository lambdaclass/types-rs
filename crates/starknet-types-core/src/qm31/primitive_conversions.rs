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
// impl_from!(i8, i128);
// impl_from!(i16, i128);
// impl_from!(i32, i128);
// impl_from!(i64, i128);
// impl_from!(isize, i128);

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

macro_rules! try_from_qm31_to_unsigned {
    ($into: ty) => {
        impl TryFrom<QM31> for $into {
            type Error = PrimitiveFromQM31Error;

            fn try_from(value: QM31) -> Result<Self, Self::Error> {
                let bytes = value.to_le_bytes();

                let (bytes_return, bytes_check) = bytes.split_at(core::mem::size_of::<$into>());

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

#[cfg(test)]
mod tests {
    use core::{u128, u8};

    use crate::qm31::QM31;

    #[test]
    fn test_qm31_to_primitive_to_qm31_valid_values() {
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
    fn qm31_to_primitive_out_of_bounds() {
        let qm31_u128_max = QM31::from(u128::MAX);
        assert!(u8::try_from(qm31_u128_max).is_err());
        assert!(u16::try_from(qm31_u128_max).is_err());
        assert!(u32::try_from(qm31_u128_max).is_err());
        assert!(u64::try_from(qm31_u128_max).is_err());
        assert!(usize::try_from(qm31_u128_max).is_err());
    }
}
