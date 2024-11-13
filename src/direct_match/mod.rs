use crate::wrap::Wrap;

#[macro_export]
macro_rules! mymacro {
    (
        $( #[$attr:meta] )*
        $vis:vis enum $name:ident {
            $(
                $(#[$variant_attrs:meta])*
                $variant:ident $({
                    $(
                        $(#[$field_attr:ident])?
                        $field:ident: $field_ty:ty
                    ),+ $(,)?
                })?
                $((
                    $(
                        $field_ty2:ty
                    ),+ $(,)?
                ))?
            ),+ $(,)?
        }
    ) => {
        $(#[$attr])*
        $vis enum $name {
            $(
                $(#[$variant_attrs])*
                $variant $({
                    $(
                        $field: mymacro!(@field $(#[$field_attr])? $field: $field_ty),
                    )*
                })?
                $((
                    $(
                        $field_ty2,
                    )+
                ))?
            ),+
        }
    };

    (@field #[unsafe] $field:ident: $field_ty:ty) => {
        $crate::wrap::Wrap<Self, $field_ty, {$crate::wrap::macro_util::hash_field_name(stringify!($field))}>
    };
    (@field $_field:ident: $field_ty:ty) => {
        $field_ty
    }
}


#[macro_export]
macro_rules! mymacro_failed {
    (
        $( #[$attr:meta] )*
        $vis:vis enum $name:ident {
            $(
                $(#[$variant_attrs:meta])*
                $variant:ident $({
                    $(
                        $(#[$field_attr:ident])?
                        $field:ident: $field_ty:ty
                    ),+ $(,)?
                })?
                $($tuple_ty:ty)?
            ),+ $(,)?
        }
    ) => {
        $(#[$attr])*
        $vis enum $name {
            $(
                $(#[$variant_attrs])*
                $variant $({
                    $(
                        $field: mymacro!(@field $(#[$field_attr])? $field: $field_ty),
                    )*
                })?
                $($tuple_ty)?
            ),+
        }
    };

    (@field #[unsafe] $field:ident: $field_ty:ty) => {
        $crate::wrap::Wrap<Self, $field_ty, {$crate::wrap::macro_util::hash_field_name(stringify!($field))}>
    };
    (@field $_field:ident: $field_ty:ty) => {
        $field_ty
    }
}

mymacro! {
    pub enum Bar {
        Baz,
        Foo(i32, String),
        Bas {
            #[unsafe]
            a: i32,
            b: i64,
        }
    }
}
pub fn test() {
    let _a = Bar::Bas { a: Wrap::new(0), b: 1 };
    // >>> Bas { pub a: Wrap<Bar, i32, _>, pub b: i64, }
}
