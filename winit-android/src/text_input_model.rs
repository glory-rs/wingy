//! Bounded GameTextInput state translation. Android spans are UTF-16; Winit IME
//! spans/deletions are UTF-8. Unsolicited native selection/correction is refused.
use std::ops::Range;

pub(crate) const MAX_NATIVE_BYTES: usize = 16384;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct NativeState {
    pub text: String,
    pub selection: (usize, usize),
    pub compose: Option<(usize, usize)>,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Edit {
    Preedit(String, (usize, usize)),
    Commit(String),
    Cancel,
    Delete { before: usize, after: usize },
}

#[derive(Debug, Default)]
pub(crate) struct TextInputModel {
    text: String,
    selection: (usize, usize), // ordered UTF-8 replacement span
    composing: bool,
    native: NativeState,
}

fn byte_at(text: &str, units: usize) -> Option<usize> {
    let mut offset = 0;
    for (byte, ch) in text.char_indices() {
        if offset == units {
            return Some(byte);
        }
        offset += ch.len_utf16();
        if offset > units {
            return None;
        }
    }
    (offset == units).then_some(text.len())
}

fn range_at(text: &str, range: (usize, usize)) -> Option<Range<usize>> {
    Some(byte_at(text, range.0.min(range.1))?..byte_at(text, range.0.max(range.1))?)
}

impl TextInputModel {
    pub fn composing(&self) -> bool {
        self.composing
    }
    pub fn native(&self) -> NativeState {
        self.native.clone()
    }

    pub fn set_context(&mut self, text: &str, cursor: usize, anchor: usize) -> Option<NativeState> {
        if self.composing
            || text.len() >= 4000
            || !text.is_char_boundary(cursor)
            || !text.is_char_boundary(anchor)
        {
            return None;
        }
        let state = NativeState {
            text: text.to_owned(),
            selection: (
                text[..anchor].encode_utf16().count(),
                text[..cursor].encode_utf16().count(),
            ),
            compose: None,
        };
        self.text.clear();
        self.text.push_str(text);
        self.selection = (cursor.min(anchor), cursor.max(anchor));
        let changed = state != self.native;
        self.native = state.clone();
        changed.then_some(state)
    }

    pub fn receive(&mut self, state: NativeState) -> Result<Vec<Edit>, ()> {
        // GameTextInput itself owns its buffer; bound all clones and events here.
        if state.text.len() > MAX_NATIVE_BYTES {
            return Err(());
        }
        let selected = range_at(&state.text, state.selection).ok_or(())?;
        let compose = state.compose.map(|r| range_at(&state.text, r).ok_or(())).transpose()?;
        if state == self.native {
            return Ok(Vec::new());
        }
        let (low, high) = self.selection;
        let prefix = &self.text[..low];
        let suffix = &self.text[high..];
        let replacement = if state.text.starts_with(prefix)
            && state.text.ends_with(suffix)
            && prefix.len() + suffix.len() <= state.text.len()
        {
            Some(low..state.text.len() - suffix.len())
        } else {
            None
        };
        let mut edits = Vec::new();
        if let Some(compose) = compose.filter(|range| !range.is_empty()) {
            let replaced = replacement.ok_or(())?;
            if compose != replaced || selected.start < compose.start || selected.end > compose.end {
                return Err(());
            }
            edits.push(Edit::Preedit(
                state.text[compose.clone()].to_owned(),
                (selected.start - compose.start, selected.end - compose.start),
            ));
            self.composing = true;
        } else if let Some(replaced) = replacement {
            let value = &state.text[replaced.clone()];
            if self.composing && (value.is_empty() || state.text == self.text) {
                // Clearing marked text cancels and restores an original selected
                // replacement in the app. A subsequent equal-value acknowledgement
                // must also restore the native store, even if its value differs.
                edits.push(Edit::Cancel);
                self.composing = false;
            } else if state.text == self.text {
                // Selection-only movement cannot be represented by Winit Ime.
                // Refuse it so the native store cannot drift from the app.
                return Err(());
            } else {
                if selected.start != replaced.end || selected.end != replaced.end {
                    return Err(());
                }
                edits.push(Edit::Commit(value.to_owned()));
                self.text.clone_from(&state.text);
                self.selection = (selected.start, selected.end);
                self.composing = false;
            }
        } else {
            if self.composing {
                return Err(());
            }
            let common_start = self
                .text
                .chars()
                .zip(state.text.chars())
                .take_while(|(a, b)| a == b)
                .map(|(ch, _)| ch.len_utf8())
                .sum::<usize>();
            let common_end = self.text[common_start..]
                .chars()
                .rev()
                .zip(state.text[common_start..].chars().rev())
                .take_while(|(a, b)| a == b)
                .map(|(ch, _)| ch.len_utf8())
                .sum::<usize>();
            let old_end = self.text.len() - common_end;
            let new_end = state.text.len() - common_end;
            // Only deletion around the current selection is representable here.
            if common_start != new_end
                || common_start > low
                || old_end < high
                || selected.start != common_start
                || selected.end != common_start
            {
                return Err(());
            }
            edits.push(Edit::Delete { before: low - common_start, after: old_end - high });
            if low != high {
                edits.push(Edit::Commit(String::new()));
            }
            self.text.clone_from(&state.text);
            self.selection = (selected.start, selected.end);
        }
        self.native = state;
        Ok(edits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn state(text: &str, cursor: usize, compose: Option<(usize, usize)>) -> NativeState {
        NativeState { text: text.into(), selection: (cursor, cursor), compose }
    }
    #[test]
    fn unicode_replacement_preedit_cancel_and_commit_restore_both_stores() {
        let mut model = TextInputModel::default();
        model.set_context("A中😀Z", 1, 8);
        assert_eq!(model.native().selection, (4, 1));
        assert_eq!(
            model.receive(state("AniZ", 3, Some((1, 3)))).unwrap(),
            [Edit::Preedit("ni".into(), (2, 2))]
        );
        assert!(model.set_context("old", 0, 0).is_none());
        assert_eq!(model.receive(state("AZ", 1, None)).unwrap(), [Edit::Cancel]);
        assert!(model.set_context("A中😀Z", 1, 8).is_some());
        assert_eq!(model.native().text, "A中😀Z");
        model.receive(state("AnihaoZ", 6, Some((1, 6)))).unwrap();
        assert_eq!(model.receive(state("A你好Z", 3, None)).unwrap(), [Edit::Commit("你好".into())]);
        assert!(!model.composing());
    }
    #[test]
    fn direct_commit_delete_surrounding_and_duplicate_notifications() {
        let mut model = TextInputModel::default();
        model.set_context("0", 1, 1);
        let next = state("0你好", 3, None);
        assert_eq!(model.receive(next.clone()).unwrap(), [Edit::Commit("你好".into())]);
        assert!(model.receive(next).unwrap().is_empty());
        assert_eq!(
            model.receive(state("0你", 2, None)).unwrap(),
            [Edit::Delete { before: 3, after: 0 }]
        );
        model.set_context("A😀Z", 1, 1);
        assert_eq!(
            model.receive(state("AZ", 1, None)).unwrap(),
            [Edit::Delete { before: 0, after: 4 }]
        );
    }
    #[test]
    fn rejects_bad_surrogates_unowned_edits_and_oversized_context_without_mutation() {
        let mut model = TextInputModel::default();
        model.set_context("A😀Z", 5, 5);
        let original = model.native();
        for s in [
            state("A😀Z", 2, None),
            state("X😀Z", 3, None),
            state("A😀Z", 0, None),
            state(&"a".repeat(16385), 0, None),
        ] {
            assert!(model.receive(s).is_err());
            assert_eq!(model.native(), original);
        }
        assert!(model.set_context(&"a".repeat(4000), 0, 0).is_none());
        assert!(model.set_context("中", 1, 0).is_none());
    }
}
