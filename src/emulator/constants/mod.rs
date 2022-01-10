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
pub const ROM_BANK_SIZE: usize = 0x4000;
pub const EXTERNAL_RAM_SIZE: usize = 0x2000;
pub const CARTRIDGE_HEADER_START: u32 = 0x0100;
pub const CARTRIDGE_HEADER_END: u32 = 0x014F;
pub const ENTRY_POINT: u32 = 0x0100;

//Cartridge constants
pub const NINTENDO_LOGO: [u8; 48] = [0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E];
pub const LOGO_START: usize = 0x0104;
pub const LOGO_END: usize = 0x0133;
pub const TITLE_START: usize = 0x0134;
pub const TITLE_END: usize = 0x0143;
pub const MANUFACTURER_START: usize = 0x013F;
pub const MANUFACTURER_END: usize = 0x0142;
pub const CGB_FLAG: usize = 0x0143;
pub const LICENSEE_START: usize = 0x0144;
pub const LICENSEE_END: usize = 0x0145;
pub const SGB_FLAG: usize = 0x0146;
pub const CARTRIDGE_TYPE: usize = 0x0147;
pub const ROM_SIZE: usize = 0x0148;
pub const RAM_SIZE: usize = 0x0149;
pub const DESTINATION_CODE: usize = 0x014A;
pub const OLD_LICENSEE_CODE: usize = 0x014B;
pub const MASK_ROM_VERSION: usize = 0x014C;
pub const HEADER_CHECKSUM: usize = 0x014D;
pub const GLOBAL_CHECKSUM_START: usize = 0x014E;
pub const GLOBAL_CHECKSUM_END: usize = 0x014F;

//Memory Map
pub const ONBOARD_ROM_START: usize = 0x000;
pub const ONBOARD_ROM_END: usize = 0x3FFF;
pub const SWITCHABLE_ROM_START: usize = 0x4000;
pub const SWITCHABLE_ROM_END: usize = 0x7FFF;
pub const SWITCHABLE_VRAM_START: usize = 0x8000;
pub const SWITCHABLE_VRAM_END: usize = 0x9FFF;
pub const EXTERNAL_RAM_START: usize = 0xA000;
pub const EXTERNAL_RAM_END: usize = 0xBFFF;
pub const ONBOARD_WRAM_START: usize = 0xC000;
pub const ONBOARD_WRAM_END: usize = 0xCFFF;
pub const SWITCHABLE_WRAM_START: usize = 0xD000;
pub const SWITCHABLE_WRAM_END: usize = 0xDFFF;
pub const ECHO_RAM_LOW_START: usize = 0xE000;
pub const ECHO_RAM_LOW_END: usize = 0xFDFF;
pub const ECHO_RAM_HIGH_START: usize = 0xF000;
pub const ECHO_RAM_HIGH_END: usize = 0xFDFF;
pub const OAM_START: usize = 0xFE00;
pub const OAM_END: usize = 0xFE9F;
pub const IO_REG_START: usize = 0xFF00;
pub const IO_REG_END: usize = 0xFF7F;
pub const HRAM_START: usize = 0xFF80;
pub const HRAM_END: usize = 0xFFFE;
pub const IE_REGISTER: usize = 0xFFFF;

//Size Constant
pub const FOUR_KB: usize = 4096;
pub const EIGHT_KB: usize = 8192;
pub const SIXTEEN_KB: usize = 16384;

//Initial Register Values
pub const DMG_AF: u16 = 0x01B0;
pub const DMG_BC: u16 = 0x0013;
pub const DMG_DE: u16 = 0x00D8;
pub const DMG_HL: u16 = 0x014D;
pub const DMG_SP: u16 = 0xFFFE;
pub const DMG_PC: u16 = 0x0100;
pub const DMG_DIV: u16 = 0xABCC;
pub const GBC_AF: u16 = 0x1180;
pub const GBC_BC: u16 = 0x0000;
pub const GBC_DE: u16 = 0xFF56;
pub const GBC_HL: u16 = 0x000D;
pub const GBC_SP: u16 = 0xFFFE;
pub const GBC_PC: u16 = 0x0100;
pub const GBC_DIV: u16 = 0x1EA0;

//Interrupt Vectors
pub const INT_VBL: u16 = 0x0040;
pub const INT_STAT: u16 = 0x0048;
pub const INT_TIMER: u16 = 0x0050;
pub const INT_SERIAL: u16 = 0x0058;
pub const INT_JOYPAD: u16 = 0x0060;