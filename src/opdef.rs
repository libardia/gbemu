macro_rules! define_options {
    (
        brief($brief:literal)
        flags($(
            $f_name:ident, $f_short:literal, $f_long:literal, $f_desc:literal, $f_required:literal;
        )*)
        args($(
            $a_name:ident, $a_short:literal, $a_long:literal, $a_desc:literal, $a_required:literal;
                $(conv($a_conv_param:ident) -> $a_conv_target:ty $a_conv_block:block)?
        )*)
        strings($(
            $s_index:literal, $s_name:ident, $s_required:literal;
                $(conv($s_conv_param:ident) -> $s_conv_target:ty $s_conv_block:block)?
        )*)
    ) => {
        pub mod options {
            paste::paste!{
                use getopts::{Fail, Options as Opts, HasArg, Occur};
                use std::ffi::OsStr;

                $(
                    conversion_fn!(
                        [< $a_name _conv >] $(
                            conv($a_conv_param) -> $a_conv_target $a_conv_block
                        )?
                    );
                )*
                $(
                    conversion_fn!(
                        [< $s_name _conv >] $(
                            conv($s_conv_param) -> $s_conv_target $s_conv_block
                        )?
                    );
                )*

                pub struct Options {
                    source: Opts,
                    brief: String,

                    $(pub $f_name: bool,)*
                    $(pub $a_name: [< $a_name:camel ConvTarget >],)*
                    $(pub $s_name: [< $s_name:camel ConvTarget >],)*
                }

                impl Options {
                    pub fn parse<C>(args: C) -> Result<Options, Fail>
                    where
                        C: IntoIterator,
                        C::Item: AsRef<OsStr>,
                    {
                        let mut opts = Opts::new();

                        $(if $f_required {
                            opts.opt($f_short, $f_long, $f_desc, "", HasArg::No, Occur::Req);
                        } else {
                            opts.opt($f_short, $f_long, $f_desc, "", HasArg::No, Occur::Optional);
                        })*

                        $(if $a_required {
                            opts.opt($a_short, $a_long, $a_desc, "", HasArg::Yes, Occur::Req);
                        } else {
                            opts.opt($a_short, $a_long, $a_desc, "", HasArg::Yes, Occur::Optional);
                        })*

                        opts.parse(args).map(|m| Self {
                            source: opts,
                            brief: $brief.to_string(),

                            $($f_name: m.opt_present($f_long),)*
                            $($s_name: m.free[$s_index].clone(),)*
                        })
                    }

                    pub fn usage(&self) -> String {
                        self.source.usage(&self.brief)
                    }
                }
            }

            #[allow(unused_macros)]
            macro_rules! conversion_fn {
                ($name:ident conv($param:ident) -> $target:ty $block:block) => {
                    paste::paste!{
                        fn $name($param) -> $target $block
                        type [< $name:camel Target >] = $target;
                    }
                };
                ($name:ident) => {
                    paste::paste!{
                        fn $name(s: String) -> String { s }
                        type [< $name:camel Target >] = String;
                    }
                }
            }
            #[allow(unused_imports)]
            use conversion_fn;
        }
    };
}
pub(super) use define_options;
