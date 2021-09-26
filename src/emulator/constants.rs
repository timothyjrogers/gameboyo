//Screen constants
pub const SCREEN_X_DIM: u32 = 160; //unit = pixels
pub const SCREEN_Y_DIM: u32 = 144; //unit = pixels

//Bit masks
pub const SET_ZERO_FLAG_MASK: u16 = 0b0000000010000000;
pub const UNSET_ZERO_FLAG_MASK: u16 = 0b1111111101111111;
pub const SET_SUBTRACTION_FLAG_MASK: u16 = 0b0000000001000000;
pub const UNSET_SUBTRACTION_FLAG_MASK: u16 = 0b1111111110111111;
pub const SET_HALFCARRY_FLAG_MASK: u16 = 0b0000000000100000;
pub const UNSET_HALFCARRY_FLAG_MASK: u16 = 0b1111111111011111;
pub const SET_CARRY_FLAG_MASK: u16 = 0b0000000000010000;
pub const UNSET_CARRY_FLAG_MASK: u16 = 0b1111111111101111;

//Memory constants
pub const ROM_BANK_SIZE: u32 = 0x4000;
pub const EXTERNAL_RAM_SIZE: u32 = 0x2000;
pub const CARTRIDGE_HEADER_START: u32 = 0x0100;
pub const CARTRIDGE_HEADER_END: u32 = 0x014F;
pub const ENTRY_POINT: u32 = 0x0100;

//Cartridge constants
pub const NINTENDO_LOGO: Vec<&str> = vec!["CE", "ED", "66", "66", "CC", "0D", "00", "0B", "03", "73", "00", "83", "00", "0C", "00", "0D", "00", "08", "11", "1F", "88", "89", "00", "0E", "DC", "CC", "6E", "E6", "DD", "DD", "D9", "99", "BB", "BB", "67", "63", "6E", "0E", "EC", "CC", "DD", "DC", "99", "9F", "BB", "B9", "33", "3E"];
pub const TITLE_START: u32 = 0x0134;
pub const TITLE_END: u32 = 0x0143;
pub const MANUFACTURER_START: u32 = 0x013F;
pub const MANUFACTURER_END: u32 = 0x0142;
pub const CGB_FLAG: u32 = 0x0143;
pub const LICENSEE_START: u32 = 0x0144;
pub const LICENSEE_END: u32 = 0x0145;
pub const SGB_FLAG: u32 = 0x0146;
pub const CARTRIDGE_TYPE: u32 = 0x0147;
pub const ROM_SIZE: u32 = 0x0148;
pub const RAM_SIZE: u32 = 0x0149;
pub const DESTINATION_CODE: u32 = 0x014A;
pub const OLD_LICENSEE_CODE: u32 = 0x014B;
pub const MASK_ROM_VERSION: u32 = 0x014C;
pub const HEADER_CHECKSUM: u32 = 0x014D;
pub const GLOBAL_CHECKSUM_START: u32 = 0x014E;
pub const GLOBAL_CHECKSUM_END: u32 = 0x014F;