//! # Art
//!
//! A library for modeling artistic concepts.
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;


pub mod kinds {
    /// The primary colors according to the RYB color model.
    #[derive(Eq, PartialEq,Debug)]
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }
    #[derive(Eq, PartialEq,Debug)]
    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create a secondary color.
    pub fn mix(c1:PrimaryColor, c2:PrimaryColor) -> SecondaryColor {
        if c1 == PrimaryColor::Red && c2 == PrimaryColor::Yellow {
            return SecondaryColor::Orange;
        } else if c1 == PrimaryColor::Red && c2 == PrimaryColor::Blue {
            return SecondaryColor::Purple;
        } else {
            return SecondaryColor::Green;
        }
    }
}