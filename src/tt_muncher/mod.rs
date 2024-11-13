
#[macro_export]
macro_rules! test_other_enum {
    // entry point
    (
        $( #[$meta:meta] )*
        $vis:vis enum $name:ident {
            $($tt:tt)*
        }
    ) => {
        $( #[$meta] )*
        $vis enum $name {
            test_other_enum!(@variant $($tt)*)
        }
    };

    // tuple variant
    (@variant $(#[$variant_meta:meta])? $variant:ident $variant_ty:ty $(, $($tt:tt)* )? ) => {
        $(#[$variant_meta:meta])? $variant $variant_ty
        $( enum_item_matcher!(@variant $( $tt )*) )?
    };
    // named variant
    (@variant $variant:ident {
        $(
            $( #[$field_meta:meta] )*
            $field_vis:vis $field_name:ident : $field_ty:ty
        ),* $(,)?
    } $(, $($tt:tt)* )? ) => {
        $variant {
            $(
                $( #[$field_meta] )*
                $field_name : $field_ty,
            )*
        }
        $( enum_item_matcher!(@variant $( $tt )*) )?
    };
    // unit variant
    (@variant $variant:ident $(, $($tt:tt)* )? ) => {
        $variant
        $( test_other_enum!(@variant $( $tt )*) )?
    };
    // trailing comma
    (@variant ,) => {};
    // base case
    (@variant) => {};
}

test_other_enum!{
    pub enum Test {
        A,
        B,
        C(i32, String),
        D {
            a: isize,
            b: String,
        }
    }
}

#[macro_export]
macro_rules! second_example {
    (@helpler #[unsafe] $field:ident || $field_ty:ty) => {
        $crate::wrap::Wrap<Self, $field_ty, {$crate::wrap::macro_util::hash_field_name(stringify!($field))}>
    };

    // VariantName
    (
        @metadata {$vis:vis enum $name:ident}
        @attribute [$(#[$meta:meta])*]
        @variants [
            $($variants:tt)*
        ]
        @parsing
            $VariantName:ident
            $(, $($input:tt)*)?
    ) => (second_example! {
        @metadata {$vis enum $name}
        @attribute [$(#[$meta])*]
        @variants [
            $($variants)*
            {
                $VariantName
            }
        ]
        @parsing
            $( $($input)* )?
    });

    // VariantName(...)
    (
        @metadata {$vis:vis enum $name:ident}
        @attribute [$(#[$meta:meta])*]
        @variants [
            $($variants:tt)*
        ]
        @parsing
            $(#[$variant_meta:meta])?
            $VariantName:ident $variant_ty:ty
            $(, $($input:tt)*)?
    ) => (second_example! {
        @metadata {$vis enum $name}
        @attribute [$(#[$meta])*]
        @variants [
            $($variants)*
            {
                $VariantName second_example!(@helpler $(#[$variant_meta])? $VariantName || $variant_ty)
            }
        ]
        @parsing
            $( $($input)* )?
    });

    // VariantName { ... }
    (
        @metadata {$vis:vis enum $name:ident}
        @attribute [$(#[$meta:meta])*]
        @variants [
            $($variants:tt)*
        ]
        @parsing
            $VariantName:ident { $($tt:tt)* }
            $(, $($input:tt)*)?
    ) => (second_example! {
        @metadata {$vis enum $name}
        @attribute [$(#[$meta])*]
        @variants [
            $($variants)*
            {
                $VariantName { $($tt)* }
            }
        ]
        @parsing
            $( $($input)* )?
    });

    // Done parsing, time to generate code:
    (
        @metadata {$vis:vis enum $name:ident}
        @attribute [$(#[$meta:meta])*]
        @variants [
            $(
                {
                    $VariantName:ident $($variant_assoc:tt)?
                }
            )*
        ]
        @parsing
            // Nothing left to parse
    ) => (
        $(#[$meta])*
        $vis enum $name {
            $(
                $VariantName $(
                    $variant_assoc
                )? ,
            )*
        }
    );

    // == ENTRY POINT ==
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $($input:tt)*
        }
    ) => (second_example! {
        @metadata {$vis enum $name}
        @attribute [$(#[$meta])*]
        // a sequence of brace-enclosed variants
        @variants []
        // remaining tokens to parse
        @parsing
            $($input)*
    });
}

second_example!{
    pub enum SecondTest {
        A,
        B,
        #[unsafe]
        C(i32, String),
        D {
            a: isize,
            b: String,
        }
    }
}

pub fn test() {
    let _a = SecondTest::D { a: 0, b: "s".to_string() };
}
