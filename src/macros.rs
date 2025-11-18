macro_rules! new {
    () => {
        pub fn new() -> Self {
            Self::default()
        }
    };
    ( $(-> $arg:ident: $argt:ty;)* $($field:ident = $value:expr;)* ) => {
        pub fn new($($arg: $argt),*) -> Self {
            Self {
                $($arg,)*
                $($field: $value,)*
                ..Self::default()
            }
        }
    };
}
pub(crate) use new;
