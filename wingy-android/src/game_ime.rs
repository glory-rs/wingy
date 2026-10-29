use crate::text_input_model::{Edit, MAX_NATIVE_BYTES, NativeState, TextInputModel};
use android_activity::{
    AndroidApp,
    input::{ImeOptions, InputType, TextInputAction, TextInputState, TextSpan},
};
use wingy_core::{
    event::Ime,
    window::{ImeCapabilities, ImePurpose, ImeRequest, ImeRequestData, ImeRequestError},
};

#[derive(Debug, Default)]
pub(crate) struct GameIme {
    pub capabilities: Option<ImeCapabilities>,
    model: TextInputModel,
    pending: Vec<Ime>,
    purpose: Option<ImePurpose>,
}

#[derive(Default)]
pub(crate) struct Effects {
    state: Option<NativeState>,
    purpose: Option<ImePurpose>,
    visibility: Option<bool>,
}

impl Effects {
    // All Android/JVM work happens after releasing the session mutex.
    pub fn apply(self, app: &AndroidApp) {
        if let Some(purpose) = self.purpose {
            let kind = match purpose {
                ImePurpose::Password => {
                    InputType::TYPE_CLASS_TEXT | InputType::TYPE_TEXT_VARIATION_PASSWORD
                },
                ImePurpose::Pin => {
                    InputType::TYPE_CLASS_NUMBER | InputType::TYPE_NUMBER_VARIATION_PASSWORD
                },
                ImePurpose::Number => {
                    InputType::TYPE_CLASS_NUMBER
                        | InputType::TYPE_NUMBER_FLAG_DECIMAL
                        | InputType::TYPE_NUMBER_FLAG_SIGNED
                },
                ImePurpose::Phone => InputType::TYPE_CLASS_PHONE,
                ImePurpose::Email => {
                    InputType::TYPE_CLASS_TEXT | InputType::TYPE_TEXT_VARIATION_EMAIL_ADDRESS
                },
                ImePurpose::Url => InputType::TYPE_CLASS_TEXT | InputType::TYPE_TEXT_VARIATION_URI,
                _ => InputType::TYPE_CLASS_TEXT,
            };
            app.set_ime_editor_info(
                kind,
                TextInputAction::Done,
                ImeOptions::IME_FLAG_NO_FULLSCREEN | ImeOptions::IME_FLAG_NO_PERSONALIZED_LEARNING,
            );
        }
        if let Some(state) = self.state {
            app.set_text_input_state(TextInputState {
                text: state.text,
                selection: TextSpan { start: state.selection.0, end: state.selection.1 },
                compose_region: state.compose.map(|(start, end)| TextSpan { start, end }),
            });
        }
        if let Some(enabled) = self.visibility {
            if enabled {
                app.show_soft_input(true);
            } else {
                app.hide_soft_input(true);
            }
        }
    }
}

impl GameIme {
    pub fn request(&mut self, request: ImeRequest) -> Result<Effects, ImeRequestError> {
        let mut effect = Effects::default();
        let data = match request {
            ImeRequest::Enable(enable) => {
                if self.capabilities.is_some() {
                    return Err(ImeRequestError::AlreadyEnabled);
                }
                let (capabilities, data) = enable.into_raw();
                self.capabilities = Some(capabilities);
                self.model = TextInputModel::default();
                self.purpose = Some(ImePurpose::Normal);
                self.pending.push(Ime::Enabled);
                effect.state = Some(self.model.native());
                effect.visibility = Some(true);
                data
            },
            ImeRequest::Update(data) => {
                if self.capabilities.is_none() {
                    return Err(ImeRequestError::NotEnabled);
                }
                data
            },
            ImeRequest::Disable => {
                if self.capabilities.take().is_some() {
                    self.pending.push(Ime::Disabled);
                }
                self.model = TextInputModel::default();
                self.purpose = None;
                return Ok(Effects {
                    state: Some(NativeState::default()),
                    visibility: Some(false),
                    ..Default::default()
                });
            },
            _ => return Err(ImeRequestError::NotSupported),
        };
        self.update(data, &mut effect);
        Ok(effect)
    }
    fn update(&mut self, data: ImeRequestData, effect: &mut Effects) {
        let capabilities = self.capabilities.unwrap();
        if capabilities.hint_and_purpose()
            && let Some((_, purpose)) = data.hint_and_purpose
        {
            self.purpose = Some(purpose);
        }
        if !self.model.composing() {
            effect.purpose = self.purpose.take();
        }
        if capabilities.surrounding_text()
            && let Some(context) = data.surrounding_text
        {
            let next = self.model.set_context(context.text(), context.cursor(), context.anchor());
            if next.is_some() {
                effect.state = next;
            }
        }
    }
    pub fn receive(&mut self, state: &TextInputState) -> Effects {
        if self.capabilities.is_none() {
            return Effects::default();
        }
        // Check the borrowed Android buffer before allocating our bounded copy.
        if state.text.len() > MAX_NATIVE_BYTES {
            return Effects { state: Some(self.model.native()), ..Default::default() };
        }
        let native = NativeState {
            text: state.text.clone(),
            selection: (state.selection.start, state.selection.end),
            compose: state.compose_region.map(|span| (span.start, span.end)),
        };
        let mut effect = Effects::default();
        match self.model.receive(native) {
            Ok(edits) => self.pending.extend(edits.into_iter().map(|edit| match edit {
                Edit::Preedit(text, cursor) => Ime::Preedit(text, Some(cursor)),
                Edit::Commit(text) => Ime::Commit(text),
                Edit::Cancel => Ime::Preedit(String::new(), None),
                Edit::Delete { before, after } => {
                    Ime::DeleteSurrounding { before_bytes: before, after_bytes: after }
                },
            })),
            Err(()) => effect.state = Some(self.model.native()),
        }
        if !self.model.composing() {
            effect.purpose = self.purpose.take();
        }
        effect
    }
    pub fn drain(&mut self) -> Vec<Ime> {
        std::mem::take(&mut self.pending)
    }
}
