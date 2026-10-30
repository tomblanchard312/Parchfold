use crate::PdfCraftApp;
use enki_speech_ui::{Action, VoiceCommand};
pub(crate) fn show(app: &mut PdfCraftApp, ctx: &egui::Context) {
    let scope = app
        .active_ids()
        .and_then(|(i, id)| {
            app.views.get(i).map(|v| format!("{id:?}:{}:{}", v.current, app.session.get(id).map(|d| d.edit_generation()).unwrap_or(0)))
        })
        .unwrap_or_default();
    let Some(action) = app.speech.show(ctx, &scope, "Add transcript as a comment") else { return };
    match action {
        Action::Insert(text) => {
            let Some((i, _)) = app.active_ids() else {
                app.notify("Open a PDF before inserting the transcript.");
                return;
            };
            let page = app.views.get(i).map(|v| v.current).unwrap_or(0);
            app.apply_edit(pdfcraft_engine::Edit::AddAnnotation(pdfcraft_engine::NewAnnotation {
                page,
                shape: pdfcraft_engine::Shape::Note { at: [72.0, 72.0], icon: pdfcraft_engine::NoteIcon::Note },
                style: Default::default(),
                contents: text,
                author: "".into(),
            }));
        }
        Action::Command(command) => match command {
            VoiceCommand::OpenAssistant => app.speech.open = true,
            VoiceCommand::Stop => app.speech.open = false,
            VoiceCommand::Undo => {
                app.execute("edit.undo");
            }
            VoiceCommand::Redo => {
                app.execute("edit.redo");
            }
            _ => {
                let Some((i, id)) = app.active_ids() else {
                    return;
                };
                let pages = app.session.get(id).map(|d| d.info.pages.len()).unwrap_or(0);
                if let Some(view) = app.views.get_mut(i) {
                    match command {
                        VoiceCommand::Next => view.go_to_page(view.current.saturating_add(1).min(pages.saturating_sub(1))),
                        VoiceCommand::Previous => view.go_to_page(view.current.saturating_sub(1)),
                        VoiceCommand::ZoomIn | VoiceCommand::ZoomOut => {
                            view.fit = crate::canvas::Fit::None;
                            view.zoom = (view.zoom * if command == VoiceCommand::ZoomIn { 1.1 } else { 1.0 / 1.1 }).clamp(0.1, 8.0);
                        }
                        _ => {}
                    }
                }
            }
        },
    }
}
