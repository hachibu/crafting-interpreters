macro_rules! impl_display_trait {
    ($type:ty) => (
        impl ::std::fmt::Display for $type {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{:#?}", self)
            }
        }
    )
}

pub mod scanner;
pub mod token;

pub use self::scanner::*;
pub use self::token::*;
