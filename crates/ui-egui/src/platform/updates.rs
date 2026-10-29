//! Help ▸ Check for updates and the check at start, through `craft_update`: the flow itself is its
//! `Updater`; this module supplies the suite, the quit path and the status dialog.

#[cfg(not(target_arch = "wasm32"))]
use craft_update::updater::{Action, CheckSource, Status, Updater};
use egui::{Align, Layout};

use crate::{PdfCraftApp, theme, widgets};

/// Parchfold has no release feed yet. This page is informational only; this build does not install from it.
pub const RELEASES_PAGE: &str = "https://github.com/tomblanchard312/Parchfold/releases";

/// Update installation stays off until Parchfold publishes a feed of its own. The upstream PdfCraft
/// feed must not be used: installing from it would replace this app with PdfCraft.
fn updates_installable() -> bool {
    false
}

/// Stands in for the network in tests.
#[cfg(not(target_arch = "wasm32"))]
pub type UpdateSource = CheckSource;

pub(crate) struct Updates {
    #[cfg(not(target_arch = "wasm32"))]
    updater: Updater,
    /// The Updates dialog is showing.
    pub(crate) open: bool,
    /// Ask for a newer release at start; the desktop app turns it on, so tests never reach the network.
    #[cfg(not(target_arch = "wasm32"))]
    startup_check: bool,
    #[cfg(not(target_arch = "wasm32"))]
    check_requested: bool,
}

impl Default for Updates {
    fn default() -> Self {
        Self {
            #[cfg(not(target_arch = "wasm32"))]
            updater: Updater::new(&craft_update::suite::PDFCRAFT, env!("CARGO_PKG_VERSION")),
            open: false,
            #[cfg(not(target_arch = "wasm32"))]
            startup_check: false,
            #[cfg(not(target_arch = "wasm32"))]
            check_requested: false,
        }
    }
}

impl PdfCraftApp {
    /// Whether the app asks for a newer release on its first frame (off until the desktop app turns it on).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_startup_update_check(&mut self, on: bool) {
        self.updates.startup_check = on;
    }

    /// Replaces the network with `source` (tests).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_update_source(&mut self, source: UpdateSource) {
        self.updates.updater = Updater::new(&craft_update::suite::PDFCRAFT, env!("CARGO_PKG_VERSION")).with_source(source);
    }

    /// Help ▸ Check for updates: ask for the latest release and show the outcome.
    pub fn check_for_updates(&mut self) {
        self.updates.open = true;
        // Do not ask the updater. It is bound to the PdfCraft suite entry and can download that app.
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.updates.check_requested = false;
        }
    }

    /// Starts the checks and picks up their answers (each frame).
    pub(crate) fn poll_updates(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if !updates_installable() {
                return;
            }
            let Some(ctx) = self.ctx.clone() else { return };
            let updates = &mut self.updates;
            if std::mem::take(&mut updates.check_requested) {
                updates.updater.check_now(&ctx);
            } else if updates.startup_check {
                updates.updater.check_at_start(&ctx);
            }
            updates.updater.tick(&ctx);
        }
    }
}

/// The startup prompt, and the Updates dialog that answers Help ▸ Check for updates.
pub(crate) fn dialog(app: &mut PdfCraftApp, ctx: &egui::Context) {
    #[cfg(not(target_arch = "wasm32"))]
    if updates_installable() {
        // The restart helper is scheduled before the app closes, so no update starts over unsaved work.
        let clean = app.first_dirty().is_none();
        match app.updates.updater.show_if(ctx, clean) {
            Action::None => {}
            Action::OpenUrl(url) => app.open_url(&url),
            Action::Close => {
                app.request_quit(ctx);
            }
        }
        // A requested check that found a release is answered by the prompt, not by this dialog.
        if app.updates.open && !app.updates.check_requested && matches!(app.updates.updater.status(), Status::Idle) {
            app.updates.open = false;
        }
    }
    if !app.updates.open {
        return;
    }
    let t = theme::Tokens::get(ctx);
    let current = env!("CARGO_PKG_VERSION");
    let mut close = false;
    let modal = crate::widgets::show_modal(ctx, "updates", |ui| {
        ui.set_width(420.0);
        ui.horizontal(|ui| {
            ui.add(crate::icons::image("cloud", 22.0, t.accent));
            ui.label(egui::RichText::new(tl!("Check for updates")).font(theme::semibold(16.0)));
        });
        ui.add_space(8.0);
        if !updates_installable() {
            ui.label(tl!("Parchfold does not install updates yet."));
            ui.label(egui::RichText::new(tl!("This build will not download or install a PdfCraft release.")).color(t.text_muted));
        }
        #[cfg(not(target_arch = "wasm32"))]
        if updates_installable() {
            match app.updates.updater.status() {
                Status::Idle => {
                    ui.horizontal(|ui| {
                        ui.spinner();
                        ui.label(tl!("Checking for a newer version…"));
                    });
                }
                Status::Checking => {
                    ui.horizontal(|ui| {
                        ui.spinner();
                        ui.label(tl!("Checking for a newer version…"));
                    });
                }
                Status::UpToDate => {
                    ui.label(crate::i18n::fmt(tl!("PdfCraft {c} is up to date."), &[("c", current)]));
                }
                Status::Failed(e) => {
                    ui.label(crate::i18n::fmt(tl!("Couldn't check for updates: {e}"), &[("e", e)]));
                    ui.label(
                        egui::RichText::new(crate::i18n::fmt(
                            tl!("You have version {c}. All releases are listed at {page}."),
                            &[("c", current), ("page", RELEASES_PAGE)],
                        ))
                        .color(t.text_muted),
                    );
                }
            }
        }
        #[cfg(target_arch = "wasm32")]
        let _ = (&t, current);
        #[cfg(not(target_arch = "wasm32"))]
        if !updates_installable() {
            let _ = current;
        }
        ui.add_space(10.0);
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if widgets::pill_button(ui, tl!("Close"), true).clicked() {
                close = true;
            }
        });
    });
    if modal.should_close() {
        close = true;
    }
    if close {
        app.updates.open = false;
        #[cfg(not(target_arch = "wasm32"))]
        if updates_installable() {
            app.updates.updater.dismiss_status();
        }
    }
}
