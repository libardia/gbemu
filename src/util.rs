pub fn either<T>(cond: bool, true_value: T, false_value: T) -> T {
    if cond {
        true_value
    } else {
        false_value
    }
}
