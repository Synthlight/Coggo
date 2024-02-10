macro_rules! non_zero_u64 {
    ($a:expr) => {
        match NonZeroU64::new($a) {
            Some(v) => v,
            #[allow(unconditional_panic)]
            None => [][0],
        }
    }
}
pub(crate) use non_zero_u64;