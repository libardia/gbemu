use crate::gb::mmu::region::MemoryRegion;

macro_rules! definitions {
    ($(#$name:ident $begin:literal .. $end:literal)*) => {$(
        pub const $name: MemoryRegion = MemoryRegion::new($begin, $end);
    )*};

    ($($name:ident $value:literal)*) => {$(
        pub const $name: u16 = $value;
    )*}
}

definitions! {
    #ENTRY_POINT      0x100 .. 0x103
    #NINTENDO_LOGO    0x104 .. 0x133
    #TITLE            0x134 .. 0x143
    #MANUFACTURER     0x13F .. 0x142
    #NEW_LICENSEE     0x144 .. 0x145
}

definitions! {
    CGB_FLAG           0x143
    SGB_FLAG           0x146
    CART_TYPE          0x147
    ROM_SIZE           0x148
    RAM_SIZE           0x149
    DESTINATION        0x14A
    OLD_LICENSEE       0x14B
    VERSION            0x14C
    HEADER_CHECKSUM    0x14D
    GLOBAL_CHECKSUM_H  0x14E
    GLOBAL_CHECKSUM_L  0x14F
}
