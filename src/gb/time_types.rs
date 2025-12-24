use crate::util::{impls_debug_as_internal, make_number_type};

type TimeInternal = u64;

make_number_type!(MTime, TimeInternal);
impls_debug_as_internal!(MTime);
impl From<TTime> for MTime {
    fn from(value: TTime) -> Self {
        MTime(value.0 / 4)
    }
}

make_number_type!(TTime, TimeInternal);
impls_debug_as_internal!(TTime);
impl From<MTime> for TTime {
    fn from(value: MTime) -> Self {
        TTime(value.0 * 4)
    }
}
