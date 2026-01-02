macro_rules! define_options {
    (
        brief: $brief:literal
        flags: $(
            $name:ident, $short:literal, $long:literal, $desc:literal
        )*
    ) => {
        paste::paste!{
            pub mod options {
                use getopts::{Fail, Options as Opts, Matches};
                use std::ffi::OsStr;

                pub struct FlagOptionDef {
                    pub short_name: &'static str,
                    pub long_name: &'static str,
                    pub desc: &'static str,
                }

                $(pub const [<FLAG_DEF_ $name:upper>]: FlagOptionDef = FlagOptionDef {
                    short_name: $short,
                    long_name: $long,
                    desc: $desc,
                };)*

                pub const ALL_FLAG_OPTIONS: &[FlagOptionDef] = &[$([<FLAG_DEF_ $name:upper>]),*];

                pub struct FlagOptions {
                    $(pub $name: bool,)*
                }

                impl From<&Matches> for FlagOptions {
                    fn from(parsed: &Matches) -> Self {
                        Self {
                            $($name: parsed.opt_present($long),)*
                        }
                    }
                }

                pub struct Options {
                    source: Opts,
                    brief: String,
                    pub flags: FlagOptions,
                    pub free: Vec<String>,
                }

                impl Options {
                    pub fn parse<C>(args: C) -> Result<Options, Fail>
                    where
                        C: IntoIterator,
                        C::Item: AsRef<OsStr>,
                    {
                        let mut opts = Opts::new();

                        for flag_def in ALL_FLAG_OPTIONS {
                            opts.optflag(flag_def.short_name, flag_def.long_name, flag_def.desc);
                        }

                        opts.parse(args).map(|m| Self {
                            source: opts,
                            brief: $brief.to_string(),

                            flags: FlagOptions::from(&m),
                            free: m.free,
                        })
                    }

                    pub fn usage(&self) -> String {
                        self.source.usage(&self.brief)
                    }
                }
            }
        }
    };
}
pub(super) use define_options;
