#[derive(Debug)]
pub struct Wrap<T>(pub T);

impl<T> Wrap<T> {
    pub fn new(input: T) -> Self {
        Wrap(input)
    }
}

#[macro_export]
macro_rules! x {
    (@field #[unsafe] $field:ident: $field_ty:ty) => {
        $crate::Unsafe<Self, $field_ty, {$crate::macro_util::hash_field_name(stringify!($field))}>
    };
    (@field $_field:ident: $field_ty:ty) => {
        $field_ty
    };

    // rest is empty, terminate the recurse and output final forms
    {
        $(#[$outer_attr:meta])*
        $vis:vis
        $E:ident
        (
            $(($(#[$attr:meta])* $variant:ident $($fields:tt)*))*
        )
        (
            (@tmp)
        )
    } => {
        $(#[$outer_attr])*
        $vis enum $E {
            $(
                $(#[$attr])* $variant $($fields)*
            ),*
        }
    };

    // matches `#[unsafe]` attribute, high priority
    // attributes before `#[my_marker]` is saved in the `(@tmp ...)` group
    // save it to the output group and recurse
    {
        $(#[$outer_attr:meta])*
        $vis:vis
        $E:ident
        (
            $(($(#[$attr:meta])* $variant:ident $($fields:tt)*))*
        )
        (
            (@tmp $(#[$before:meta])*)
            (#[unsafe] $(#[$after:meta])* $next_variant:ident $($next_fields:tt)*)
            $($rest:tt)*
        )
    } => {
        x! {
            $(#[$outer_attr])*
            $vis
            $E
            (
                $(($(#[$attr])* $variant $($fields)*))*
                ($(#[$before])* $(#[$after])* $next_variant (Unsafe<Self, $($next_fields)*, {$crate::xxx::macro_util::hash_field_name(stringify!($next_variant))}>))
            )
            (
                (@tmp)
                $($rest)*
            )
        }
    };
    // capture (consume) a single attribute that is not `#[my_marker]`
    // note the attributes after the first one must be repetition of `tt`s
    {
        $(#[$outer_attr:meta])*
        $vis:vis
        $E:ident
        (
            $(($(#[$attr:meta])* $variant:ident $($fields:tt)*))*
        )
        (
            (@tmp $(#[$before:meta])*)
            (#[$not_marker:meta] $(#[$($after:tt)*])* $next_variant:ident $($next_fields:tt)*)
            $($rest:tt)*
        )
    } => {
        x! {
            $(#[$outer_attr])*
            $vis
            $E
            (
                $(($(#[$attr])* $variant $($fields)*))*
            )
            (
                (@tmp $(#[$before])* #[$not_marker])
                ($(#[$($after)*])* $next_variant $($next_fields)*)
                $($rest)*
            )
        }
    };
    // consumed all attributes for current variant, no match
    // save it to the output group and recurse
    {
        $(#[$outer_attr:meta])*
        $vis:vis
        $E:ident
        (
            $(($(#[$attr:meta])* $variant:ident $($fields:tt)*))*
        )
        (
            (@tmp $(#[$before:meta])*)
            ($next_variant:ident $($next_fields:tt)*)
            $($rest:tt)*
        )
    } => {
        x! {
            $(#[$outer_attr])*
            $vis
            $E
            (
                $(($(#[$attr])* $variant $($fields)*))*
                ($(#[$before])* $next_variant $($next_fields)*)
            )
            (
                (@tmp)
                $($rest)*
            )

        }
    };
    
}

#[macro_export]
macro_rules! xx {
    {
        $(#[$outer_attr:meta])*
        $vis:vis enum $E:ident {
            $(
                $(#[$($attr:tt)*])* 
                $variant:ident 
                $(($($fields:tt)*))?
                $({$($struct_fields:tt)*})?
            ),* $(,)?
        }
    } => {
        x! {
            $(#[$outer_attr])*
            $vis $E
            ()
            (
                (@tmp)
                $(
                    (
                        $(#[$($attr)*])* 
                        $variant 
                        $(($($fields)*))?
                        $({$($struct_fields)*})?
                    )
                )*
            )
        }
    }
}

xx! {
    /// this is an outer doc comment
    pub enum Test {
        A,
        /// this is an inner doc comment
        #[unsafe]
        /// another doc comment
        B (i32, String),
        C,
        D (i32, i32),
        E {
            a: i32,
            b: String,
        }
    }
}


pub fn test() {
    let _a = Test::B(unsafe{ Unsafe::new((1,"abc".to_string())) });
    let _b = Test::A;
    let _c = Test::D(0, 1);
    // println!("{:#?}", _a);
    // println!("{:#?}", _b);
    // println!("{:#?}", _c);
}


use core::marker::PhantomData;

pub struct Unsafe<O: ?Sized, F: ?Sized, const NAME_HASH: u128> {
    _marker: PhantomData<O>,
    field: F,
}

impl<O: ?Sized, F: Copy, const NAME_HASH: u128> Copy for Unsafe<O, F, { NAME_HASH }> {}
impl<O: ?Sized, F: Copy, const NAME_HASH: u128> Clone for Unsafe<O, F, { NAME_HASH }> {
    #[inline(always)]
    fn clone(&self) -> Self {
        Unsafe { _marker: PhantomData, field: self.field }
    }
}

impl<O: ?Sized, F, const NAME_HASH: u128> Unsafe<O, F, { NAME_HASH }> {
    #[inline(always)]
    pub const unsafe fn new(field: F) -> Unsafe<O, F, { NAME_HASH }> {
        Unsafe { _marker: PhantomData, field }
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
