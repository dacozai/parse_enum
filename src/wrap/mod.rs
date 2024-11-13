
use core::marker::PhantomData;

pub struct Wrap<O: ?Sized, F: ?Sized, const NAME_HASH: u128> {
    _marker: PhantomData<O>,
    field: F,
}

impl<O: ?Sized, F: Copy, const NAME_HASH: u128> Copy for Wrap<O, F, { NAME_HASH }> {}
impl<O: ?Sized, F: Copy, const NAME_HASH: u128> Clone for Wrap<O, F, { NAME_HASH }> {
    #[inline(always)]
    fn clone(&self) -> Self {
        Wrap { _marker: PhantomData, field: self.field }
    }
}

impl<O: ?Sized, F, const NAME_HASH: u128> Wrap<O, F, { NAME_HASH }> {
    #[inline(always)]
    pub const fn new(field: F) -> Wrap<O, F, { NAME_HASH }> {
        Wrap { _marker: PhantomData, field }
    }
}


#[doc(hidden)]
pub mod macro_util {
    #[inline(always)]
    #[must_use]
    #[allow(clippy::as_conversions, clippy::indexing_slicing, clippy::arithmetic_side_effects)]
    pub const fn hash_field_name(field_name: &str) -> u128 {
        let field_name = field_name.as_bytes();
        let mut hash = 0u128;
        let mut i = 0;
        while i < field_name.len() {
            const K: u128 = 0x517cc1b727220a95517cc1b727220a95;
            hash = (hash.rotate_left(5) ^ (field_name[i] as u128)).wrapping_mul(K);
            i += 1;
        }
        hash
    }
}
