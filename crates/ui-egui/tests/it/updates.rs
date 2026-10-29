//! Help ▸ Check for updates. Parchfold has no release feed, so the command explains that and never
//! asks the updater (which is still bound to the upstream PdfCraft suite).

use std::sync::Arc;

use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use pdfcraft_ui_egui::PdfCraftApp;
use pdfcraft_ui_egui::updates::UpdateSource;

fn release(tag: &str) -> craft_update::Release {
    craft_update::Release {
        tag: tag.to_string(),
        version: craft_update::parse_version(tag),
        html_url: format!("https://github.com/tomblanchard312/Parchfold/releases/tag/{tag}"),
        ..Default::default()
    }
}

/// `Ok(Some(tag))` is a newer release, `Ok(None)` is up to date.
fn source(answer: Result<Option<&str>, &str>) -> UpdateSource {
    let answer = answer.map(|tag| tag.map(release)).map_err(str::to_string);
    Arc::new(move || answer.clone())
}

fn harness(answer: Result<Option<&str>, &str>) -> Harness<'static, PdfCraftApp> {
    Harness::builder().with_size(egui::vec2(1200.0, 800.0)).build_eframe(move |_cc| {
        let mut app = PdfCraftApp::new();
        app.set_option("language", "en").unwrap();
        app.set_update_source(source(answer));
        app
    })
}

#[test]
fn check_for_updates_explains_that_installation_is_off() {
    let mut h = harness(Ok(Some("v99.0.0")));
    h.state_mut().community_card_closed = true;
    h.run_steps(2);
    h.state_mut().execute("help.check_updates");
    h.run_steps(4);
    h.get_by_label_contains("Parchfold does not install updates yet.");
    h.get_by_label_contains("will not download or install a PdfCraft release");
    assert!(h.query_by_label("DOWNLOAD").is_none());
    assert!(h.query_by_label_contains("is available").is_none());
    h.get_by_label("Close").click();
    h.run_steps(3);
    assert!(h.query_by_label_contains("does not install updates").is_none());
}

#[test]
fn a_release_source_is_never_asked() {
    let calls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let counted = calls.clone();
    let mut h = Harness::builder().with_size(egui::vec2(1200.0, 800.0)).build_eframe(move |_cc| {
        let mut app = PdfCraftApp::new();
        app.set_option("language", "en").unwrap();
        app.restore(r#"{"check_updates_at_start": true}"#);
        app.set_startup_update_check(true);
        let counted = counted.clone();
        app.set_update_source(Arc::new(move || {
            counted.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            Ok(Some(release("v99.0.0")))
        }));
        app
    });
    h.run_steps(6);
    h.state_mut().execute("help.check_updates");
    h.run_steps(6);
    assert_eq!(calls.load(std::sync::atomic::Ordering::SeqCst), 0);
    h.get_by_label_contains("Parchfold does not install updates yet.");
}
