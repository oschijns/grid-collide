//! Define the data layout used for each tile in the map

#[cfg(feature = "2d")]
pub(crate) mod tile {
    bitfield::bitfield! {
        /// Data layout specific to 2D grid map
        pub struct TileLayout(u16);
        impl Debug;
        u16;
        pub is_slope_bit    , set_slope_bit       :      0; // is the slope data relevant
        pub slope_direction , set_slope_direction :      1; // is the slope ascending or descending
        pub slope_side      , set_slope_side      :  3,  2; // on which side (bottom, left, top, right) to place the slope
        pub slope_level     , set_slope_level     :  5,  4; // associated with the `slope_length` value [0, 3]
        pub slope_length    , set_slope_length    :  7,  6; // length of the slope in tiles [0, 3] -> [1, 4] (inverse of the steepness)
            _               , _                   : 11,  8; // --- 4 bits padding ---
        pub collide_right   , set_collide_right   :     12; // collide on its right side
        pub collide_top     , set_collide_top     :     13; // collide on its top side
        pub collide_left    , set_collide_left    :     14; // collide on its left side
        pub collide_bottom  , set_collide_bottom  :     15; // collide on its bottom side
    }

    impl Default for TileLayout {
        fn default() -> Self {
            Self(0)
        }
    }

    impl TileLayout {
        #[inline]
        pub fn is_slope(&self) -> bool {
            self.is_slope_bit()
        }
    }
}

#[cfg(feature = "3d")]
pub(crate) mod tile {
    bitfield::bitfield! {
        /// Data layout specific to 3D grid map
        pub struct TileLayout(u16);
        impl Debug;
        u16;
        pub slope_side      , set_slope_side      :  2,  0; // on which side (bottom, left, front, top, right, back) to place the slope and is the slope data relevant [1, 6]
        pub slope_direction , set_slope_direction :  5,  3; // orientation of the slope [0, 7] (eightway)
        pub slope_level     , set_slope_level     :  7,  6; // associated with the `slope_length` value [0, 3]
        pub slope_length    , set_slope_length    :  9,  8; // length of the slope in tiles [0, 3] -> [1, 4] (inverse of the steepness)
        pub collide_back    , set_collide_back    :     10; // collide on its back side
        pub collide_right   , set_collide_right   :     11; // collide on its right side
        pub collide_top     , set_collide_top     :     12; // collide on its top side
        pub collide_front   , set_collide_front   :     13; // collide on its front side
        pub collide_left    , set_collide_left    :     14; // collide on its left side
        pub collide_bottom  , set_collide_bottom  :     15; // collide on its bottom side
    }

    impl Default for TileLayout {
        fn default() -> Self {
            Self(0)
        }
    }

    impl TileLayout {
        #[inline]
        pub fn is_slope(&self) -> bool {
            self.slope_side() != 0
        }
    }
}

/// re-export the tile layout bitfield
pub use tile::TileLayout;

/// Tile with arbitrary payload
pub struct Tile<P = u16> {
    /// Layout of the tile
    pub layout: TileLayout,

    /// Arbitrary payload
    pub payload: P,
}
