pub struct SimpleOptionDef {
    pub short_name: &'static str,
    pub long_name: &'static str,
    pub desc: &'static str,
}

macro_rules! simple_options {
    ($($name:ident, $short:expr, $long:expr, $desc:expr;)*) => {
        $(pub const $name: SimpleOptionDef = SimpleOptionDef {
            short_name: $short,
            long_name: $long,
            desc: $desc,
        };)*

        pub const ALL_SIMPLE_OPTIONS: &[SimpleOptionDef] = &[$($name),*];
    };
}

simple_options!(
    META_INST, "m", "meta", "Enable meta-instructions.";
    DO_BOOT, "b", "do-boot", "If set, runs the boot ROM before cartridge ROM. Skips the boot ROM otherwise.";
);
