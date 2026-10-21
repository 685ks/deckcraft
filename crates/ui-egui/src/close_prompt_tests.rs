//! Closing the window or quitting (the close button, Alt+F4, ⌘Q, the OS asking the window to close)
//! asks about unsaved changes first, through egui's viewport close request (#60's quit path).
#![allow(clippy::unwrap_used, clippy::expect_used)]

use deckcraft_engine::Session;
use egui::{Event, Key, Modifiers, ViewportCommand, ViewportEvent, ViewportId};
use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use serde_json::json;

use crate::{Services, SlideApp};

fn harness(app: SlideApp) -> Harness<'static, SlideApp> {
    let mut h = Harness::builder().with_size(egui::vec2(1400.0, 900.0)).build_ui_state(
        |ui, app: &mut SlideApp| {
            app.logic(ui.ctx());
            app.ui(ui);
        },
        app,
    );
    h.run_steps(3);
    h
}

fn edited() -> Harness<'static, SlideApp> {
    let mut h = harness(SlideApp::new(Session::with_new(), Services::default()));
    h.state_mut().run("slide.new", json!({})).unwrap();
    h.run_steps(2);
    assert!(h.state().session.documents().iter().any(|d| d.is_dirty()));
    h
}

/// The viewport commands the app sent in the last frame.
fn commands(h: &Harness<'_, SlideApp>) -> Vec<ViewportCommand> {
    h.output().viewport_output.get(&ViewportId::ROOT).map(|v| v.commands.clone()).unwrap_or_default()
}

/// The window manager asks the window to close (close button, Alt+F4, the Dock's Quit…).
fn request_close(h: &mut Harness<'_, SlideApp>) -> Vec<ViewportCommand> {
    h.input_mut().viewports.entry(ViewportId::ROOT).or_default().events.push(ViewportEvent::Close);
    h.step();
    commands(h)
}

/// Presses a key in one frame and returns that frame's viewport commands. (The harness's own
/// `key_press` queues four events and `step` runs a frame for each, so the frame that handled the
/// key, and its commands, would be followed by three more and lost.)
fn press(h: &mut Harness<'_, SlideApp>, modifiers: Modifiers, key: Key) -> Vec<ViewportCommand> {
    let events = &mut h.input_mut().events;
    events.push(Event::ModifiersChanged(modifiers));
    for pressed in [true, false] {
        events.push(Event::Key { key, physical_key: None, pressed, repeat: false, modifiers });
    }
    events.push(Event::ModifiersChanged(Modifiers::NONE));
    h.step();
    commands(h)
}

fn dialog(h: &Harness<'_, SlideApp>) -> Option<String> {
    h.state().dialog.as_ref().map(|d| d.id.clone())
}

#[test]
fn closing_with_unsaved_changes_is_cancelled_and_asks() {
    let mut h = edited();
    let cmds = request_close(&mut h);
    assert!(cmds.contains(&ViewportCommand::CancelClose), "the close must be held back: {cmds:?}");
    assert_eq!(dialog(&h).as_deref(), Some("quit"));
    assert_eq!(h.state().session.documents().len(), 1, "nothing is closed while the question is open");
    // Asking again (a second click on the close button) keeps one question and still holds the close.
    let cmds = request_close(&mut h);
    assert!(cmds.contains(&ViewportCommand::CancelClose), "{cmds:?}");
    assert_eq!(dialog(&h).as_deref(), Some("quit"));
}

#[test]
fn closing_with_nothing_unsaved_closes_at_once() {
    let mut h = harness(SlideApp::new(Session::with_new(), Services::default()));
    let cmds = request_close(&mut h);
    assert!(!cmds.contains(&ViewportCommand::CancelClose), "{cmds:?}");
    assert_eq!(dialog(&h), None);
}

#[test]
fn cancel_keeps_the_window_and_the_changes() {
    let mut h = edited();
    request_close(&mut h);
    h.run_steps(2);
    h.get_by_label("Cancel").click();
    h.step();
    assert!(!commands(&h).contains(&ViewportCommand::Close), "Cancel must not close the window");
    h.run_steps(2);
    assert_eq!(dialog(&h), None);
    assert!(!h.state().quit_confirmed);
    assert!(h.state().session.documents().iter().any(|d| d.is_dirty()), "the edits are still there");
    // The next close asks again.
    assert!(request_close(&mut h).contains(&ViewportCommand::CancelClose));
}

#[test]
fn dont_save_closes_the_window() {
    let mut h = edited();
    request_close(&mut h);
    h.run_steps(2);
    h.get_by_label("Don't Save").click();
    h.step();
    assert!(commands(&h).contains(&ViewportCommand::Close), "Don't Save closes: {:?}", commands(&h));
    assert!(h.state().quit_confirmed);
    // The close that follows is let through.
    assert!(!request_close(&mut h).contains(&ViewportCommand::CancelClose));
}

#[test]
fn save_writes_the_file_then_closes() {
    let dir = std::env::temp_dir().join(format!("deckcraft-close-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let target = dir.clone();
    let services = Services {
        pick_save: Some(Box::new(move |name: &str| Some(target.join(name).to_string_lossy().to_string()))),
        write: Some(Box::new(|p: &str, b: &[u8]| std::fs::write(p, b).map_err(|e| e.to_string()))),
        ..Default::default()
    };
    let mut h = harness(SlideApp::new(Session::with_new(), services));
    h.state_mut().run("slide.new", json!({})).unwrap();
    h.run_steps(2);
    request_close(&mut h);
    h.run_steps(2);
    let cmds = press(&mut h, Modifiers::NONE, Key::Enter);
    assert!(cmds.contains(&ViewportCommand::Close), "Save then close: {cmds:?}");
    assert!(h.state().quit_confirmed);
    let saved: Vec<_> = std::fs::read_dir(&dir).unwrap().flatten().collect();
    assert_eq!(saved.len(), 1, "one presentation saved: {saved:?}");
    assert!(!h.state().session.documents().iter().any(|d| d.is_dirty()));
    let _ = std::fs::remove_dir_all(&dir);
}

/// ⌘Q goes through the same close request (and so the same question) instead of ending the
/// process; Ctrl+Q on other platforms is left alone.
#[test]
fn command_q_requests_a_close_that_asks() {
    let mut h = edited();
    let cmds = press(&mut h, Modifiers::MAC_CMD | Modifiers::COMMAND, Key::Q);
    assert!(cmds.contains(&ViewportCommand::Close), "⌘Q asks the window to close: {cmds:?}");
    assert!(!h.state().quit_confirmed, "⌘Q itself confirms nothing");
    // The platform turns that command into a close request, which asks.
    assert!(request_close(&mut h).contains(&ViewportCommand::CancelClose));
    assert_eq!(dialog(&h).as_deref(), Some("quit"));

    let mut h = edited();
    let cmds = press(&mut h, Modifiers::CTRL | Modifiers::COMMAND, Key::Q);
    assert!(!cmds.contains(&ViewportCommand::Close), "Ctrl+Q is not a quit shortcut here: {cmds:?}");
}
