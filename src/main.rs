
pub struct Wrap<T>(pub T);

macro_rules! x {
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
    pub enum Test {
        A,
        /// this is an inner doc comment
        #[my_marker]
        /// another doc comment
        B (i32, i32),
        C,
    }
}

fn main() {
    let _a = Test::B(Wrap((0, 1)));
}