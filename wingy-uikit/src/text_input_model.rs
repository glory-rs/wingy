//! UIKit positions use UTF-16; Wingy preedit positions use UTF-8 bytes.
//! No Objective-C, host callbacks or native lifetime assumptions in this module.
pub(crate) fn utf16_to_byte(text: &str, offset: usize) -> Option<usize> {
    let mut units = 0;
    for (byte, character) in text.char_indices() {
        if units == offset {
            return Some(byte);
        }
        units += character.len_utf16();
        if units > offset {
            return None;
        }
    }
    (units == offset).then_some(text.len())
}
pub(crate) fn byte_to_utf16(text: &str, offset: usize) -> Option<usize> {
    text.get(..offset).map(|prefix| prefix.encode_utf16().count())
}
pub(crate) fn selected_bytes(text: &str, start: usize, length: usize) -> Option<(usize, usize)> {
    Some((utf16_to_byte(text, start)?, utf16_to_byte(text, start.checked_add(length)?)?))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ime_positions_preserve_utf8_and_utf16_boundaries() {
        let text = "a中😀e\u{301}";
        for byte in 0..=text.len() {
            if let Some(units) = byte_to_utf16(text, byte) {
                assert_eq!(utf16_to_byte(text, units), Some(byte));
            } else {
                assert!(!text.is_char_boundary(byte));
            }
        }
        assert_eq!(utf16_to_byte(text, 3), None); // inside the emoji surrogate pair
        assert_eq!(selected_bytes(text, 2, 2), Some((4, 8)));
        assert_eq!(selected_bytes(text, usize::MAX, 1), None);
        assert_eq!(selected_bytes(text, 3, 0), None);
        assert_eq!(selected_bytes(text, 20, 0), None);
    }
}
