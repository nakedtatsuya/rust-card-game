pub fn is_shuffle<T: PartialEq>(v1: &[T], v2: &[T]) -> bool {
    !v1.starts_with(v2)
}
