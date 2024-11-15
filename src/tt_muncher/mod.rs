#[derive(Debug)]
pub struct Wrap<T>(pub T);

impl<T> Wrap<T> {
    pub fn new(input: T) -> Self {
        Wrap(input)
    }
}

#[macro_export]
macro_rules! zz {
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
        zz! {
            $(#[$outer_attr])*
            $vis
            $E
            (
                $(($(#[$attr])* $variant $($fields)*))*
                ($(#[$before])* $(#[$after])* $next_variant (Wrap<$($next_fields)*>))
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
        zz! {
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
        zz! {
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
macro_rules! zzz {
    {
        $(#[$outer_attr:meta])*
        $vis:vis enum $E:ident {
            $(
                $(#[$($attr:tt)*])* 
                $variant:ident 
                $(($($fields:tt)*))?
            ),* $(,)?
        }
    } => {
        zz! {
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
                    )
                )*
            )
        }
    }
}

zzz! {
    /// this is an outer doc comment
    pub enum Test {
        A,
        /// this is an inner doc comment
        #[unsafe]
        /// another doc comment
        B (i32, String),
        C,
        D (i32, i32),
    }
}
