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
        $crate::wrap::Wrap<Self, $field_ty, {$crate::wrap::macro_util::hash_field_name(stringify!($field))}>
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
    // matches `#[my_marker]` attribute, high priority
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
            ($(#[$struct_meta:meta])* $next_variant:ident {
                $($(#[$field_attr:tt])? $field:ident: $field_ty:ty),* $(,)?
            })
            $($rest:tt)*
        )
    } => {
        x! {
            $(#[$outer_attr])*
            $vis
            $E
            (
                $(($(#[$attr])* $variant $($fields)*))*
                ($(#[$struct_meta])* $next_variant {
                    $(
                        $field: x!(@field $(#[$field_attr])? $field: $field_ty),
                    )*
                })
            )
            (
                (@tmp)
                $($rest)*
            )
        }
    };
    // matches `#[my_marker]` attribute, high priority
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
            (#[my_marker] $(#[$after:meta])* $next_variant:ident $($next_fields:tt)*)
            $($rest:tt)*
        )
    } => {
        x! {
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
                $(
                    ($($fields:tt)*)
                )?
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
                        $(
                            ($($fields)*)
                        )?
                    )
                )*
            )
        }
    }
}

xx! {
    /// this is an outer doc comment
    #[derive(Debug)]
    pub enum Test {
        A,
        /// this is an inner doc comment
        #[my_marker]
        /// another doc comment
        B (i32, i32),
        C,
    }
}

pub fn test() {
    let _a = Test::B(Wrap::new((1,2)));
    let _b = Test::A;
    println!("{:#?}", _a);
    println!("{:#?}", _b);
}