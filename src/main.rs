use parse_enum::xxx::test;

#[macro_export]
macro_rules! xxx {
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

fn main() {
    test();
}