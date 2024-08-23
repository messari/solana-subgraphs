use crate::utils::idl_discriminator;

pub struct DiscriminatorConstants;

impl DiscriminatorConstants {
    // V1
    pub const INITIALIZE_POOL: [u8; 8] = [95, 180, 10, 172, 84, 174, 232, 40];

    pub const INCREASE_LIQUIDITY: [u8; 8] = [46, 156, 243, 118, 13, 205, 251, 178];
    pub const DECREASE_LIQUIDITY: [u8; 8] = [160, 38, 208, 111, 104, 91, 44, 1];

    pub const TWO_HOP_SWAP: [u8; 8] = [195, 96, 237, 108, 68, 162, 219, 230];
    pub const SWAP: [u8; 8] = [248, 198, 158, 145, 225, 117, 135, 200];

    // V2
    pub const INITIALIZE_POOL_V2: [u8; 8] = [207, 45, 87, 242, 27, 63, 204, 67];

    pub const DECREASE_LIQUIDITY_V2: [u8; 8] = [58, 127, 188, 62, 79, 82, 196, 96];
    pub const INCREASE_LIQUIDITY_V2: [u8; 8] = [133, 29, 89, 223, 69, 238, 176, 10];

    pub const TWO_HOP_SWAP_V2: [u8; 8] = [186, 143, 209, 29, 254, 2, 194, 117];
    pub const SWAP_V2: [u8; 8] = [43, 4, 237, 11, 26, 201, 30, 98];
}

pub const ORCA_WHIRLPOOL: &str = "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc";
