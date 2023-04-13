#[inline]
pub fn align(value: usize, align_to: usize) -> usize {
    (value + align_to - 1) & !(align_to - 1)
}
