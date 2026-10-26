//! A non-displayed UIKit text store supplies UIKit's opaque position/range and
//! tokenizer implementations. It is owned by one WinitView on the main thread;
//! it is never installed in the view tree or made first responder.
use crate::text_input_model::{byte_to_utf16, selected_bytes};
use objc2::{
    MainThreadMarker,
    rc::{Retained, Weak},
    runtime::ProtocolObject,
};
use objc2_core_foundation::{CGPoint, CGRect, CGSize};
use objc2_foundation::{NSRange, NSString};
use objc2_ui_kit::{
    UIKeyboardType, UITextAutocapitalizationType, UITextAutocorrectionType, UITextInput,
    UITextInputDelegate, UITextInputTraits, UITextSpellCheckingType, UITextView,
};
use std::cell::{Cell, RefCell};
use winit_core::{
    event::Ime,
    window::{ImePurpose, ImeRequestData},
};

pub(crate) struct TextInput {
    pub(crate) backing: Retained<UITextView>,
    pub(crate) enabled: Cell<bool>,
    pub(crate) delegate: RefCell<Option<Weak<ProtocolObject<dyn UITextInputDelegate>>>>,
    pub(crate) caret: Cell<CGRect>,
    pending_purpose: Cell<Option<ImePurpose>>,
}
impl TextInput {
    pub(crate) fn new(mtm: MainThreadMarker) -> Self {
        let backing = UITextView::new(mtm);
        backing.setAutocorrectionType(UITextAutocorrectionType::No);
        backing.setSpellCheckingType(UITextSpellCheckingType::No);
        backing.setAutocapitalizationType(UITextAutocapitalizationType::None);
        Self {
            backing,
            enabled: Cell::new(false),
            delegate: RefCell::new(None),
            pending_purpose: Cell::new(None),
            caret: Cell::new(CGRect::new(CGPoint::new(0., 0.), CGSize::new(1., 1.))),
        }
    }
    pub(crate) fn selection(&self) -> NSRange {
        let begin = self.backing.beginningOfDocument();
        let Some(range) = self.backing.selectedTextRange() else {
            return NSRange::new(0, 0);
        };
        let start = self.backing.offsetFromPosition_toPosition(&begin, &range.start());
        let end = self.backing.offsetFromPosition_toPosition(&begin, &range.end());
        match (usize::try_from(start), usize::try_from(end)) {
            (Ok(start), Ok(end)) if start <= end => NSRange::new(start, end - start),
            _ => NSRange::new(0, 0),
        }
    }
    fn set_selection(&self, range: NSRange) {
        let begin = self.backing.beginningOfDocument();
        let (Ok(start), Some(end)) = (
            isize::try_from(range.location),
            range.location.checked_add(range.length).and_then(|v| isize::try_from(v).ok()),
        ) else {
            return;
        };
        let (Some(start), Some(end)) = (
            self.backing.positionFromPosition_offset(&begin, start),
            self.backing.positionFromPosition_offset(&begin, end),
        ) else {
            return;
        };
        let selection = self.backing.textRangeFromPosition_toPosition(&start, &end);
        self.backing.setSelectedTextRange(selection.as_deref());
    }
    pub(crate) fn update(&self, data: ImeRequestData, scale: f64) -> bool {
        if let Some((_, purpose)) = data.hint_and_purpose {
            self.pending_purpose.set(Some(purpose));
        }
        let changed = self.apply_pending_purpose();
        if let Some((position, size)) = data.cursor_area {
            let p = position.to_logical::<f64>(scale);
            let s = size.to_logical::<f64>(scale);
            if [p.x, p.y, s.width, s.height].iter().all(|v| v.is_finite())
                && s.width >= 0.
                && s.height >= 0.
            {
                self.caret.set(CGRect::new(
                    CGPoint::new(p.x, p.y),
                    CGSize::new(s.width.max(1.), s.height.max(1.)),
                ));
            }
        }
        if let Some(context) = data.surrounding_text {
            // A renderer can acknowledge the old committed value while native
            // marked text is active. Never overwrite that live composition.
            if self.backing.markedTextRange().is_some() {
                return changed;
            }
            let text = context.text();
            let start = byte_to_utf16(text, context.cursor().min(context.anchor())).unwrap();
            let end = byte_to_utf16(text, context.cursor().max(context.anchor())).unwrap();
            let selected = NSRange::new(start, end - start);
            if self.backing.text().to_string() != text {
                self.backing.setText(Some(&NSString::from_str(text)));
            }
            if self.selection() != selected {
                self.set_selection(selected);
            }
        }
        changed
    }
    // Changing native traits must not destroy live marked text. Defer them
    // until the first terminal update, then refresh the actual input views.
    pub(crate) fn apply_pending_purpose(&self) -> bool {
        if self.backing.markedTextRange().is_some() {
            return false;
        }
        let Some(purpose) = self.pending_purpose.take() else {
            return false;
        };
        let secure = matches!(purpose, ImePurpose::Password | ImePurpose::Pin);
        let keyboard = match purpose {
            ImePurpose::Number => UIKeyboardType::NumbersAndPunctuation,
            ImePurpose::Phone => UIKeyboardType::PhonePad,
            ImePurpose::Url => UIKeyboardType::URL,
            ImePurpose::Email => UIKeyboardType::EmailAddress,
            ImePurpose::Pin => UIKeyboardType::NumberPad,
            _ => UIKeyboardType::Default,
        };
        let changed =
            self.backing.isSecureTextEntry() != secure || self.backing.keyboardType() != keyboard;
        if self.backing.isSecureTextEntry() != secure {
            self.backing.setSecureTextEntry(secure);
        }
        if self.backing.keyboardType() != keyboard {
            self.backing.setKeyboardType(keyboard);
        }
        changed
    }
    pub(crate) fn preedit(&self) -> Ime {
        let Some(marked) = self.backing.markedTextRange() else {
            return Ime::Preedit(String::new(), Some((0, 0)));
        };
        let text =
            self.backing.textInRange(&marked).map(|text| text.to_string()).unwrap_or_default();
        let start = self
            .backing
            .offsetFromPosition_toPosition(&self.backing.beginningOfDocument(), &marked.start());
        let selection = self.selection();
        let relative =
            usize::try_from(start).ok().and_then(|start| selection.location.checked_sub(start));
        let cursor = relative.and_then(|start| selected_bytes(&text, start, selection.length));
        Ime::Preedit(text, cursor)
    }
    pub(crate) fn marked(&self) -> Option<String> {
        self.backing
            .markedTextRange()
            .and_then(|range| self.backing.textInRange(&range))
            .map(|text| text.to_string())
    }
    pub(crate) fn clear(&self) {
        self.backing.unmarkText();
        self.backing.setText(Some(&NSString::new()));
        self.apply_pending_purpose();
    }
}
